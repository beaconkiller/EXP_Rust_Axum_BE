use axum::{Router, routing::get};

use crate::controllers::cont_sysinfo::ContSysinfo;
pub struct SrvRoutes;

impl SrvRoutes {
    pub fn get_routes() -> Vec<fn(Router) -> Router> {
        let arr_routes: Vec<fn(Router) -> Router> = vec![
            |v| v.route("/get_sysinfo", get(ContSysinfo::get_sysinfo)),
            |v| v.route("/get_status", get(ContSysinfo::get_sysinfo)),
        ];

        arr_routes
    }
}
