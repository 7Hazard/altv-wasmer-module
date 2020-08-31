
use wasmer_runtime::{Ctx, Memory, Instance};
use crate::wasm::ctxdata::CtxData;
use crate::wasm::memory::Heap;
use wasmer_runtime::memory::MemoryView;
use crate::WasmResource;

pub trait WasmCtxExtentions {
    fn data(&self) -> &CtxData;
    fn data_mut(&mut self) -> &mut CtxData;
    fn instance(&self) -> &Instance;
    fn memview(&self) -> MemoryView<u8>;
    fn heap(&self) -> &Heap;
    fn heap_mut(&mut self) -> &mut Heap;
    fn alloc(&mut self, size: u32) -> u32;
    fn dealloc(&mut self, pointer: u32);
}

impl WasmCtxExtentions for Ctx {
    fn data(&self) -> &CtxData {
        unsafe { &*(self.data as *const CtxData) }
    }
    fn data_mut(&mut self) -> &mut CtxData {
        unsafe { &mut *(self.data as *mut CtxData) }
    }
    fn instance(&self) -> &Instance {
        self.data().instance
    }
    fn memview(&self) -> MemoryView<u8> {
        self.memory(0).view()
    }
    fn heap(&self) -> &Heap {
        self.data().heap.as_ref().expect("heap is unavailable")
    }
    fn heap_mut(&mut self) -> &mut Heap {
        self.data_mut().heap.as_mut().expect("heap is unavailable")
    }
    fn alloc(&mut self, size: u32) -> u32 {
        self.heap_mut().alloc(size)
    }
    fn dealloc(&mut self, pointer: u32) {
        self.heap_mut().dealloc(pointer)
    }
}
