extern crate altv_capi;
extern crate wasmer_runtime;

use altv_capi::*;
use std::option::Option;
use std::ffi::{CString, CStr};
use std::ptr::{null, null_mut};
use wasmer_runtime::{Module, compile, Instance};
use crate::wasm::ctxdata::CtxData;

mod util;
#[macro_use]
mod log;
mod resource;
mod wasm;
mod wasm_capi;
#[allow(non_snake_case)]
mod imports;

pub struct WasmResource {
    name: String,
    instance: Instance,
}

impl WasmResource {
    pub fn from<'a>(res: *mut alt_IResource_Impl) -> &'a mut WasmResource
    {
        unsafe {
            return &mut *(alt_CAPIResource_Impl_GetExtra(res) as *mut WasmResource);
        };
    }

//  pub fn instance(&self) -> &Instance {
//    &self.instance
//  }
//  pub fn instance_mut(&mut self) -> &mut Instance {
//    &mut self.instance
//  }
}

unsafe extern "C" fn create_resource(runtime: *mut alt_IScriptRuntime, resource: *mut alt_IResource) -> *mut alt_IResource_Impl
{
    let mut name = alt_StringView { size: 0, data: null_mut() };
    alt_IResource_GetName(resource, &mut name);

    let mut main = alt_StringView { size: 0, data: null_mut() };
    alt_IResource_GetMain(resource, &mut main);
    if main.size < 1 {
        loge!("'client-main' must be specified in resource.cfg");
        return null_mut();
    }
    let pkg = alt_IResource_GetPackage(resource);
    let file = alt_IPackage_OpenFile(pkg, &mut main);
    if file.is_null() {
        loge!("Could not open file {}", util::from_stringview(&main));
        return null_mut();
    }

    let size = alt_IPackage_GetFileSize(pkg, file);
    let mut src = Vec::with_capacity(size as _);
    src.set_len(size as _);
    let readsize = alt_IPackage_ReadFile(pkg, file, src.as_mut_ptr() as _, size);
    alt_IPackage_CloseFile(pkg, file);
    if readsize != size {
        let mainstr = util::from_stringview(&mut main);
        logi!("Could not read {} appropriately", mainstr);
        return null_mut();
    }

    let module = match compile(&src) {
        Err(e) => {
            loge!("Error compiling module: {:?}", e);
            return null_mut();
        }
        Ok(module) => module
    };

    let mut wasmres = Box::new(
        WasmResource {
            name: String::from(util::from_stringview(&mut main)),
            instance: match module.instantiate(imports::get()) {
                Err(e) => {
                    loge!("Error instantiating module: {}", e.to_string());
                    return null_mut();
                }
                Ok(val) => val
            }
        }
    );

    // Attach CtxData
    wasmres.instance.context_mut().data = Box::into_raw(Box::new(CtxData::new(&wasmres.instance))) as _;

    let resource_impl = alt_CAPIResource_Impl_Create(
        resource,
        Some(resource::start),
        Some(resource::stop),
        Some(resource::event),
        Some(resource::tick),
        Some(resource::create_object),
        Some(resource::destroy_object),
    );

    alt_CAPIResource_Impl_SetExtra(resource_impl, Box::into_raw(wasmres) as _);

    resource_impl
}

unsafe extern "C" fn remove_resource(runtime: *mut alt_IScriptRuntime, resource_impl: *mut alt_IResource_Impl)
{
    logi!("REMOVE RESOURCE");
}

unsafe extern "C" fn tick(runtime: *mut alt_IScriptRuntime)
{}

#[no_mangle]
pub unsafe extern "C" fn create_wasm_runtime(core: *mut alt_ICore) -> *mut alt_IScriptRuntime
{
    alt_ICore_SetInstance(core);

    let runtime = alt_CAPIScriptRuntime_Create(
        Some(create_resource),
        Some(remove_resource),
        Some(tick),
    );

    logi!("Initialized runtime");

    runtime
}
