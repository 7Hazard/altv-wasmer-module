//extern crate wasmer_runtime;
//extern crate libc;
//extern crate bimap;

use wasmer_runtime::{
    imports,
    func,
    ImportObject,
    Ctx
};

use altv_server::altv_capi as capi;
use wasm_util::get_ptr;
use std::mem::{
    transmute,
    size_of
};
use wasmer_runtime::memory::MemoryView;
use core::borrow::BorrowMut;
use altv_server::altv_capi::alt_StringView;
use ctxdata::CtxData;

fn alt_IServer_LogInfo(ctx: &mut Ctx, server: u32, msg: u32)
{
    // server is ptr, 4 bytes
    // msg is ptr to struct, 4 bytes
    // msg.data is ptr to u8, 4 bytes
    // msg.size is u64, 8 bytes

    let mem = ctx.memory(0);
    let memview : MemoryView<u8> = mem.view();

    unsafe {
        let server_ptr = get_ptr(server) as *mut capi::alt_IServer;

//        logv!("MSG PTR: {}", msg as u32);

        let msg_data = memview[msg as usize + 0].as_ptr() as *mut u32;
        let msg_data_value = memview[*msg_data as usize].as_ptr();

        let msg_size = memview[msg as usize + 4].as_ptr() as *mut u64;
        let msg_size_value = *msg_size;

        let mut msg_c = capi::alt_StringView {
            data: msg_data_value as _,
            size: msg_size_value as _,
        };

        capi::alt_IServer_LogInfo(server_ptr, &mut msg_c);
    }
}

// C

fn malloc(ctx: &mut Ctx, size: u32) -> u32
{
    let heap = CtxData::get(ctx).heap();
    return heap.alloc(size);
}
fn free(ctx: &mut Ctx, ptr: u32)
{
    let heap = CtxData::get(ctx).heap();
    heap.dealloc(ptr);
}

// C++

fn _Znwm(ctx: &mut Ctx, size: u32) -> u32
{
    let heap = CtxData::get(ctx).heap();
    return heap.alloc(size);
}
fn _Znam(ctx: &mut Ctx, size: u32) -> u32
{
    let heap = CtxData::get(ctx).heap();
    return heap.alloc(size);
}

fn _ZdlPv(ctx: &mut Ctx, ptr: u32)
{
    let heap = CtxData::get(ctx).heap();
    heap.dealloc(ptr);
}
fn _ZdaPv(ctx: &mut Ctx, ptr: u32)
{
    let heap = CtxData::get(ctx).heap();
    heap.dealloc(ptr);
}


pub fn get() -> ImportObject
{
    let object: ImportObject = imports! {
        // Define the "env" namespace that was implicitly used
        // by our sample application.
        "env" => {
            // the func! macro autodetects the signature
            "alt_IServer_LogInfo" => func!(alt_IServer_LogInfo),

            "malloc" => func!(malloc),
            "free" => func!(free),

            "_Znwm" => func!(_Znwm),
            "_Znam" => func!(_Znam),
            "_ZdlPv" => func!(_ZdlPv),
            "_ZdaPv" => func!(_ZdaPv),
        },
    };

    return object;
}

