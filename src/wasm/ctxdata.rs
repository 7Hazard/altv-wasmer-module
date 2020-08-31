use wasmer_runtime::{Instance, Memory};
use std::cell::UnsafeCell;
use crate::wasm::memory::Heap;
use crate::wasm::pointers::PointerTable;
use crate::WasmResource;

pub struct CtxData<'ctx> {
    pub instance: &'ctx Instance,
    pub heap: Option<Heap<'ctx>>,
    pub ptr_table: PointerTable
}

impl<'ctx> CtxData<'ctx>
{
    // Creates new context data
    pub fn new(instance: &Instance) -> CtxData
    {
        CtxData {
            instance,
            heap: Heap::init(instance),
            ptr_table: PointerTable::new()
        }
    }
}
