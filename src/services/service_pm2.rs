use std::{os::windows::process, process::Command, time::Duration};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::Mutex;

use crate::global::Global::GL_SRV_PM2;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StrPM2Send {
    pub name: Option<String>,
    pub status: Option<String>,
    pub uptime: Option<i64>,
    pub id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StrPM2Output {
    pub name: Option<String>,
    pub status: Option<String>,
    pub uptime: Option<i64>,
    pub id: Option<String>,
}

#[derive(Debug)]
pub struct SrvPM2 {
    pub data: tokio::sync::Mutex<Vec<StrPM2Output>>,
}

impl SrvPM2 {
    pub async fn init(&self) {
        tokio::spawn(async move {
            loop {
                let new_data = Self::get_pm2_jlist().await;
                let SrvPm2 = &GL_SRV_PM2;
                // println!("{:?}", &new_data);
                {
                    let mut curr_data = SrvPm2.data.lock().await;
                    *curr_data = new_data;
                }
                tokio::time::sleep(Duration::from_millis(1000)).await
            }
        });
    }

    pub async fn get_pm2_jlist() -> Vec<StrPM2Output> {
        #[cfg(target_os = "windows")]
        let output = Command::new("cmd")
            .args(["/C", "pm2", "jlist"])
            .output()
            .expect("failed to execute pm2");

        #[cfg(not(target_os = "windows"))]
        let output = Command::new("pm2")
            .args(["jlist"])
            .output()
            .expect("failed to execute pm2");

        let arr_pm2_raw: Value = serde_json::from_slice(&output.stdout).unwrap();
        println!("{:#?}", arr_pm2_raw);
        println!();
        println!("{:#?}", "--------------------------");
        println!();

        let mut tmp_arr: Vec<StrPM2Output> = vec![];
        if let Some(processes) = arr_pm2_raw.as_array() {
            for proc in processes {
                let name = proc["name"].as_str().unwrap_or_default();

                let mut status: Option<&str> = None;
                let mut uptime: Option<i64> = None;
                let mut id: Option<String> = None;
                if let Some(pm2_env) = proc["pm2_env"].as_object() {
                    // println!("{:#?}", pm2_env);
                    uptime = pm2_env["pm_uptime"].as_i64();
                    status = pm2_env["status"].as_str();
                    id = Some(pm2_env["pm_id"].to_string());
                }

                let strpm2output = StrPM2Output {
                    name: Some(name.to_string()),
                    status: Some(status.clone().unwrap().to_string()),
                    uptime: uptime,
                    id: id,
                };
                tmp_arr.push(strpm2output);
            }
        }

        // println!("{:?}", tmp_arr);
        tmp_arr
    }

    pub async fn get_mapped_pm2_output(&self) -> Vec<StrPM2Send> {
        let arr_data = self.data.lock().await;

        // println!("{:?}", " ---------- arr_data ---------- ");
        // println!("{:?}", arr_data);

        let mut arr_tmp: Vec<StrPM2Send> = vec![];
        for el in arr_data.iter() {
            arr_tmp.push(StrPM2Send {
                name: el.name.clone(),
                status: el.status.clone(),
                uptime: el.uptime,
                id: el.id.clone(),
            });
        }
        arr_tmp
    }
}
