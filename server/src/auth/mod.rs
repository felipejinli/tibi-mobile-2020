//! This file contains the code for the `/auth/*` endpoints.
use actix_web::{client, http, web, HttpRequest, HttpResponse, Responder};

mod request;
mod response;

pub mod jwt;

pub mod middleware;
use middleware::Authorized;

mod sso_state;
use sso_state::SSOStatus;

use crate::{try_or_log_error, Config, DbPool, RedisPool};

use slog::{error, Logger};

const AUTH_REDIS_PREFIX: &'static str = "AUTH_SSO_STATE_";

/// Adds the endpoints under `/auth/*` to the supplied App object and
/// returns the modified App.
pub fn add_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.route("/auth/sso", web::post().to(sso))
        .route("/auth/sso_check", web::post().to(sso_check))
        .route("/auth/sso_callback", web::get().to(sso_callback))
        .route("/auth/refresh", web::post().to(refresh));
}

/// Handles `/auth/sso`
async fn sso(
    req: HttpRequest,
    config: web::Data<Config>,
    redis_pool: web::Data<RedisPool>,
    root_logger: web::Data<Logger>,
) -> Result<HttpResponse, HttpResponse> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let logger = root_logger.new(
        slog::o!("endpoint" =>  "/auth/sso", "host" => req.connection_info().host().to_owned()),
    );
    let mut redis_conn = try_or_log_error!(logger, redis_pool.get().await);

    // Add config.sso_timeout to now and the find the number of seconds of that time from the
    // UNIX_EPOCH.
    let timeout = (SystemTime::now() + config.sso_timeout)
        .duration_since(UNIX_EPOCH)
        .expect("Current time cannot be before the UNIX_EPOCH")
        .as_secs();

    let check_code = sso_state::new_state(&logger, &mut redis_conn, timeout).await?;

    let redirect_url = format!(
        "{}?client_id={}&state={}",
        &config.sso_redirect_url, &config.oauth_client_id, &check_code
    );

    let res = response::SSOResponse {
        redirect_url,
        check_code,
        timeout,
    };

    Ok(HttpResponse::Ok().json(res))
}

/// Handles `/auth/sso_check`
async fn sso_check(
    req: HttpRequest,
    req_body: web::Json<request::SSOCheck>,
    redis_pool: web::Data<RedisPool>,
    root_logger: web::Data<Logger>,
) -> Result<HttpResponse, HttpResponse> {
    use serde_json::json;
    let logger = root_logger
        .new(slog::o!("endpoint" =>  "/auth/sso_check", "host" => req.connection_info().host().to_owned()));

    let mut redis_conn = try_or_log_error!(logger, redis_pool.get().await);

    let check_code = req_body.into_inner().check_code;
    Ok(
        if let Some(status) = sso_state::get_status(&logger, &mut redis_conn, check_code).await? {
            use SSOStatus::*;

            let response = match status {
                WAITING => json!({ "status": "WAITING" }),
                PROCESSING => json!({ "status": "PROCESSING" }),
                ERROR {
                    human_readable_message,
                } => json!({ "status": "ERROR", "error": human_readable_message }),
                AUTHENTICATED { auth_token } => {
                    json!({ "status": "AUTHENTICATED", "auth_token": auth_token })
                }
            };

            HttpResponse::Ok()
                .set_header(http::header::CONTENT_TYPE, "application/json")
                .body(response)
        } else {
            HttpResponse::NotFound().body("NOT FOUND")
        },
    )
}

