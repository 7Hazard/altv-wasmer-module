extern crate wasmer_runtime;
extern crate libc;

use wasmer_runtime::{
    imports,
    func,
    ImportObject,
    Ctx
};

use std::ffi::c_void;
use libc::c_char;

#[repr(C)]
pub struct alt_IClient { _private: [u8; 0] }

#[repr(C)]
pub struct alt_StringView {
    data: *const c_char,
    size: libc::size_t
}

#[link(name = "altv-capi-client", kind = "static")]
extern {
    pub fn alt_IClient_LogInfo(_instance: *const alt_IClient, str: *const alt_StringView);
}

fn print_str(ctx: &mut Ctx)
{

}

fn test_wasm(ctx: &mut Ctx, _instance: i64)
{
    unsafe {
        let s = "HELLO FROM WASM";
        let string = std::ffi::CString::new(s).unwrap();
        let altstring = alt_StringView {
            data: string.as_ptr(),
            size: s.len(),
        };

        alt_IClient_LogInfo(_instance as *const alt_IClient, &altstring);
    }
}

// Let's define the import object used to import our function
// into our webassembly sample application.
//
// We've defined a macro that makes it super easy.
//
// The signature tells the runtime what the signature (the parameter
// and return types) of the function we're defining here is.
// The allowed types are `i32`, `u32`, `i64`, `u64`,
// `f32`, and `f64`.
//
// Make sure to check this carefully!

// pub const object: ImportObject = imports! {
//     // Define the "env" namespace that was implicitly used
//     // by our sample application.
//     "env" => {
//         // name        // the func! macro autodetects the signature
//         "print_str" => func!(print_str),
//     },
// };

pub fn get_object() -> ImportObject
{
    let object: ImportObject = imports! {
        // Define the "env" namespace that was implicitly used
        // by our sample application.
        "env" => {
            // the func! macro autodetects the signature
            "print_str" => func!(print_str),
            "test_wasm" => func!(test_wasm),
        },
    };

    return object;
}

