use std::sync::LazyLock;

use actix_web::{HttpResponse, HttpResponseBuilder, Result};
use awc::{Client, http::StatusCode};

use crate::app::ErrorTypes;

static UNIFI: LazyLock<(String, String)> = LazyLock::new(|| {
    (
        std::env::var("CCTV_URL").expect("failed to fetch cctv_url from .env file."),
        std::env::var("CCTV_TOKEN").expect("failed to fetch cctv_token from .env file."),
    )
});

pub async fn fetch() -> Result<HttpResponse, ErrorTypes> {
    let (cctv_url, cctv_token) = (&UNIFI.0, UNIFI.1.as_str());

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .finish();

    let mut response = client
        .get(cctv_url)
        .insert_header(("X-API-Key", cctv_token))
        .send()
        .await
        .map_err(|e| {
            eprintln!("request error {e}");
            ErrorTypes::InternalServerError
        })?;

    let response_body = response.body().await.map_err(|e| {
        eprintln!("body error {e}");
        ErrorTypes::InternalServerError
    })?;

    let mut v: Vec<serde_json::Value> = Vec::new();
    let slice: serde_json::Value = serde_json::from_slice(&response_body).unwrap();

    v.push(slice);

    let body = serde_json::to_vec(&v).unwrap();

    Ok(HttpResponseBuilder::new(StatusCode::OK)
        .content_type("application/json")
        .body(body))
}
