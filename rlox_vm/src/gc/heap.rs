use std::{cell::{Cell, RefCell}, ptr::NonNull};

use crate::{chunk::VM, system_calls::SystemCalls};

use super::{mark_roots, root::{Blob, CustomClone, MemoryBlob, Root, Trace, UniqueRoot}};
pub struct Heap {
    mem: RefCell<Vec<Box<dyn MemoryBlob>>>
}

impl Heap {
    pub fn new() -> Self {
        Heap {
            mem: RefCell::new(vec![])
        }
    }
    fn allocate<T: 'static + Trace + CustomClone>(&self, val: T) -> NonNull<Blob<T>>{
        let mut alloc: Box<Blob<T>> = Box::new(Blob::new(val));
        let ptr = unsafe {NonNull::new_unchecked(&mut *alloc)};
        self.mem.borrow_mut().push(alloc);
        ptr
    } 
    pub fn get_root<T: 'static + Trace + CustomClone>(&self, val: T) -> Root<T> {
        Root{
            data: self.allocate(val)
        }
    }
    pub fn get_unique_root<T: 'static + Trace + CustomClone>(&self, val: T) -> UniqueRoot<T> {
        UniqueRoot{
            data: self.allocate(val)
        }
    }
    pub fn clone_root<T: 'static + Trace + CustomClone>(&self, val: &Root<T>) -> Root<T> {
        Root {
            data: val.data
        }
    }
    pub fn clone_unique_root<T: 'static + Trace + CustomClone>(&self, val: &UniqueRoot<T>) -> UniqueRoot<T> {
        UniqueRoot{ 
            data: self.allocate(unsafe{val.data.as_ref().data.clone(self)})
        }
    }
    pub fn collect_free<S: SystemCalls>(&self, vm: &mut VM<S>) {
        mark_roots(vm);
        println!("GC before: {}", self.get_heap_size());
        self.mem.borrow_mut().retain(|val| {
            val.get_is_marked()
        });
        println!("GC after: {}", self.get_heap_size());
    }
    pub fn get_heap_size(&self) -> usize {
        self.mem.borrow().len()
    }
}