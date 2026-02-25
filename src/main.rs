mod constant;
mod controllers;
mod global;
mod models;
mod services;

use std::sync::Arc;

use crate::{
    global::Global::GLOBAL_SYS,
    services::{service_sysinfo::SrvSysinfo, service_web::SrvWeb},
};

#[tokio::main]
async fn main() {
    let srv_sys = Arc::new(SrvSysinfo::new());

    srv_sys.clone().init_loop();
    {
        let mut clone_this = GLOBAL_SYS.lock().unwrap();
        *clone_this = Some(srv_sys);
    }

    let x = SrvWeb::init().await;
    println!(" ----- WEBSERVER RUNS {:?} ----- ", x);
}
