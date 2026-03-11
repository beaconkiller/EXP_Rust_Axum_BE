use std::{os::windows::process, process::Command, time::Duration};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug)]
pub struct StrPM2Output {
    pub name: Option<String>,
    pub status: Option<String>,
    pub uptime: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SrvPM2;

impl SrvPM2 {
    pub async fn init(&self) {
        Self::loop_pm2(&self).await
    }

    pub async fn loop_pm2(&self) {
        // println!("{}", "Loop pm2 ");
        loop {
            Self::get_pm2_jlist().await;
            tokio::time::sleep(Duration::from_millis(5000)).await
        }
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
        // println!("{:#?}", arr_pm2_raw);
        // println!();
        // println!("{:#?}", "--------------------------");
        // println!();

        let mut tmp_arr: Vec<StrPM2Output> = vec![];
        if let Some(processes) = arr_pm2_raw.as_array() {
            for proc in processes {
                let name = proc["name"].as_str().unwrap_or_default();

                let mut status: Option<&str> = None;
                let mut uptime: Option<i64> = None;
                if let Some(pm2_env) = proc["pm2_env"].as_object() {
                    // println!("{:#?}", pm2_env);
                    uptime = pm2_env["pm_uptime"].as_i64();
                    status = pm2_env["status"].as_str();
                }

                let strpm2output = StrPM2Output {
                    name: Some(name.to_string()),
                    status: Some(status.clone().unwrap().to_string()),
                    uptime: uptime,
                };
                tmp_arr.push(strpm2output);
            }
        }

        println!("{:?}", tmp_arr);

        tmp_arr
    }
}
