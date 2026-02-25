use std::collections::HashMap;
use std::sync::Arc;

use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sysinfo::{Disks, Networks, System};

use crate::GLOBAL_SYS;
use crate::constant::var_constant::{StrConfig, VarConstant};
use crate::models::{model_api_response::ApiResponse, model_disk_info::StrDiskInfo};
use crate::services::service_sysinfo::SrvSysinfo;

#[derive(Serialize)]
pub struct StrClientInfo {
    pub addr: String,
    pub data: StrClientData,
}

#[derive(Serialize)]
pub struct StrClientData {
    pub disk_info: Vec<StrDiskInfo>,
    pub cpu_info: Vec<StrCpuInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StrCpuInfo {
    pub i: i16,
    pub cpu_name: String,
    pub cpu_n: i16,
    pub cpu_usage: String,
}

#[derive(Serialize)]
pub struct ContSysinfo;

impl ContSysinfo {
    pub async fn get_sysinfo() -> Json<ApiResponse<StrClientInfo>> {
        // pub async fn get_sysinfo() -> Json<ApiResponse<Vec<StrDiskInfo>>> {
        let mut srv_sysinfo: Arc<SrvSysinfo> = GLOBAL_SYS.lock().unwrap().clone().unwrap();
        let mut sys = srv_sysinfo.instance_sys.lock().await;

        sys.refresh_all();

        let disks = Disks::new_with_refreshed_list();

        // ============================
        // ============ CPU ===========
        // ============================

        let cpu_info = sys.cpus();
        let cpu_data: Vec<StrCpuInfo> = SrvSysinfo::get_cpu_info(cpu_info);

        // ============================
        // =========== DISKS ==========
        // ============================

        let mut new_arr: Vec<StrDiskInfo> = Vec::new();
        for el in disks.list() {
            let x: StrDiskInfo = SrvSysinfo::get_disk_info(el);
            new_arr.push(x);
        }

        // ================================
        // ============ NETWORK ===========
        // ================================

        let networks: Networks = Networks::new_with_refreshed_list();
        // SrvSysinfo::get_netw_info(networks);

        let client_data = StrClientData {
            cpu_info: cpu_data,
            disk_info: new_arr,
        };

        let config: StrConfig = VarConstant::get_config();
        let addr: String = config.addr;
        let port: String = config.port.to_string();

        let client_info = StrClientInfo {
            addr: format!("{addr}:{port}"),
            data: client_data,
        };

        Json(ApiResponse {
            status: 200,
            data: client_info,
            message: "success".to_string(),
        })
    }
}
