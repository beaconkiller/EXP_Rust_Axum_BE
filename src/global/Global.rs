use std::sync::{Arc, LazyLock, Mutex};

use crate::services::service_sysinfo::SrvSysinfo;

pub static GLOBAL_SYS: LazyLock<Mutex<Option<Arc<SrvSysinfo>>>> =
    LazyLock::new(|| Mutex::new(std::option::Option::None));
