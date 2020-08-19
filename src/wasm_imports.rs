use wasmer_runtime::{
  imports,
  func,
  ImportObject,
  Ctx
};

use altv_capi as capi;
use crate::wasm_util::get_ptr;
use std::mem::{
  transmute,
  size_of
};
use wasmer_runtime::memory::MemoryView;
use core::borrow::BorrowMut;
use altv_capi::alt_StringView;
use crate::wasm_ctxdata::CtxData;
use once_cell::sync::OnceCell;
use std::sync::RwLock;

fn alt_ICore_LogInfo(ctx: &mut Ctx, server: u32, msg: u32)
{
  // server is ptr, 4 bytes
  // msg is ptr to struct, 4 bytes
  // msg.data is ptr to u8, 4 bytes
  // msg.size is u64, 8 bytes

  let mem = ctx.memory(0);
  let memview : MemoryView<u8> = mem.view();

  unsafe {
      let core_ptr = get_ptr(server) as *mut capi::alt_ICore;

      let msg_data = memview[msg as usize + 0].as_ptr() as *mut u32;
      let msg_data_value = memview[*msg_data as usize].as_ptr();

      let msg_size = memview[msg as usize + 4].as_ptr() as *mut u64;
      let msg_size_value = *msg_size;

      let mut msg_c = capi::alt_StringView {
          data: msg_data_value as _,
          size: msg_size_value as _,
      };

      capi::alt_ICore_LogInfo(core_ptr, &mut msg_c);
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

static IMPORTS: OnceCell<ImportObject> = OnceCell::new();

pub fn get() -> &'static ImportObject
{
  IMPORTS.get_or_init(|| {
      imports! {
          // Define the "env" namespace that was implicitly used
          // by our sample application.
          "env" => {
              // the func! macro autodetects the signature
              "alt_ICore_LogInfo" => func!(alt_ICore_LogInfo),

              "malloc" => func!(malloc),
              "free" => func!(free),

              "_Znwm" => func!(_Znwm),
              "_Znam" => func!(_Znam),
              "_ZdlPv" => func!(_ZdlPv),
              "_ZdaPv" => func!(_ZdaPv),
          },
      }
  })
}

