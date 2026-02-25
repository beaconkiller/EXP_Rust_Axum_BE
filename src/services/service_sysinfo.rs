use std::{alloc::System, collections::HashMap, sync::Arc, time::Duration};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sysinfo::{Cpu, Disk, Networks};
use tokio::sync::Mutex;

use crate::{controllers::cont_sysinfo::StrCpuInfo, models::model_disk_info::StrDiskInfo};

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
            println!("{:?}", "----------------------------");
            println!("{:?}", "Loop started.");
            println!("{:?}", "----------------------------");
            loop {
                println!("{}", "asdasd");
                {
                    let mut sys = self.instance_sys.lock().await;
                    sys.refresh_all();
                    println!("{:?}", sys.cpus())
                }

                tokio::time::sleep(Duration::from_secs(2)).await
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

        // println!("{:?}", disk_info);

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
}
