mod constant;
mod controllers;
mod global;
mod models;
mod services;

use std::sync::Arc;

use crate::{
    global::Global::GLOBAL_SYS,
    services::{service_pm2::SrvPM2, service_sysinfo::SrvSysinfo, service_web::SrvWeb},
};

#[tokio::main]
async fn main() {
    // =====================================
    // =============== DEBUG ===============
    // =====================================

    let x = SrvPM2;
    x.init().await;

    // =====================================
    // =====================================
    // =====================================

    let srv_sys = Arc::new(SrvSysinfo::new());

    srv_sys.clone().init_loop();
    {
        let mut clone_this = GLOBAL_SYS.lock().unwrap();
        *clone_this = Some(srv_sys);
    }

    SrvWeb::init().await;
}
