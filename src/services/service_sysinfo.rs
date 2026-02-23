use std::collections::HashMap;

use serde_json::Value;
use sysinfo::{Cpu, Disk, Networks};

use crate::{controllers::cont_sysinfo::StrCpuInfo, models::model_disk_info::StrDiskInfo};

pub struct SrvSysinfo;

impl SrvSysinfo {
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
