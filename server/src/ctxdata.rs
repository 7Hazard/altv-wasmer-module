use memory::Heap;
use wasmer_runtime::{Ctx, Instance};
use std::cell::UnsafeCell;

pub struct CtxData<'ctx> {
    heap: Heap<'ctx>,
}

impl<'ctx> CtxData<'ctx>
{
    // Creates new context data and attaches it to the instance
    pub fn attach(wasminst: &mut Instance)
    {
        wasminst.context_mut().data = Box::into_raw(
            Box::new(
                CtxData {
                    heap: Heap::init(wasminst)
                }
            )
        ) as _;
    }

    pub fn get(ctx: &mut Ctx) -> &'ctx mut CtxData
    {
        unsafe {
            return &mut *(ctx.data as *mut CtxData);
        }
    }

    pub fn heap(&mut self) -> &'ctx mut Heap
    {
        return &mut self.heap;
    }
}
