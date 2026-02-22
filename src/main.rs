mod constant;
mod controllers;
mod models;
mod services;

use crate::services::service_web::SrvWeb;

#[tokio::main]
async fn main() {
    let x = SrvWeb::init().await;
    println!(" ----- WEBSERVER RUNS {:?} ----- ", x);
}
