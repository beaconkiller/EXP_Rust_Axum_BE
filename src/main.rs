mod controllers;
mod models;
mod services;

use axum::{Router, routing::get};

use crate::controllers::cont_sysinfo::ContSysinfo;

fn create_app() -> Router {
    Router::new().route("/get_sysinfo", get(ContSysinfo::get_sysinfo))
}

#[tokio::main]
async fn main() {
    let app = create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("msg");

    println!("{}", "asdasd");

    axum::serve(listener, app).await.expect("Failed to start");
}