/// Handles `/auth/sso_callback`
async fn sso_callback(
    req: HttpRequest,
    params: web::Query<request::SSOCallback>,
    config: web::Data<Config>,
    pg_pool: web::Data<DbPool>,
    redis_pool: web::Data<RedisPool>,
    root_logger: web::Data<Logger>,
) -> Result<HttpResponse, HttpResponse> {
    let logger = root_logger
        .new(slog::o!("endpoint" =>  "/auth/sso_callback", "host" => req.connection_info().host().to_owned()));
    let params = params.into_inner();

    let state_token = params.state.clone();

    let mut redis_conn = try_or_log_error!(logger, redis_pool.get().await);

    let status = sso_state::get_status(&logger, &mut redis_conn, params.state.clone())
        .await?
        .ok_or_else(|| {
            HttpResponse::NotFound().body(
                "We don't have any record of this authentication request. \
                    This could happen if the authentication request already succeeded a while ago \
                    or if you didn't complete it in time. \
                    Please go back to your app to check.",
            )
        })?;

    // The only way to transition from WAITING to any other status is when this callback is called.
    // If the status is not waiting this callback has already been called for a particular state.
    // We should print an error and avoid changing the status as there may be an ongoing request
    // curently.
    if status != SSOStatus::WAITING {
        return Ok(HttpResponse::BadRequest().body(
            "We have already begun to process your request. \
            Did you try to double authenticate? \
            Go back to the app to see if the authentication worked.",
        ));
    }

    sso_state::update_status(
        &logger,
        &mut redis_conn,
        params.state.clone(),
        SSOStatus::PROCESSING,
    )
    .await?;

    // util function to both return an HTTP error and update the error on the sso_state so that the
    // client app can show it to the user and try again.
    // In future a nicer HTML page should be rendered as this page is for users and not api
    // requests.
    async fn handle_error(
        logger: Logger,
        state_token: String,
        redis_conn: &mut crate::RedisConnection,
        human_readable_message: &'static str,
    ) -> Result<HttpResponse, HttpResponse> {
        sso_state::update_status(
            &logger,
            redis_conn,
            state_token,
            SSOStatus::ERROR {
                human_readable_message: human_readable_message.to_string(),
            },
        )
        .await?;

        Ok(HttpResponse::InternalServerError().body(human_readable_message.to_string()))
    }

    let client = client::Client::new();

    let token_request = client.get(format!(
        "https://uclapi.com/oauth/token?client_id={}&client_secret={}&code={}",
        config.oauth_client_id, config.oauth_client_secret, params.code
    ));

    let mut token_res = match token_request.send().await {
        Ok(res) => res,
        Err(err) => {
            error!(logger, "(token) Error: {:?}", err);
            return handle_error(
                logger,
                state_token,
                &mut redis_conn,
                "There was an error when contacting the UCL oauth system",
            )
            .await;
        }
    };

    if token_res.status() != http::StatusCode::OK {
        error!(
            logger,
            "Contacting UCL api gave non-zero status: {:?}", token_res
        );
        return handle_error(
            logger,
            state_token,
            &mut redis_conn,
            "There was an error when contacting the UCL oauth system (Bad status code)",
        )
        .await;
    }

    let token_res: request::TokenApi = match token_res.json().await {
        Ok(res) => res,
        Err(err) => {
            error!(
                logger,
                "Contacting UCL api gave non-json response: {:?}", err
            );
            return handle_error(
                logger,
                state_token,
                &mut redis_conn,
                "There was an error when contacting the UCL oauth system (Bad response format)",
            )
            .await;
        }
    };

    if token_res.state != params.state {
        println!("States did not match");
        return handle_error(
            logger,
            state_token,
            &mut redis_conn,
            "There were some inconsistency errors when contacting the UCL oauth system.",
        )
        .await;
    }

    let user_request = client.get(format!(
        "https://uclapi.com/oauth/user/data?token={}&client_secret={}",
        token_res.token, config.oauth_client_secret
    ));

    let mut user_res = match user_request.send().await {
        Ok(res) => res,
        Err(err) => {
            error!(logger, "(user) Error: {:?}", err);
            return handle_error(
                logger,
                state_token,
                &mut redis_conn,
                "There was an error when fetching the user data from UCL",
            )
            .await;
        }
    };

    if user_res.status() != http::StatusCode::OK {
        error!(
            logger,
            "Fetching the user data from UCL retured a non-zero status: {:?}", user_res
        );
        return handle_error(
            logger,
            state_token,
            &mut redis_conn,
            "There was an error when fetching the user data from UCL (Bad status code)",
        )
        .await;
    }

    let user_res: request::UserData = match user_res.json().await {
        Ok(res) => res,
        Err(err) => {
            error!(
                logger,
                "Contacting UCL api gave non-json response: {:?}", err
            );
            return handle_error(
                logger,
                state_token,
                &mut redis_conn,
                "There was an error when fetching the user data from UCL (Bad response format)",
            )
            .await;
        }
    };

    let user_id = user_res.upi.clone();

    // Insert the user into the database (or update if already exists)
    if let Err(err) = web::block(move || {
        use crate::model::NewUser;
        let user = NewUser {
            id: user_res.upi,
            full_name: user_res.full_name,
            given_name: user_res.given_name,
            username: user_res.cn,
            email: user_res.email,
            is_student: user_res.is_student,
            department: user_res.department,
        };

        crate::actions::upsert_user(
            &pg_pool.get().expect("Couldn't connect to the database"),
            user,
        )
    })
    .await
    {
        println!("upsert error {:?}", err);
        return handle_error(
            logger,
            state_token,
            &mut redis_conn,
            "There was an error when accessing the database, please try again later.",
        )
        .await;
    }

    let auth_token = jwt::create_token(&config.jwt_secret, user_id, config.jwt_lifetime.clone());

    // Update status to a successful state
    sso_state::update_status(
        &logger,
        &mut redis_conn,
        state_token,
        SSOStatus::AUTHENTICATED { auth_token },
    )
    .await?;

    Ok(HttpResponse::Ok()
        .body("You have been successfully authenticated, you can safely close this webpage and return to the app."))
}

/// Handles `/auth/refresh`
async fn refresh(authorized: Authorized, config: web::Data<Config>) -> impl Responder {
    let new_auth_token =
        jwt::create_token(&config.jwt_secret, authorized.user.id, config.jwt_lifetime);

    HttpResponse::Ok().json(response::RefreshResponse {
        auth_token: new_auth_token,
    })
}
