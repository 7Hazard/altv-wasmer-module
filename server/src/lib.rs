
extern crate wasmer_runtime;
extern crate libc;
extern crate once_cell;
extern crate altv_server;
extern crate core;

use altv_server::{
    API,
    register_runtime,
    runtime,
    set_api,
    get_version,
};

use wasmer_runtime::{
    Instance,
    instantiate,
};

use std::{
    ffi::{
        c_void,
        CStr,
        CString
    },
    ptr,
    slice,
    os::raw::c_char
};
use std::sync::RwLock;

#[macro_use]
mod log;

mod memory;
mod ctxdata;
mod imports;
mod wasm_resource;
mod wasm_runtime;
mod wasm_util;

#[no_mangle]
pub extern "C" fn altMain(api: *mut API) -> bool
{
    set_api(api);

    // Currently verbose by default
    log::is_verbose.get_or_init(|| return RwLock::new(true));

    let rt = runtime::new(
        wasm_runtime::create_resource,
        wasm_runtime::remove_resource,
        wasm_runtime::on_tick
    );
    register_runtime(api, "wasmer", rt);

    logi!("Module loaded");

    return true;
}

#[no_mangle]
pub extern "C" fn GetSDKVersion() -> u32
{
    return get_version();
}
