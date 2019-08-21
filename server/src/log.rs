use std::sync::RwLock;
use once_cell::sync::OnceCell;

/**
 * 
 * */

pub static is_verbose: OnceCell<RwLock<bool>> = OnceCell::new();

macro_rules! logi {
    ($($arg:tt)*) => ({
        altv_server::logi!("[Wasmer] {}", format!($($arg)*));
    })
}

macro_rules! loge {
    ($($arg:tt)*) => ({
        altv_server::loge!("[Wasmer] {}", format!($($arg)*));
    })
}

macro_rules! logw {
    ($($arg:tt)*) => ({
        altv_server::logw!("[Wasmer] {}", format!($($arg)*));
    })
}

macro_rules! logv {
    ($($arg:tt)*) => ({
        if *$crate::log::is_verbose.get_or_init(|| return std::sync::RwLock::new(false)).read().unwrap() {
            altv_server::logi!("[Wasmer] {}", format!($($arg)*));
        }
    })
}
