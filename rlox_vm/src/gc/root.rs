use std::ptr::NonNull;
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    ops::{Deref, DerefMut},
    todo,
};

use crate::chunk::{CallFrame, FuncSpec, Object, UpValue, UpValueWrap};

use super::heap::Heap;
use std::fmt::Debug;

pub trait MemoryBlob {
    fn mark(&mut self);
    fn un_mark(&mut self);
    fn get_is_marked(&self) -> bool;
}

#[derive(Clone)]
pub struct Blob<T: Trace + Sized + CustomClone + Debug> {
    pub data: T,
    is_marked: Cell<bool>,
}

impl<T: Trace + Sized + CustomClone + Debug> Deref for Blob<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Trace + Sized + CustomClone + Debug> DerefMut for Blob<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
impl<T: Trace + CustomClone + Debug> MemoryBlob for Blob<T> {
    fn mark(&mut self) {
        self.is_marked.replace(true);
    }
    fn un_mark(&mut self) {
        self.is_marked.replace(false);
    }
    fn get_is_marked(&self) -> bool {
        return self.is_marked.get();
    }
}

impl<T: Trace + Sized + CustomClone + Debug> Blob<T> {
    pub fn new(val: T) -> Self {
        Blob {
            data: val,
            is_marked: Cell::from(false),
        }
    }
}

#[derive(Clone)]
pub struct Root<T: Trace + Sized + CustomClone + Debug> {
    pub(crate) data: NonNull<Blob<T>>,
}

impl<T: Default + Trace + Sized + CustomClone + Debug> Default for Root<T> {
    fn default() -> Self {
        todo!()
    }
}

impl<T: Trace + Sized + CustomClone + Debug> Deref for Root<T> {
    type Target = Blob<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { &self.data.as_ref() }
    }
}

impl<T: Trace + Sized + CustomClone + Debug> DerefMut for Root<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.data.as_mut() }
    }
}
pub struct UniqueRoot<T: Trace + Sized + CustomClone + Debug> {
    pub(crate) data: NonNull<Blob<T>>,
}

impl<T: Default + Trace + Sized + CustomClone + Debug> Default for UniqueRoot<T> {
    fn default() -> Self {
        todo!()
    }
}

impl<T: Trace + Sized + CustomClone + Debug> Deref for UniqueRoot<T> {
    type Target = Blob<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { &self.data.as_ref() }
    }
}

impl<T: Trace + Sized + CustomClone + Debug> DerefMut for UniqueRoot<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.data.as_mut() }
    }
}

pub trait Trace {
    fn trace(&mut self);
}

impl Trace for Object {
    fn trace(&mut self) {
        match self {
            Object::Str(_) => {}
            Object::Num(_) => {}
            Object::Bool(_) => {}
            Object::Nil => {}
            // Object::Function(_) => {}
            Object::NativeFunction(_) => {}
            Object::Closure(val) => val.trace(),
            Object::ClassDef(val) => {
                todo!()
            }
            Object::InstanceDef(val) => {
                todo!()
            }
            Object::InstanceBindDef(val) => {
                todo!()
            }
        }
    }
}

impl<T: Trace + Sized + CustomClone + Debug> Trace for Root<T> {
    fn trace(&mut self) {
        if self.get_is_marked() {
            return;
        }
        unsafe { self.data.as_mut() }.data.trace();
        self.mark();
    }
}

impl<T: Trace + Sized + CustomClone + Debug> Trace for UniqueRoot<T> {
    fn trace(&mut self) {
        if self.get_is_marked() {
            return;
        }
        unsafe { self.data.as_mut() }.data.trace();
        self.mark();
    }
}

impl Trace for FuncSpec {
    fn trace(&mut self) {
        self.upvalues_ref.trace();
    }
}

impl Trace for CallFrame {
    fn trace(&mut self) {
        self.func.trace();
    }
}

impl Trace for UpValueWrap {
    fn trace(&mut self) {
        self.0.trace();
    }
}

impl Trace for UpValue {
    fn trace(&mut self) {
        match self {
            UpValue::Open(_) => {}
            UpValue::Closed(val) => val.trace(),
        }
    }
}

impl<T> Trace for RefCell<T>
where
    T: Trace,
{
    fn trace(&mut self) {
        self.borrow_mut().trace();
    }
}

impl<T> Trace for Vec<T>
where
    T: Trace,
{
    fn trace(&mut self) {
        for val in self {
            val.trace()
        }
    }
}

