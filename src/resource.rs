
use altv_capi::*;

pub unsafe extern "C" fn start(resource: *mut alt_IResource) -> bool
{

    true
}

pub unsafe extern "C" fn stop(resource: *mut alt_IResource) -> bool
{

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
