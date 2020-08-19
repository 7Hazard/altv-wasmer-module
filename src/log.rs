
macro_rules! logi {
    ($($arg:tt)*) => ({
        let mut sv = crate::util::stringview(&format!("[WASM] {}", format!($($arg)*))[..]);
        #[allow(unused_unsafe)]
        unsafe { crate::altv_capi::alt_ICore_LogInfo(crate::util::core(), &mut sv); }
    })
}

macro_rules! loge {
    ($($arg:tt)*) => ({
        let mut sv = crate::util::stringview(&format!("[WASM] {}", format!($($arg)*))[..]);
        #[allow(unused_unsafe)]
        unsafe { crate::altv_capi::alt_ICore_LogError(crate::util::core(), &mut sv); }
    })
}

macro_rules! logw {
    ($($arg:tt)*) => ({
        let mut sv = crate::util::stringview(&format!("[WASM] {}", format!($($arg)*))[..]);
        #[allow(unused_unsafe)]
        unsafe { crate::altv_capi::alt_ICore_LogWarning(crate::util::core(), &mut sv); }
    })
}
