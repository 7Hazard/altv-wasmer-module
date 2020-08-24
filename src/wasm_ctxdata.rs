use crate::wasm_memory::Heap;
use wasmer_runtime::{Ctx, Instance};
use std::cell::UnsafeCell;
use crate::wasm_pointers::PointerTable;

pub struct CtxData<'ctx> {
    pub heap: Heap<'ctx>,
    pub ptr_table: PointerTable
}

impl<'ctx> CtxData<'ctx>
{
    // Creates new context data and attaches it to the instance
    pub fn attach(wasminst: &mut Instance)
    {
        wasminst.context_mut().data = Box::into_raw(
            Box::new(
                CtxData {
                    heap: Heap::init(wasminst),
                    ptr_table: PointerTable::new()
                }
            )
        ) as _;
    }

//    pub fn get(ctx: &mut Ctx) -> &'ctx mut CtxData
//    {
//        unsafe {
//            return &mut *(ctx.data as *mut CtxData);
//        }
//    }
//
//    pub fn heap(&mut self) -> &'ctx mut Heap
//    {
//        return &mut self.heap;
//    }
//
//    pub fn ptr_table(&mut self) -> &mut PointerTable {
//        &mut self.ptr_table
//    }
}

pub trait WasmCtxDataGetter {
    fn ctxdata(&self) -> &CtxData;
    fn ctxdata_mut(&mut self) -> &mut CtxData;
}

impl WasmCtxDataGetter for Ctx {
    fn ctxdata(&self) -> &CtxData {
        unsafe { &*(self.data as *const CtxData) }
    }
    fn ctxdata_mut(&mut self) -> &mut CtxData {
        unsafe { &mut *(self.data as *mut CtxData) }
    }
}
