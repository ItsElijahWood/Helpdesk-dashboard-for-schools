mod app;
mod database;
mod freshservice;
mod integrations;
mod misc;
pub mod public;

use dotenvy::dotenv;

use crate::app::app;

fn main() {
    dotenv().expect("failed to load .env file");
    app().expect("failed to start web server")
}
