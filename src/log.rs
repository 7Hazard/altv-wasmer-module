
use altv_capi::*;

macro_rules! logi {
    ($($arg:tt)*) => ({
        let mut sv = util::stringview(&format!("[WASM] {}", format!($($arg)*))[..]);
        alt_ICore_LogInfo(util::core(), &mut sv);
    })
}

macro_rules! loge {
    ($($arg:tt)*) => ({
        let mut sv = util::stringview(&format!("[WASM] {}", format!($($arg)*))[..]);
        alt_ICore_LogError(util::core(), &mut sv);
    })
}

macro_rules! logw {
    ($($arg:tt)*) => ({
        let mut sv = util::stringview(&format!("[WASM] {}", format!($($arg)*))[..]);
        alt_ICore_LogWarning(util::core(), &mut sv);
    })
}
