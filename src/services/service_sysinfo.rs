use std::{alloc::System, collections::HashMap, sync::Arc, time::Duration};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sysinfo::{Cpu, Disk, Disks, Networks};
use tokio::sync::Mutex;

use crate::{
    controllers::cont_sysinfo::{ContSysinfo, StrCpuInfo, StrRamInfo},
    global::Global::GLOBAL_SYS,
    models::model_disk_info::StrDiskInfo,
};

#[derive(Serialize, Debug)]
pub struct StrClientData {
    pub disk_info: Vec<StrDiskInfo>,
    pub cpu_info: Vec<StrCpuInfo>,
    pub mem_info: Option<StrRamInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StrClientInfo {}

#[derive(Debug)]
pub struct SrvSysinfo {
    pub disk_info: Mutex<Option<Vec<StrDiskInfo>>>,
    pub cpu_info: Mutex<Option<Vec<StrCpuInfo>>>,
    pub instance_sys: Mutex<sysinfo::System>,
}

impl SrvSysinfo {
    pub fn new() -> Self {
        let mut sys = sysinfo::System::new();
        Self {
            disk_info: Mutex::new(None),
            cpu_info: Mutex::new(None),
            instance_sys: Mutex::new(sys),
        }
    }

    pub fn init_loop(self: Arc<Self>) {
        // let sys = sysinfo.
        tokio::spawn(async move {
            println!("{:?}", "----------------------------------------");
            println!("{:?}", "------- SrvSysInfo Loop started. -------");
            println!("{:?}", "----------------------------------------");
            loop {
                {
                    let mut sys = self.instance_sys.lock().await;
                    sys.refresh_all();

                    // if let Some(sys) = GLOBAL_SYS.lock().unwrap().as_ref() {
                    //     println!("{:?}", sys);
                    // }

                    // println!("{:?}", sys.cpus());

                    // println!("{:?}", sys.cpus());
                }
                tokio::time::sleep(Duration::from_secs(1)).await
            }
        });
    }

    pub fn get_disk_info(el: &Disk) -> StrDiskInfo {
        let d_total: f64 = el.total_space() as f64;
        let d_avail: f64 = el.available_space() as f64;
        let d_usage: f64 = d_total - d_avail;
        let d_usage_percent: f64 = format!("{:.2}", ((d_usage / d_total) * 100.0))
            .parse()
            .unwrap();

        let d_mounted_on: String = el.mount_point().to_string_lossy().to_string();
        let disk_info: StrDiskInfo = StrDiskInfo {
            usage: d_usage_percent.to_string(),
            mounted_on: d_mounted_on,
        };
        disk_info
    }

    pub fn get_netw_info(data: Networks) {
        println!("{:?}", " ========= data ========= ");

        for el in &data {
            println!("{:?}", el.0);
            println!("{:?}", el);
        }
    }

    pub fn get_cpu_info(data: &[Cpu]) -> Vec<StrCpuInfo> {
        let mut arr = Vec::new();
        let mut i = 1;
        for el in data {
            arr.push(StrCpuInfo {
                cpu_n: data.len() as i16,
                cpu_name: data[0].brand().to_string(),
                cpu_usage: (format!("{:.2}", el.cpu_usage())),
                i: i,
            });
            i += 1;
        }
        arr
    }

    pub fn get_ram_info(ram_used: u64, ram_total: u64) -> StrRamInfo {
        let percent = format!("{:.2}", (ram_used as f64 / ram_total as f64) * 100 as f64);
        StrRamInfo {
            memory_total: ram_total,
            memory_used: ram_used,
            percent: percent.to_string(),
        }
    }

    pub async fn get_all_info() -> StrClientData {
        // let guard = GLOBAL_SYS.lock().unwrap();
        let srv_sysinfo: Arc<SrvSysinfo> = GLOBAL_SYS.lock().unwrap().clone().unwrap();
        // println!("{:?}", guard);
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

        // ================================
        // =========== PM2 DATA ===========
        // ================================

        // let pm2_data = GL_SRV_PM2.get_mapped_pm2_output().await;

        let client_data = StrClientData {
            cpu_info: cpu_data,
            disk_info: new_arr,
            mem_info: Some(ram_data),
        };

        client_data
    }
}
