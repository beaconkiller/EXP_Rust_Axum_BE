use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrDiskInfo {
    pub usage: String,
    pub mounted_on: String,
}
