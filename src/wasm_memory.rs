
use wasmer_runtime::{Ctx, Memory, Instance};
use wasmer_runtime::units::Pages;
use once_cell::sync::OnceCell;
use std::sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard};
use std::collections::{HashMap, BTreeMap, HashSet};
use std::os::raw::c_void;
use wasmer_runtime::memory::MemoryView;
use crate::wasm_ctxdata::CtxData;
use std::mem::size_of;
use std::ptr::{null_mut, null};
use std::cell::UnsafeCell;

/// Size of one WASM memory page
#[allow(dead_code)]
const PAGE_SIZE: usize = 65536;

struct HeapBlock
{
    start: u32,
    size: u32,
    free: bool,
    next: *mut HeapBlock,
    prev: *mut HeapBlock,
}

pub struct Heap<'ctx>
{
    // The page used for the memory
//    page: Pages,

    /// The WASM instance
    wasm_instance: &'ctx mut Instance,

    /// The start of the heap
    start: *mut HeapBlock,

    /// The total size of the heap
    pub size: u32,

    /// Blocks of allocated memory
//    allocated: BTreeMap<u32, usize>,
    allocations: HashSet<u32>,

    /// The total amount of allocated memory
    pub total_allocated: u32,

    /// Total unallocated memory
    pub total_unallocated: u32,
}

impl<'ctx> Heap<'ctx>
{
    pub fn init(wasminst: &mut Instance) -> Heap
    {
//        let ctx = wasminst.context_mut();
        let mem = wasminst.context().memory(0);
        let memview: MemoryView<u8> = mem.view();

        // Get heap base from exported global
        let heapbase = wasminst.exports().find(
            |(name, _export)| {
                name == "__heap_base"
            }
        ).expect("Could not find __heap_base").1;
        let start = match heapbase {
            wasmer_runtime::Export::Global(g) => g.get().to_u128() as u32,
            _ => {
                loge!("Could not retrieve '__heap_base' as a global export");
                0
            }
        };
        logi!("Heap start: {}", start);
//        assert!(start, "HEAP BASE HAS TO NOT BE 0 (for now)");
        let size = memview.len() as u32 - start;
        logi!("Heap size: {}", size);

//        logi!("Size of HeapBlock: {}", size_of::<HeapBlock>());

        // Initialize the first unallocated block in memory
        unsafe {
            let first = memview[start as usize].as_ptr() as *mut HeapBlock;
            *first = HeapBlock {
                start: start + size_of::<HeapBlock>() as u32,
                size: size - size_of::<HeapBlock>() as u32,
                free: true,
                next: null_mut(),
                prev: null_mut()
            };

            return Heap {
//            page: page,
//            mem: memview,
                wasm_instance: wasminst,
                start: first,
                size: size,
//            allocations: BTreeMap::new(),
                allocations: HashSet::new(),
                total_allocated: 0,
                total_unallocated: (*first).size,
            };
        }
    }

    pub fn view(&self) -> MemoryView<u8>
    {
        return self.wasm_instance.context().memory(0).view();
    }

    /// Inside the closure 'f',
    /// return false to stop iterating
    /// return true to continue iterating
    fn iter<F>(&mut self, mut f: F) where F: FnMut(*mut HeapBlock) -> bool
    {
        let mut cur = self.start;

        unsafe {
            let end = self.start.offset(self.size as isize);
            while cur != end {
                if (cur as u32 > end as u32) {
                    loge!("Heap: Block iterator is misaligned");
                    break;
                }

                // DEBUG
//                logi!("IT BLOCK ptr: {}", (*cur).start-size_of::<HeapBlock>() as u32);
//                logi!("IT BLOCK start: {}", (*cur).start);
//                logi!("IT BLOCK size: {}", (*cur).size);'
                // DEBUG

                if !f(cur) || (*cur).next == null_mut() {
//                    logi!("IT STOPPING ITERATION");
                    break;
                }

//                logi!("IT CONTINUING TO NEXT BLOCK");
                cur = (*cur).next;
            }
        }
    }

