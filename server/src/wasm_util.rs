

use std::sync::{Mutex, Arc, RwLock, RwLockReadGuard};
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;

// HIGH RISK FOR DEADLOCK
// HAPPENS WHEN READING THEN WRITING
static PTR_BY_ID: OnceCell<RwLock<HashMap<u32, u64>>> = OnceCell::new();
static ID_BY_PTR: OnceCell<RwLock<HashMap<u64, u32>>> = OnceCell::new();

pub fn get_ptr(id: u32) -> u64
{
    let ptrs = PTR_BY_ID.get_or_init(||
        RwLock::new(HashMap::new())
    ).read().expect("Could not read to PTR_BY_ID");

    let ptr = ptrs.get(&id);
    if ptr.is_none()
    {
        return 0;
    }
    else {
        return *ptr.unwrap();
    }
}

pub fn get_id_by_ptr<T>(ptr: *mut T) -> u32
{
    return get_id(ptr as u64);
}

pub fn get_id(ptr: u64) -> u32
{
    // HIGH RISK FOR DEADLOCK
    // HAPPENS WHEN READING THEN WRITING

    let mut id = 0;

    {
        let ids = ID_BY_PTR.get_or_init(||
            RwLock::new(HashMap::new())
        ).read().expect("Could not read ID_BY_PTR");

        // get id if already cached, return it
        let bid = ids.get(&ptr);
        if !bid.is_none() {
            id = *bid.unwrap();
        }
    }

    if id == 0
    {
        let mut ids = ID_BY_PTR.get_or_init(||
            RwLock::new(HashMap::new())
        ).write().expect("Could not write to ID_BY_PTR");

        // create a new id for the ptr
        let new_id = ids.len() as u32 +1;

        let mut ptrs = PTR_BY_ID.get_or_init(||
            RwLock::new(HashMap::new())
        ).write().expect("Could not write to PTR_BY_ID");

        // insert new id
        ids.insert(ptr, new_id);
        ptrs.insert(new_id, ptr);

        return new_id;
    }

    return id;
}
