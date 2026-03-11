use std::sync::{Arc, LazyLock, Mutex};

use crate::services::{
    service_pm2::{SrvPM2, StrPM2Output},
    service_sysinfo::SrvSysinfo,
};

pub static GLOBAL_SYS: LazyLock<Mutex<Option<Arc<SrvSysinfo>>>> =
    LazyLock::new(|| Mutex::new(std::option::Option::None));

pub static GL_SRV_PM2: LazyLock<Arc<SrvPM2>> = LazyLock::new(|| {
    Arc::new(SrvPM2 {
        data: vec![].into(),
    })
});
