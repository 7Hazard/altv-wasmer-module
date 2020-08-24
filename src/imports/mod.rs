#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod capi;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod wasi;

use wasmer_runtime::{
    imports,
    func,
    ImportObject,
    Ctx
};

use std::mem::{
    transmute,
    size_of
};
use wasmer_runtime::memory::MemoryView;
use core::borrow::BorrowMut;
use altv_capi::alt_StringView;
use crate::wasm_ctxdata::{CtxData, WasmCtxDataGetter};
use once_cell::sync::OnceCell;
use std::sync::RwLock;
use altv_capi;

// C

fn malloc(ctx: &mut Ctx, size: u32) -> u32
{
    ctx.ctxdata_mut().heap.alloc(size)
}
fn free(ctx: &mut Ctx, ptr: u32)
{
    ctx.ctxdata_mut().heap.dealloc(ptr);
}

// C++

fn _Znwm(ctx: &mut Ctx, size: u32) -> u32
{
    ctx.ctxdata_mut().heap.alloc(size)
}
fn _Znam(ctx: &mut Ctx, size: u32) -> u32
{
    ctx.ctxdata_mut().heap.alloc(size)
}

fn _ZdlPv(ctx: &mut Ctx, ptr: u32)
{
    ctx.ctxdata_mut().heap.dealloc(ptr);
}
fn _ZdaPv(ctx: &mut Ctx, ptr: u32)
{
    ctx.ctxdata_mut().heap.dealloc(ptr);
}

static IMPORTS: OnceCell<ImportObject> = OnceCell::new();

pub fn get() -> &'static ImportObject
{
    IMPORTS.get_or_init(|| {
        let mut i: ImportObject = imports! {
          // Define the "env" namespace that was implicitly used
          // by our sample application.
          "env" => {
              // the func! macro autodetects the signature
              "malloc" => func!(malloc),
              "free" => func!(free),

              "_Znwm" => func!(_Znwm),
              "_Znam" => func!(_Znam),
              "_ZdlPv" => func!(_ZdlPv),
              "_ZdaPv" => func!(_ZdaPv),
          },
        };
        i.extend(capi::get().into_iter());
        i.extend(wasi::get().into_iter());

        i
    })
}