impl<T> Trace for HashMap<String, T>
where
    T: Trace,
{
    fn trace(&mut self) {
        for (_, val) in self {
            val.trace()
        }
    }
}

pub trait CustomClone {
    fn clone(&self, gc: &Heap) -> Self;
}

impl<T> CustomClone for Root<T>
where
    T: Trace + Sized + CustomClone + Debug + 'static,
{
    fn clone(&self, gc: &Heap) -> Self {
        gc.clone_root(self)
    }
}

impl<T> CustomClone for UniqueRoot<T>
where
    T: Trace + Sized + CustomClone + Debug + 'static,
{
    fn clone(&self, gc: &Heap) -> Self {
        gc.clone_unique_root(self)
    }
}

impl<T> CustomClone for RefCell<T>
where
    T: CustomClone,
{
    fn clone(&self, gc: &Heap) -> Self {
        RefCell::new(self.borrow().clone(gc))
    }
}

impl<T> CustomClone for Option<T>
where
    T: CustomClone,
{
    fn clone(&self, gc: &Heap) -> Self {
        match self {
            None => None,
            Some(val) => Some(val.clone(gc)),
        }
    }
}

impl<T> CustomClone for Vec<T>
where
    T: CustomClone,
{
    fn clone(&self, gc: &Heap) -> Self {
        let mut res = vec![];
        for val in self {
            res.push(val.clone(gc))
        }
        res
    }
}

impl<T> CustomClone for HashMap<String, T>
where
    T: CustomClone,
{
    fn clone(&self, gc: &Heap) -> Self {
        let mut res = HashMap::new();
        for (k, v) in self {
            res.insert(k.clone(), v.clone(gc));
        }
        res
    }
}

pub trait CustomVecOps {
    type R;

    fn to_vec(&self, gc: &mut Heap) -> Self::R;
}

impl<T> CustomVecOps for [T]
where
    T: CustomClone,
{
    type R = Vec<T>;

    fn to_vec(&self, gc: &mut Heap) -> Self::R {
        let mut res = vec![];
        for it in self {
            res.push(it.clone(gc))
        }
        res
    }
}

impl<T: Trace + Sized + CustomClone + Debug> Debug for Root<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Root<{:?}>", unsafe {
            &self.data.as_ref().data
        }))
    }
}

impl<T: Trace + Sized + CustomClone + Debug> Debug for UniqueRoot<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("UniqueRoot<{:?}>", unsafe {
            &self.data.as_ref().data
        }))
    }
}

mod tests {
    use std::cell::RefCell;

    use super::super::heap::*;
    use super::*;

    #[derive(Debug)]
    struct Data {
        inner: u32,
    }

    impl CustomClone for Data {
        fn clone(&self, gc: &Heap) -> Self {
            todo!()
        }
    }

    impl Data {
        pub fn new(inner: u32) -> Self {
            Data { inner }
        }
        pub fn set(&mut self, inner: u32) {
            self.inner = inner;
        }
        pub fn get(&self) -> &u32 {
            &self.inner
        }
    }

    impl Trace for Data {
        fn trace(&mut self) {
            todo!()
        }
    }

    #[test]
    fn test_root() {
        let mut gc = Heap::new();
        let root = gc.get_root(RefCell::new(Data::new(7)));
        assert_eq!(root.borrow().get(), &7);
    }

    #[test]
    fn test_root_1() {
        let mut gc = Heap::new();
        let root = gc.get_root(RefCell::new(Data::new(7)));
        root.borrow_mut().set(9);
        assert_eq!(root.borrow().get(), &9);
    }

    #[test]
    fn test_root_2() {
        let mut gc = Heap::new();
        let root = gc.get_root(RefCell::new(Data::new(7)));
        let root1 = gc.clone_root(&root);
        root1.borrow_mut().set(9);
        assert_eq!(root.borrow().get(), &9);
    }

    #[test]
    fn test_unique_root_2() {
        let mut gc = Heap::new();
        let root = gc.get_unique_root(RefCell::new(Data::new(7)));
        let root1 = gc.clone_unique_root(&root);
        root1.borrow_mut().set(9);
        assert_eq!(root.borrow().get(), &7);
    }
    // #[test]
    // fn test_root() {
    //     let mut gc = Heap::new();
    //     let root  = gc.get_root(RefCell::new(Data::new(7)));
    //     assert_eq!(root.borrow().get(), &7);
    // }
}
