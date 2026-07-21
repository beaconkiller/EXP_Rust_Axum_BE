use std::sync::{Arc, LazyLock, Mutex, atomic::AtomicI32};

use crate::services::{
    service_pm2::{SrvPM2, StrPM2Output},
    service_sysinfo::SrvSysinfo,
    service_ws::SrvWs,
};

pub static GLOBAL_SYS: LazyLock<Mutex<Option<Arc<SrvSysinfo>>>> =
    LazyLock::new(|| Mutex::new(std::option::Option::None));

pub static GL_WS: LazyLock<Arc<SrvWs>> = LazyLock::new(|| {
    Arc::new(SrvWs {
        arr_clients: tokio::sync::Mutex::new(vec![].into()),
        status: AtomicI32::new(0),
        tx: tokio::sync::Mutex::new(None),
    })
});

pub static GL_SRV_PM2: LazyLock<Arc<SrvPM2>> = LazyLock::new(|| {
    Arc::new(SrvPM2 {
        data: vec![].into(),
    })
});