    pub fn alloc(&mut self, size: u32) -> u32
    {
//        logi!("malloc: allocating size {}", size);
        unsafe {
            let mut freeblock: *mut HeapBlock = null_mut();

//            logi!("malloc: iterating heapblocks");
            self.iter(|block| {
                // if block size is at least the requested size
                if (*block).free && (*block).size > size {
                    freeblock = block;
//                    logi!("malloc: found free block");
                    return false; // stop iterating
                }
                else {
//                    logi!("malloc: block was not free");
                    return true; // continue iterating
                }
            });

//            logi!("malloc: freeblock start {}", (*freeblock).start);

            if freeblock == null_mut()
            {
                loge!("Heap: Could not allocate memory, insufficient free memory");
                panic!("Heap: Could not allocate memory, insufficient free memory");
                return 0;
            }

            // If the free block is larger than requested, create a new free heap block
            if (*freeblock).size > size
            {
//                logi!("malloc: freeblock was larger than requested: {}, free {}", size,
//                    (*freeblock).size);
                let newfreeblock: *mut HeapBlock = freeblock.offset((size+1) as isize);
                (*newfreeblock) = HeapBlock {
                    start: (*freeblock).start+size+size_of::<HeapBlock>() as u32,
                    size: (*freeblock).size-size-size_of::<HeapBlock>() as u32,
                    free: true,
                    next: (*freeblock).next,
                    prev: freeblock,
                };

//                logi!("malloc: newfreeblock start: {}", (*newfreeblock).start);
//                logi!("malloc: newfreeblock size: {}", (*newfreeblock).size);

                (*freeblock).size = size;
                (*freeblock).next = newfreeblock;
                (*freeblock).free = false;
            }

            self.allocations.insert((*freeblock).start);
            self.total_allocated +=size;
            self.total_unallocated -=size;

//            logi!("malloc: heapblock ptr: {}", (*freeblock).start - size_of::<HeapBlock>() as u32);
//            logi!("malloc: returning {}", (*freeblock).start);

            return (*freeblock).start;
        }
    }

    pub fn dealloc(&mut self, ptr: u32)
    {
        // The ptr to the start of the heap block
//        logi!("free: freeing {}", ptr);
        let blockptr = ptr - size_of::<HeapBlock>() as u32;
//        logi!("free: blockptr: {}", blockptr);

        if !self.allocations.contains(&ptr)
        {
            loge!("Heap: The pointer {:x} is not a valid for deallocation", ptr);
            return;
        }

        self.allocations.remove(&ptr);

        let mem = self.view();

        unsafe {
            let block: *mut HeapBlock = mem[blockptr as usize].as_ptr() as *mut HeapBlock;
//            logi!("free: block size: {}", (*block).size);
            (*block).free = true;

            self.total_unallocated +=(*block).size;
            self.total_allocated -=(*block).size;

            // Check if prev block is free, coalesce
            let prevblock = (*block).prev;
            if prevblock != null_mut() && (*prevblock).free
            {
//                logi!("free: prev block was {}", (*prevblock).start as u32);
//                logi!("free: prevblock was valid and free");
                (*prevblock).size+=(*block).size+size_of::<HeapBlock>() as u32;
                (*prevblock).next = (*block).next;
            }

            // Check if next block is free, coalesce
            let nextblock = (*block).next;
            if nextblock != null_mut() && (*nextblock).free && prevblock != null_mut()
            {
//                logi!("free: next block was {}", (*nextblock).start as u32);
//                logi!("free: prevblock AND nextblock was valid and free");
                // if prev was free
                (*prevblock).size+=(*nextblock).size+size_of::<HeapBlock>() as u32;
                (*prevblock).next = (*nextblock).next;
            }
            else if nextblock != null_mut() && (*nextblock).free
            {
//                logi!("free: next block was {}", (*nextblock).start as u32);
//                logi!("free: nextblock was valid and free");
                // if only next is free
                (*block).size+=(*nextblock).size+size_of::<HeapBlock>() as u32;
                (*block).next = (*nextblock).next;
            }
        }
    }
}
