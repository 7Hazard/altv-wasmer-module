extern crate altv_capi;

use altv_capi::*;
use std::option::Option;
use std::ffi::CString;

mod util;
#[macro_use]
mod log;
mod resource;

unsafe extern "C" fn create_resource(runtime: *mut alt_IScriptRuntime, resource: *mut alt_IResource) -> *mut alt_IResource_Impl
{
  alt_CAPIResource_Impl_Create(
    resource,
    Some(resource::start),
    Some(resource::stop),
    Some(resource::event),
    Some(resource::tick),
    Some(resource::create_object),
    Some(resource::destroy_object)
  )
}

unsafe extern "C" fn remove_resource(runtime: *mut alt_IScriptRuntime, resource_impl: *mut alt_IResource_Impl)
{

}

unsafe extern "C" fn tick(runtime: *mut alt_IScriptRuntime)
{

}

#[no_mangle]
pub unsafe extern "C" fn create_wasm_runtime(core: *mut alt_ICore) -> *mut alt_IScriptRuntime
{
  alt_ICore_SetInstance(core);

  let runtime = alt_CAPIScriptRuntime_Create(
    Some(create_resource),
    Some(remove_resource),
    Some(tick)
  );

  logi!("Initialized runtime");

  runtime
}
