
use altv_server::{
    altv_capi::*,
    string,
    string_view,
};

use wasmer_runtime::{Value, Instance};
use wasm_util::{get_id, get_id_by_ptr};
use std::sync::RwLock;
use std::collections::{HashMap, HashSet};
use wasmer_runtime::memory::MemoryView;
use wasmer_runtime::units::Pages;

pub struct WasmResource {
    pub name: String,
//    pub on_event_callbacks: HashSet<u32>,
    pub wasm: Instance,
}

impl WasmResource {
    pub fn from<'a>(res: *mut alt_IResource) -> &'a mut WasmResource
    {
        unsafe {
            return &mut *(alt_CAPIResource_GetExtra(res) as *mut WasmResource);
        };
    }
}

pub extern "C" fn on_make_client(
    res: * mut alt_IResource,
    info: * mut alt_IResource_CreationInfo,
    files: * mut alt_Array_String
)
{
    logv!("on_make_client");
}

// What does Instantiating infer???
pub extern "C" fn on_instantiate(res: * mut alt_IResource) -> bool
{
    let wasmres = WasmResource::from(res);
//    logv!("Insantiating {}", wasmres.name);

    return true;
}

pub extern "C" fn on_start(res: * mut alt_IResource) -> bool
{
    let wasmres = WasmResource::from(res);
    logv!("Starting {}", wasmres.name);

    let inst = unsafe { &wasmres.wasm };

    // debug
    let mem = inst.context().memory(0);
    let memview: MemoryView<u8> = mem.view();

    let server_ptr = unsafe {
        alt_IServer_Instance()
    };
    let server_id = unsafe {
        get_id_by_ptr(server_ptr)
    };

    let main = inst.call(
        "altMain",
        &[Value::I32(server_id as i32)]
    );
    if main.is_ok()
    {
        if main.unwrap()[0].to_u64() == 0 {
            loge!("altMain returned 0 (false)");
            return false;
        }
    }
    else {
        let err = main.err().unwrap();
        loge!("Call to altMain failed \n {}", err.to_string());
        return false;
    }

    return true;
}

pub extern "C" fn on_stop (res: * mut alt_IResource) -> bool
{
    let wasmres = WasmResource::from(res);
    logv!("TODO: Stopping {}", wasmres.name);

//    unsafe {
//        // get the box and let it be destructed
//        Box::from_raw(wasmres);
//    }

    return true;
}

pub extern "C" fn on_event(
    res: * mut alt_IResource,
    ev: * mut alt_CEvent
) -> bool
{
    logv!("on_event WASM Resource");

//    let wasmres = WasmResource::from(res);
//
//    let mem: MemoryView<u8> = wasmres.wasm_instance.context_mut()
//        .memory(0).view();
//    let pages = wasmres.wasm_instance.context_mut()
//        .memory(0).grow(Pages(1));
//
//    if pages.is_ok() {
//        let page = pages.unwrap();
//        pag
//
//    }
//    else {
//
//    }
//
//
//    wasmres.wasm_instance.call("alt_WASM_OnEvent", &[]);

    return true;
}

pub extern "C" fn on_tick(res: * mut alt_IResource)
{

}

pub extern "C" fn on_create_base_object(
    res: * mut alt_IResource,
    obj: * mut alt_IBaseObject
)
{
    logv!("on_create_base_object WASM Resource, obj 0x{:x}", obj as u64);

}

pub extern "C" fn on_remove_base_object(
    res: * mut alt_IResource,
    obj: * mut alt_IBaseObject
)
{
    logv!("on_remove_base_object WASM Resource, obj 0x{:x}", obj as u64);

}

