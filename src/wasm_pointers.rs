use std::sync::{Mutex, Arc, RwLock, RwLockReadGuard};
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use wasmer_runtime::{Ctx, WasmPtr, Array};
use crate::wasm_ctxdata::WasmCtxDataGetter;

/// Current has memory leaks cause entries are not removed  
/// HIGH RISK FOR DEADLOCK  
/// HAPPENS WHEN READING THEN WRITING  
pub struct PointerTable {
    ptr_by_id: RwLock<HashMap<u32, u64>>,
    id_by_ptr: RwLock<HashMap<u64, u32>>,
}

impl PointerTable {
    pub fn new() -> Self {
        Self{
            ptr_by_id: RwLock::new(HashMap::new()),
            id_by_ptr: RwLock::new(HashMap::new()),
        }
    }

    pub fn get_ptr<T>(&self, id: u32) -> Option<*mut T>
    {
        let ptrs = self.ptr_by_id.read().expect("Could not read to PTR_BY_ID");

        match ptrs.get(&id) {
            None => {
                loge!("Pointer with ID {} was not found in the pointer table", id);
                None
            },
            Some(val) => Some(*val as *mut T)
        }
    }

    pub fn get_id_by_ptr<T>(&mut self, ptr: *mut T) -> u32
    {
        return self.get_id(ptr as u64);
    }

    pub fn get_id(&self, ptr: u64) -> u32
    {
        // HIGH RISK FOR DEADLOCK
        // HAPPENS WHEN READING THEN WRITING

        let mut id = 0;

        {
            let ids = self.id_by_ptr.read().expect("Could not read ID_BY_PTR");

            // get id if already cached, return it
            let bid = ids.get(&ptr);
            if !bid.is_none() {
                id = *bid.unwrap();
            }
        }

        if id == 0
        {
            let mut ids = self.id_by_ptr.write().expect("Could not write to ID_BY_PTR");

            // create a new id for the ptr
            let new_id = ids.len() as u32 +1;

            let mut ptrs = self.ptr_by_id.write().expect("Could not write to PTR_BY_ID");

            // insert new id
            ids.insert(ptr, new_id);
            ptrs.insert(new_id, ptr);

            return new_id;
        }

        return id;
    }
}

pub trait WasmPtrExtentions<T> {
    //    fn ptr_by_id(&self, ctx: &Ctx) -> Option<*mut T>;
    fn ptr_by_id_err<Z>(&self, ctx: &Ctx, err_msg: &String) -> *mut Z;
    fn mem_err(&self, ctx: &Ctx, err_msg: &String) -> *mut T;
}

impl <T: Copy + wasmer_runtime::types::ValueType> WasmPtrExtentions<T> for WasmPtr<T>
{
    //    fn ptr_by_id(&self, ctx: &Ctx) -> Option<*mut T>
//    {
//        ctx.ctxdata().ptr_table.get_ptr(self.offset())
//    }

    fn ptr_by_id_err<Z>(&self, ctx: &Ctx, err_msg: &String) -> *mut Z
    {
        ctx.ctxdata().ptr_table.get_ptr(self.offset()).expect(err_msg.as_str())
    }

    // TODO: validate self.offset
    fn mem_err(&self, ctx: &Ctx, err_msg: &String) -> *mut T {
        ctx.ctxdata().heap.view()[self.offset() as usize].as_ptr() as *mut T
    }
}

pub trait WasmPtrArrayExtentions<T> {
    //    fn ptr_by_id(&self, ctx: &Ctx) -> Option<*mut T>;
//    fn ptr_by_id_err(&self, ctx: &Ctx, err_msg: &String) -> *mut T;
    fn mem_index_err(&self, ctx: &Ctx, index: usize, err_msg: &String) -> *mut T;
    fn mem_err(&self, ctx: &Ctx, err_msg: &String) -> *mut T;
}

impl <T: Copy + wasmer_runtime::types::ValueType> WasmPtrArrayExtentions<T> for WasmPtr<T, Array>
{
//    fn ptr_by_id(&self, ctx: &Ctx) -> Option<*mut T>
//    {
//        ctx.ctxdata().ptr_table.get_ptr(self.offset())
//    }

//    fn ptr_by_id_err(&self, ctx: &Ctx, err_msg: &String) -> *mut T
//    {
//        ctx.ctxdata().ptr_table.get_ptr(self.offset()).expect(err_msg.as_str())
//    }

    // TODO: validate self.offset
    fn mem_index_err(&self, ctx: &Ctx, index: usize, err_msg: &String) -> *mut T {
        ctx.memory(0).view()[self.offset() as usize + std::mem::size_of::<T>()*index].as_ptr() as *mut T
    }
    fn mem_err(&self, ctx: &Ctx, err_msg: &String) -> *mut T {
        self.mem_index_err(ctx, 0, err_msg)
    }
}
