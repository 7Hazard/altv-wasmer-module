
use altv_capi::*;
use std::ffi::{CString, CStr};
use std::slice::from_raw_parts;
use std::str::from_utf8;

pub fn stringview(s: &str) -> alt_StringView
{
    let cstr = CString::new(s).unwrap(); // make a cstr
    alt_StringView{ data: cstr.into_raw(), size: s.len() as _ }
}

// TODO: Make it more efficient and make use of sv.size
pub fn from_stringview(sv: &alt_StringView) -> &str
{
    unsafe {
        CStr::from_ptr(sv.data as _).to_str().unwrap_or("FAILED from_stringview")
    }
}

pub fn core() -> *mut alt_ICore
{
    unsafe {
        alt_ICore_Instance()
    }
}
