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

mod memory;

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
use once_cell::sync::OnceCell;
use std::sync::RwLock;
use altv_capi;
use crate::wasm::ctx::WasmCtxExtentions;

static IMPORTS: OnceCell<ImportObject> = OnceCell::new();

pub fn get() -> &'static ImportObject
{
    IMPORTS.get_or_init(|| {
        let mut i: ImportObject = imports! {
          // Define the "env" namespace that was implicitly used
          // by our sample application.
          "env" => {
              // the func! macro autodetects the signature
          },
        };
        i.extend(capi::get().into_iter());
        i.extend(wasi::get().into_iter());
        i.extend(memory::get().into_iter());

        i
    })
}
