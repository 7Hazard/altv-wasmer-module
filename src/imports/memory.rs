
use wasmer_runtime::{Ctx, ImportObject, imports, func};
use crate::wasm::ctx::WasmCtxExtentions;

// C
fn malloc(ctx: &mut Ctx, size: u32) -> u32
{
  ctx.alloc(size)
}
fn free(ctx: &mut Ctx, ptr: u32)
{
  ctx.dealloc(ptr);
}

// C++
fn _Znwm(ctx: &mut Ctx, size: u32) -> u32
{
  ctx.alloc(size)
}
fn _Znam(ctx: &mut Ctx, size: u32) -> u32
{
  ctx.alloc(size)
}
fn _ZdlPv(ctx: &mut Ctx, ptr: u32)
{
  ctx.dealloc(ptr);
}
fn _ZdaPv(ctx: &mut Ctx, ptr: u32)
{
  ctx.dealloc(ptr);
}

pub fn get() -> ImportObject
{
  imports! {
    "env" => {
      "malloc" => func!(malloc),
      "free" => func!(free),

      "_Znwm" => func!(_Znwm),
      "_Znam" => func!(_Znam),
      "_ZdlPv" => func!(_ZdlPv),
      "_ZdaPv" => func!(_ZdaPv),
    },
  }
}
