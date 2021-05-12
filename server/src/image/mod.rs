//! Handles the `/image/*` endpoints

use crate::{actions, Authorized, Config, DbPool};
use actix_multipart as mp;
use actix_web::{error, http, web, HttpRequest, HttpResponse};
use futures_util::stream::StreamExt;
use serde_json::json;

mod request;
mod response;

/// Adds the endpoints under `/image/*` to the supplied App object and
/// returns the modified App.
pub fn add_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.route("/image/new", web::post().to(new));
    cfg.route("/image/public/{id}", web::get().to(get_public));
}

/// Handles `/image/new`
async fn new(
    authorized: Authorized,
    mut payload: mp::Multipart,
    config: web::Data<Config>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, error::Error> {
    if !authorized.user.is_admin {
        return Ok(HttpResponse::Forbidden().json(json!({
            "error": "You do not have sufficient privileges to upload this image",
            "ERROR_CODE": "LACKS_PERMISSIONS"
        })));
    }

    let mut rng = rand::thread_rng();
    let mut new_images = Vec::new();

    while let Some(item) = payload.next().await {
        use rand::Rng;
        let mut field = item?;
        let mime = field.content_type();
        if mime.type_() != "image" {
            return Ok(HttpResponse::BadRequest().into());
        }

        let extension = match mime.subtype().as_str() {
            "jpg" | "jpeg" => "jpg",
            "png" => "png",
            _ => return Ok(HttpResponse::BadRequest().into()),
        };

        let id_int: u64 = rng.gen();
        let id = format!("{:X}.{}", id_int, extension);

        println!(
            "{} Content-Disposition {:?}",
            id,
            field.content_disposition()
        );
        println!("{} mime={:?}", id, mime);

        let disposition = field
            .content_disposition()
            .ok_or_else(|| HttpResponse::BadRequest())?;

        let mut form_name = None;
        let mut original_name = None;
        for param in disposition.parameters {
            use http::header::DispositionParam;
            match param {
                DispositionParam::Name(name) => form_name = Some(name),
                DispositionParam::Filename(file_name) => original_name = Some(file_name),
                _ => {}
            }
        }

        let (form_name, original_name) = form_name
            .zip(original_name)
            .ok_or_else(HttpResponse::BadRequest)?;

        let mut file = std::fs::File::create(config.image_dir.join(&id))?;

        let mut total = 0;
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            use std::io::Write;
            // TODO: check to see if id already exists
            let n = file.write(&*chunk?)?;
            total += n;

            if total > 5 * 1024 * 1024 {
                println!("Stopped processing file {}: too big", id);
                drop(file);
                std::fs::remove_file(config.image_dir.join(&id))?;

                return Err(HttpResponse::BadRequest().into());
            }
        }
        println!("Done with {}", id);
        let new_image = crate::model::NewImage {
            id,
            created_by: authorized.user.id.clone(),
            private: false,
            original_size_bytes: total as i32,
            optimised_size_bytes: total as i32,
        };

        new_images.push((form_name, original_name, new_image));
    }

    match web::block(move || {
        let conn = pool.get().expect("Couldn't get db connection");
        let mut new_images_reponse = response::NewImages::default();
        for (form_name, original_name, new_image) in new_images {
            let id = new_image.id.clone();
            new_images_reponse.images.insert(
                form_name,
                response::NewImage {
                    original_name,
                    id: id.clone(),
                },
            );
            if let Ok(n) = actions::register_image(&conn, new_image) {
                if n != 1 {
                    println!("Didn't insert image {} into db", id);
                    return Err(());
                }
            } else {
                println!("Register image faile");
                return Err(());
            }
        }

        Ok(new_images_reponse)
    })
    .await
    {
        Ok(images) => Ok(HttpResponse::Ok().json(images)),
        Err(_) => Err(HttpResponse::InternalServerError().into()),
    }
}

/// Handles `/image/public/{ID}`
async fn get_public(
    req: HttpRequest,
    path: web::Path<String>,
    config: web::Data<Config>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, error::Error> {
    use actix_files::NamedFile;
    let id = path.into_inner();

    let meta = match web::block(move || {
        let conn = pool.get().expect("Couldn't connect to db");
        actions::get_image_by_id(&conn, &id)
    })
    .await
    {
        Ok(meta) => meta,
        Err(_) => return Err(HttpResponse::NotFound().into()),
    };

    if meta.private {
        return Err(HttpResponse::NotFound().into());
    }

    // TODO: Last modified
    NamedFile::open(config.image_dir.join(meta.id))?.into_response(&req)
}
