
use altv_capi::*;
use crate::{WasmResource, util};
use wasmer_runtime::memory::MemoryView;
use wasmer_runtime::Value;
use crate::wasm::ctx::WasmCtxExtentions;

pub unsafe extern "C" fn start(resource: *mut alt_IResource) -> bool
{
    let wasmres = WasmResource::from(alt_IResource_GetImpl(resource));
    logi!("Starting {}", wasmres.name);

    // start functions
    match wasmres.instance.call("_start", &[]) {
        Err(e) => {}
        Ok(v) => {}
    };
    match wasmres.instance.call("__wasm_call_ctors", &[]) {
        Err(e) => {}
        Ok(v) => {}
    };

    // altMain
    let core_ptr = wasmres.instance.context_mut().data_mut().ptr_table.get_id_by_ptr(util::core());
    match wasmres.instance.call("altMain", &[Value::I32(core_ptr as _)]) {
        Err(e) => {
            loge!("Call to altMain failed \n {}", e.to_string());
            return false;
        },
        Ok(val) => {
            if val[0].to_u128() == 0 {
                loge!("altMain returned 0 (false)");
                return false;
            }
        }
    };

    true
}

pub unsafe extern "C" fn stop(resource: *mut alt_IResource) -> bool
{
    logi!("STOP RESOURCE");
    true
}

pub unsafe extern "C" fn event(resource: *mut alt_IResource, event: *mut alt_CEvent) -> bool
{

    true
}

pub unsafe extern "C" fn tick(resource: *mut alt_IResource)
{

}

pub unsafe extern "C" fn create_object(resource: *mut alt_IResource, object: *mut alt_RefBase_RefStore_IBaseObject)
{

}

pub unsafe extern "C" fn destroy_object(resource: *mut alt_IResource, object: *mut alt_RefBase_RefStore_IBaseObject)
{

}
