use sysinfo::Disk;

use crate::models::model_disk_info::StrDiskInfo;

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
            usage: d_usage_percent,
            mounted_on: d_mounted_on,
        };

        disk_info
    }
}
