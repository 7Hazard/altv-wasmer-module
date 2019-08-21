

use altv_server::{
    altv_capi::*,
    resource,
    string_view,
    string,
};

use wasm_resource;
use std::fs;
use std::io::Read;

use wasmer_runtime;
use imports;
use wasm_resource::WasmResource;
use ctxdata::CtxData;

pub extern "C" fn create_resource(
    rt: *mut alt_IScriptRuntime,
    info: *mut alt_IResource_CreationInfo
) -> *mut alt_IResource
{
    let mut inf = unsafe { *info };

    let main = string::from(&mut inf.main);
    let path = string::from(&mut inf.path);
    let mainpath = format!("{}/{}", path, main);
    let fileres = fs::File::open(mainpath);
    if !fileres.is_ok() {
        loge!("Main is not ok \n Main was: {} \n Path to main was: {}", main, path);
    }

    let mut file = fileres.unwrap();

    let mut wasm  = vec![];
    file.read_to_end(&mut wasm);

    let instance = wasmer_runtime::instantiate(
        &wasm,
        &imports::get()
    );

    if !instance.is_ok() {
        let err = instance.err().unwrap();
        loge!("Could not create WASM instance \n {}", err.to_string());

        return std::ptr::null_mut();
    }

    let altres = altv_server::resource::new(
        info,
        wasm_resource::on_make_client,
        wasm_resource::on_instantiate,
        wasm_resource::on_start,
        wasm_resource::on_stop,
        wasm_resource::on_event,
        wasm_resource::on_tick,
        wasm_resource::on_create_base_object,
        wasm_resource::on_remove_base_object
    );

    unsafe {
        let mut wasmres = Box::new(
            WasmResource {
                name: string::from(&mut (*info).name),
                wasm: instance.unwrap(),
            }
        );

        // Create CtxData (Heap etc...)
        CtxData::attach(&mut wasmres.wasm);

        alt_CAPIResource_SetExtra(altres, Box::into_raw(wasmres) as _);
    }

    return altres;
}

pub extern "C" fn remove_resource(
    rt: *mut alt_IScriptRuntime,
    res: *mut alt_IResource
)
{
    let wasmres = WasmResource::from(res);
    logv!("Removing {}", wasmres.name);

    unsafe {
        // get the box and let it be destructed
        Box::from_raw(wasmres);
    }
}

pub extern "C" fn on_tick(
    rt: *mut alt_IScriptRuntime,
)
{

}
