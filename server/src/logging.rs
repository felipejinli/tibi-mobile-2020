//! Middleware & helper for structured logging of auth endpoints.

use actix_service::{Service, Transform};
use actix_web::{
    dev::{Body, MessageBody, ServiceRequest, ServiceResponse},
    Error,
};

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;

use slog::{info, Logger};

/// The struct that handles the logging of requests
#[derive(Clone)]
pub struct Logging {
    logger: Logger,
}

impl Logging {
    pub fn new(logger: Logger) -> Self {
        Logging { logger }
    }
}

pub struct LoggingMiddleware<S> {
    logger: Logger,
    service: S,
}

impl<S> Transform<S> for Logging
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<Body>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<Body>;
    type Error = Error;
    type InitError = ();
    type Transform = LoggingMiddleware<S>;
    type Future = futures_util::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        futures_util::future::ok(LoggingMiddleware {
            logger: self.logger.clone(),
            service,
        })
    }
}

impl<S> Service for LoggingMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<Body>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<Body>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let start = Instant::now();
        let fut = self.service.call(req);

        let logger = self.logger.clone();

        Box::pin(async move {
            use actix_web::dev::BodySize;
            let res = fut.await?;
            let req = res.request();

            let end = Instant::now();
            let elapsed = (end - start).as_nanos() as f64 / 1_000_000.0;

            let size = match res.response().body().size() {
                BodySize::Sized(bytes) => format!("{}b", bytes),
                BodySize::None => format!("?"),
                BodySize::Empty => format!("0"),
                BodySize::Stream => format!("~"),
            };

            info!(logger, "{} {:.6}ms", req.path(), elapsed; "status" => res.status().as_u16(), "host" => req.connection_info().host(), "size" => size);

            Ok(res)
        })
    }
}
