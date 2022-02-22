#![feature(generic_associated_types)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::marker::PhantomData;

use field_project::{Project, ProjectMut, proj};

pub struct VolatileRef<'a, T> {
    ptr: *const T,
    _marker: PhantomData<&'a T>,
}

impl<T> VolatileRef<'_, T> {
    pub unsafe fn new(ptr: *const T) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }
}

impl<T: Copy> VolatileRef<'_, T> {
    pub fn read(&self) -> T {
        unsafe {
            self.ptr.read_volatile()
        }
    }
}

pub struct VolatileMut<'a, T> {
    ptr: *mut T,
    _marker: PhantomData<&'a mut T>,
}

impl<T> VolatileMut<'_, T> {
    pub unsafe fn new(ptr: *mut T) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }
    
    pub fn write(&self, value: T) {
        unsafe {
            self.ptr.write_volatile(value)
        }
    }
}

impl<T: Copy> VolatileMut<'_, T> {
    pub fn read(&self) -> T {
        unsafe {
            self.ptr.read_volatile()
        }
    }
}

impl<'this, T> Project for VolatileRef<'this, T> {
    type Base = T;

    type Output<'a, Field: 'a> where Self: 'a = VolatileRef<'a, Field>;

    unsafe fn project<'a, Field>(&'a self, project_field: fn(*const Self::Base) -> *const Field) -> VolatileRef<'a, Field> {
        unsafe {
            VolatileRef::new(project_field(self.ptr))
        }
    }
}

impl<'this, T> Project for VolatileMut<'this, T> {
    type Base = T;
    type Output<'a, Field: 'a> where Self: 'a = VolatileMut<'a, Field>;

    unsafe fn project<'a, Field>(&'a self, project_field: fn(*const Self::Base) -> *const Field) -> VolatileMut<'a, Field> {
        unsafe {
            VolatileMut::new(project_field(self.ptr) as *mut _)
        }
    }
}

impl<'this, T> ProjectMut for VolatileMut<'this, T> {
    type OutputMut<'a, Field: 'a> where Self: 'a = VolatileMut<'a, Field>;

    unsafe fn project_mut<'a, Field>(&'a mut self, project_field: fn(*mut Self::Base) -> *mut Field) -> VolatileMut<'a, Field> {
        unsafe {
            VolatileMut::new(project_field(self.ptr))
        }
    }
}

#[derive(Default)]
struct Foo {
    reg1: u32,
    reg2: i32,
}

fn main() {
    let mut v = unsafe { VolatileMut::new(&mut Foo::default()) };

    let reg1 = proj!(v.reg1);
    // let reg2 = proj!(mut v.reg2);

    // reg2.write(42);
    
    println!("reg1: {:?}", reg1.read());
    // println!("reg2: {:?}", reg2.read());
}
