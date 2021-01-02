pub mod heap;
pub mod root;

use std::collections::HashMap;

use heap::Heap;

use crate::{
    chunk::{Object, VM},
    system_calls::SystemCalls,
};

pub static STRESS_GC: bool = false;

// TODO: debug logs in gc

pub fn collect_garbage<T: SystemCalls>(vm: &mut VM<T>) {
    mark_roots(vm);
}

pub fn reallocate<T: Sized, S: SystemCalls>(
    vm: &mut VM<S>,
    obj: T,
    old_size: usize,
    new_size: usize,
) -> T {
    if (new_size > old_size) {
        if STRESS_GC {
            collect_garbage(vm);
        }
    }
    todo!();
}

fn mark_roots<T: SystemCalls>(vm: &mut VM<T>) {
    use crate::gc::root::Trace;

    vm.stack.trace();
    vm.globals.trace();
    vm.open_upvalues.trace();
    vm.frames.trace();
    vm.constant_pool.trace();
}
