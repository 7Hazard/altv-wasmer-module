#![allow(non_camel_case_types)]

use wasmer_runtime::{imports, func, ImportObject, Export, Ctx};

pub fn get() -> ImportObject
{
    imports! {
        "wasi_snapshot_preview1" => {
            "proc_exit" => func!(proc_exit)
//            "args_get" => func!(args_get)
//            "args_sizes_get" => get_fn("args_sizes_get")
        },
    }
}

fn proc_exit(ctx: &mut Ctx, code: i32) {
    loge!("proc_exit CALLED WITH CODE {} (THIS FUNCTION DOES NOTHING)", code);
}

//fn args_get(
//    ctx: &mut Ctx,
//    argv: WasmPtr<WasmPtr<u8, Array>, Array>,
//    argv_buf: WasmPtr<u8, Array>,
//) -> __wasi_errno_t
//{
//
//}

//fn args_sizes_get(ctx: &mut Ctx, code: __wasi_exitcode_t) {
//    loge!("proc_exit CALLED WITH CODE {} (THIS FUNCTION DOES NOTHING)", code);
//}
