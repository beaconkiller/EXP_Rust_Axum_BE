use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct StrDiskInfo {
    pub usage: f64,
    pub mounted_on: String,
}
