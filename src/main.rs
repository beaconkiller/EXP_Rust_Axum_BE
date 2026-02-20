mod repository;

use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use pad::PadStr;
use repository::struct_DiskInfo;
use rust_decimal::dec;
use serde_json::{Value, json};
use std::{array, fmt};
use sysinfo::{Components, Disk, Disks, Networks, System};

use crate::repository::{
    service_sysinfo::Srv_sysinfo, struct_ApiResponse::ApiResponse, struct_DiskInfo::StrDiskInfo,
};

async fn get_sysinfo() -> Json<ApiResponse<Vec<StrDiskInfo>>> {
    let mut sys = System::new_all();

    sys.refresh_all();

    let disks = Disks::new_with_refreshed_list();
    // println!("{:?}", disks);

    let mut new_arr: Vec<StrDiskInfo> = Vec::new();

    for el in disks.list() {
        let x: StrDiskInfo = Srv_sysinfo::_get_disk_info(el);
        new_arr.push(x);
    }

    println!("{:?}", new_arr);

    // println!("System host name:        {:?}", Disk::is_read_only(&self));

    Json(ApiResponse {
        status: (200),
        data: new_arr,
        message: "success".to_string(),
    })
}

fn create_app() -> Router {
    Router::new().route("/get_sysinfo", get(get_sysinfo))
}

#[tokio::main]
async fn main() {
    let app = create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("msg");

    println!("{}", "asdasd");

    get_sysinfo().await;

    axum::serve(listener, app).await.expect("Failed to start");
}
