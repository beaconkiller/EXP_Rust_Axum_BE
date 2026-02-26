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

#[derive(Serialize, Clone, Debug)]
pub struct StrClientInfo {
    pub addr: String,
    pub data: StrClientData,
}

#[derive(Serialize, Debug, Clone)]
pub struct StrClientData {
    pub disk_info: Vec<StrDiskInfo>,
    pub cpu_info: Vec<StrCpuInfo>,
    pub mem_info: Option<StrRamInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StrCpuInfo {
    pub i: i16,
    pub cpu_name: String,
    pub cpu_n: i16,
    pub cpu_usage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrRamInfo {
    pub memory_total: u64,
    pub memory_used: u64,
    pub percent: String,
}

#[derive(Serialize, Debug)]
pub struct ContSysinfo;

impl ContSysinfo {
    pub async fn get_sysinfo() -> Json<ApiResponse<StrClientInfo>> {
        println!("{:?}", "Got hit");
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
        // ============ RAM ===========
        // ============================

        let ram_used = sys.used_memory();
        let ram_total = sys.total_memory();
        let ram_data: StrRamInfo = SrvSysinfo::get_ram_info(ram_used, ram_total);
        // println!("{:?}", ram_data);

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
            mem_info: Some(ram_data),
        };

        let config: StrConfig = VarConstant::get_config();
        let addr: String = config.addr;
        let port: String = config.port.to_string();

        let client_info = StrClientInfo {
            addr: format!("{addr}:{port}"),
            data: client_data,
        };

        // println!();
        // println!("{:?}", " ---------- client_data ---------- ");
        // println!("{:#?}", client_info.clone());
        // println!();

        Json(ApiResponse {
            status: 200,
            data: client_info,
            message: "success".to_string(),
        })
    }
}
