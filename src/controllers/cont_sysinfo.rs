use axum::Json;
use sysinfo::{Disks, System};

use crate::services::service_sysinfo::SrvSysinfo;

use crate::models::{model_api_response::ApiResponse, model_disk_info::StrDiskInfo};

pub struct ContSysinfo;

impl ContSysinfo {
    pub async fn get_sysinfo() -> Json<ApiResponse<Vec<StrDiskInfo>>> {
        let mut sys: System = System::new_all();

        sys.refresh_all();

        let disks = Disks::new_with_refreshed_list();
        // println!("{:?}", disks);

        let mut new_arr: Vec<StrDiskInfo> = Vec::new();

        for el in disks.list() {
            let x: StrDiskInfo = SrvSysinfo::get_disk_info(el);
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
}
