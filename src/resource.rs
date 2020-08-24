
use altv_capi::*;
use crate::{WasmResource, util};
use wasmer_runtime::memory::MemoryView;
use wasmer_runtime::Value;
use crate::wasm_ctxdata::{WasmCtxDataGetter, CtxData};

pub unsafe extern "C" fn start(resource: *mut alt_IResource) -> bool
{
    let wasmres = WasmResource::from(alt_IResource_GetImpl(resource));
    logi!("Starting {}", wasmres.name);
    
    let inst = &mut wasmres.instance;
    let ctx = inst.context_mut();
    let core_ptr = ctx.ctxdata_mut().ptr_table.get_id_by_ptr(util::core());
    let main = inst.call(
        "altMain",
        &[Value::I32(core_ptr as _)]
    );
    if main.is_ok()
    {
        if main.unwrap()[0].to_u128() == 0 {
            loge!("altMain returned 0 (false)");
            return false;
        }
    }
    else {
        let err = main.err().unwrap();
        loge!("Call to altMain failed \n {}", err.to_string());
        return false;
    }

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
