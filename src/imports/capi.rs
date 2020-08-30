use wasmer_runtime::{imports, func, ImportObject, Ctx, WasmPtr};
use altv_capi;
use std::mem::{
  transmute,
  size_of,
};
use wasmer_runtime::memory::MemoryView;
use core::borrow::BorrowMut;
use crate::wasm_ctxdata::{CtxData, WasmCtxDataGetter};
use once_cell::sync::OnceCell;
use std::sync::RwLock;
use crate::wasm_pointers::WasmPtrExtentions;
use crate::wasm_capi;

pub fn get() -> ImportObject
{
  imports! {
      // Define the "env" namespace that was implicitly used
      // by our sample application.
      "env" => {
          // the func! macro autodetects the signature
          "alt_ICore_LogInfo" => func!(alt_ICore_LogInfo),
          "alt_StringView_Create_6" => func!(alt_StringView_Create_6),
      },
    }
}

fn alt_ICore_LogInfo(
  ctx: &mut Ctx,
  _instance: WasmPtr<wasm_capi::alt_ICore>,
  msg: WasmPtr<wasm_capi::alt_StringView>,
)
{
  std::panic::set_hook(Box::new(|panic_info|{
    if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
      loge!("panic occurred: {:?}", s);
    } else {
      loge!("panic occurred");
    }
  }));
  
  unsafe {
    logi!("1");
    let _instance = _instance.ptr_by_id_err(ctx, &format!("_instance arg was invalid ({})", _instance.offset()));
    logi!("2");
    let mut msg = (*msg.mem_err(ctx, &format!("msg arg was invalid ({})", msg.offset()))).reconstruct_err(ctx, &format!("could not reconstruct msg"));
    logi!("3");
    altv_capi::alt_ICore_LogInfo(_instance, &mut msg as _);
    logi!("4");
  }
  
  std::panic::take_hook();
}

fn alt_StringView_Create_6(
  ctx: &mut Ctx,
  _p0: WasmPtr<altv_capi::alt_StringView>,
  _returnValue: WasmPtr<altv_capi::alt_StringView>,
)
{
  // core is ptr, 4 bytes
  // msg is ptr to struct, 4 bytes
  // msg.data is ptr to u8, 4 bytes
  // msg.size is u64, 8 bytes

  let mem = ctx.memory(0);
  let memview: MemoryView<u8> = mem.view();

//    unsafe {


//        let _p0_ptr = get_ptr(_p0_wasm);
//
//        let mut _p0_reconstruct: altv_capi::alt_StringView;
//        let _p0: *mut altv_capi::alt_StringView = if _p0_ptr == 0 {
//            // reconstruct capi struct from wasm struct
//            _p0_reconstruct.reconstruct(WasmPtr::new(_p0_wasm), mem);
//            &mut _p0_reconstruct
//        } else {
//            // use the struct
//            _p0_ptr as _
//        };
//
//        let _returnValue = altv_capi::alt_StringView;
//        altv_capi::alt_StringView_Create_6(_p0, &mut _returnValue);
//    }
}