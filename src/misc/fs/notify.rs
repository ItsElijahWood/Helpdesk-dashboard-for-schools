use std::{fs::File, io::BufReader};

use actix_web::{HttpResponse, HttpResponseBuilder, Result};
use awc::http::StatusCode;
use rodio::{Decoder, DeviceSinkBuilder, Player};
use sqlx::Row;

use crate::{app::ErrorTypes, database::connection};

pub async fn play() -> Result<HttpResponse, ErrorTypes> {
    let mut conn = connection()
        .await
        .expect("failed to initialise connection to database notify.rs");

    let notification = sqlx::query("SELECT * FROM freshservice WHERE is_selected = ?")
        .bind(1)
        .fetch_one(&mut conn)
        .await
        .unwrap();

    let mut sink_handle = DeviceSinkBuilder::open_default_sink()
        .expect("failed to open os sink for default audio output stream.");
    sink_handle.log_on_drop(false);

    let player = Player::connect_new(&sink_handle.mixer());

    let file_is_default: u8 = notification.try_get("is_default").unwrap();
    let freshservice_file_name: String = notification.try_get("file_name").unwrap();
    let freshservice_file_ext: String = notification.try_get("file_ext").unwrap();

    let mut file_name = String::new();
    if file_is_default == 1 {
        file_name =
            format!("assets/media/fs/default/{freshservice_file_name}.{freshservice_file_ext}");
    } else if file_is_default == 0 {
        file_name =
            format!("assets/media/fs/custom/{freshservice_file_name}.{freshservice_file_ext}");
    }

    let file =
        BufReader::new(File::open(file_name).expect("failed to load sound file into buffer."));
    let source = Decoder::new(file).expect("failed to decode audio.");

    player.append(source);
    player.sleep_until_end();

    Ok(HttpResponseBuilder::new(StatusCode::OK).finish())
}
