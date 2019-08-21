extern crate wasmer_runtime;
extern crate libc;
// #[macro_use]
// extern crate lazy_static;

// mod altv_capi;
mod imports;

use wasmer_runtime::{
    Instance,
    instantiate,
    Value
};

use std::{
    ptr,
    ffi::c_void,
    slice
};

// #[no_mangle]
// pub extern "C" fn altv_wasmer_start_runtime()
// {
    
// }


#[no_mangle]
pub unsafe extern "C" fn altv_wasmer_create_instance(
    wasmbytes: *const u8,
    wasmbyteslen: usize
) -> *mut c_void
{
    let wasm: &[u8] = slice::from_raw_parts(wasmbytes, wasmbyteslen);
    let instance = instantiate(wasm, &imports::get_object());
    let boxed_instance = match instance {
        Ok(i) => 
            Box::into_raw(Box::new(i)) as *mut c_void,
        Err(err) =>
            // msg
            ptr::null_mut() as *mut c_void
    };
    return boxed_instance;
}

#[no_mangle]
pub unsafe extern "C" fn altv_wasmer_dispose_instance(instance: *mut c_void)
{
    drop(Box::from_raw(instance));
}

#[no_mangle]
pub unsafe extern "C" fn altv_wasmer_invoke_main(
    instance: *mut c_void,
    api_instance: *const c_void, // IClient*
)
{
    let i = Box::from_raw(instance as *mut Instance);

    // pass api_instance ptr as 64-bit int
    let res = (*i).call("main", &[Value::I64(api_instance as i64)]);

    match res {
        Ok(ret) => {
            //ok
        },
        Err(err) => {
            //no
        }
    }
}
