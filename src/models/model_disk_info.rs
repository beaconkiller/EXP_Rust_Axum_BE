use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct StrDiskInfo {
    pub usage: String,
    pub mounted_on: String,
}
