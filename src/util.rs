
use altv_capi::*;
use std::ffi::CString;

pub fn stringview(s: &str) -> alt_StringView
{
    let cstr = CString::new(s).unwrap(); // make a cstr
    alt_StringView{ data: cstr.into_raw(), size: s.len() as _ }
}

pub fn core() -> *mut alt_ICore
{
    unsafe {
        alt_ICore_Instance()
    }
}
