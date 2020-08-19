
use altv_capi::*;
use crate::{WasmResource, util};
use wasmer_runtime::memory::MemoryView;
use crate::wasm_util::get_id_by_ptr;
use wasmer_runtime::Value;

pub unsafe extern "C" fn start(resource: *mut alt_IResource) -> bool
{
    let wasmres = WasmResource::from(alt_IResource_GetImpl(resource));
    logi!("Starting {}", wasmres.name);

    let inst = &wasmres.wasm;

    // debug
    let mem = inst.context().memory(0);
    let memview: MemoryView<u8> = mem.view();

    let core_id = get_id_by_ptr(util::core());

    let main = inst.call(
        "altMain",
        &[Value::I32(core_id as i32)]
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
