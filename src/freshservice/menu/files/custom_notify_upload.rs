use std::path::Path;

use actix_multipart::Multipart;
use actix_web::{HttpResponse, HttpResponseBuilder};
use awc::http::StatusCode;
use futures_util::StreamExt;
use serde::Serialize;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::{app::ErrorTypes, freshservice::menu::database::custom_notify_save};

pub async fn file(mut payload: Multipart) -> Result<HttpResponse, ErrorTypes> {
    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();

        let cd = field.content_disposition().cloned();
        let field_name = cd.as_ref().and_then(|c| c.get_name()).unwrap();
        let filename = cd.as_ref().and_then(|c| c.get_filename()).unwrap();

        let custom_dir = Path::new("assets/media/fs/custom");
        let custom_file = custom_dir.join(filename);

        if custom_file.exists() {
            let err = "File already exists.";

            let body = serde_json::to_string(err).unwrap();

            return Ok(HttpResponseBuilder::new(StatusCode::CONFLICT).body(body));
        }

        if field_name != "file" {
            while let Some(chunk) = field.next().await {
                let _ = chunk.unwrap();
            }
            continue;
        }

        let mut f = File::create(format!("assets/media/fs/custom/{}", filename))
            .await
            .unwrap();

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await.unwrap();
        }

        #[derive(Serialize)]
        struct Resp<'a> {
            filename: &'a str,
        }

        let resp = Resp {
            filename: &filename,
        };
        let body = serde_json::to_string(&resp).unwrap();

        custom_notify_save::db(&filename.to_string()).await;

        return Ok(HttpResponseBuilder::new(StatusCode::CREATED).body(body));
    }

    Ok(HttpResponse::InternalServerError().finish())
}
