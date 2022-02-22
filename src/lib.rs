#![feature(generic_associated_types)]
#![deny(unsafe_op_in_unsafe_fn)]

mod pin;
mod maybe_uninit;

pub trait Project {
    type Base: ?Sized;
    type Output<'a, Field: 'a> where Self: 'a;
    
    unsafe fn project<'a, Field>(&'a self, project_field: fn(*const Self::Base) -> *const Field) -> Self::Output<'a, Field>;
}

pub trait ProjectMut: Project {
    type OutputMut<'a, Field: 'a> where Self: 'a;
    
    unsafe fn project_mut<'a, Field>(&'a mut self, project_field: fn(*mut Self::Base) -> *mut Field) -> Self::OutputMut<'a, Field>;
}

impl<T> Project for &'_ T where T: Project {
    type Base = T::Base;
    type Output<'a, Field: 'a> where Self: 'a = T::Output<'a, Field>;

    unsafe fn project<'a, Field>(&'a self, project_field: fn(*const Self::Base) -> *const Field) -> Self::Output<'a, Field> {
        unsafe {
            T::project(*self, project_field)
        }
    }
}

impl<T> Project for &'_ mut T where T: Project {
    type Base = T::Base;
    type Output<'a, Field: 'a> where Self: 'a = T::Output<'a, Field>;

    unsafe fn project<'a, Field>(&'a self, project_field: fn(*const Self::Base) -> *const Field) -> Self::Output<'a, Field> {
        unsafe {
            T::project(*self, project_field)
        }
    }
}

impl<T> ProjectMut for &'_ mut T where T: ProjectMut {
    type OutputMut<'a, Field: 'a> where Self: 'a = T::OutputMut<'a, Field>;

    unsafe fn project_mut<'a, Field>(&'a mut self, project_field: fn(*mut Self::Base) -> *mut Field) -> Self::OutputMut<'a, Field> {
        unsafe {
            T::project_mut(*self, project_field)
        }
    }
}

impl<T> Project for Box<T> where T: Project {
    type Base = T::Base;
    type Output<'a, Field: 'a> where Self: 'a = T::Output<'a, Field>;

    unsafe fn project<'a, Field>(&'a self, project_field: fn(*const Self::Base) -> *const Field) -> Self::Output<'a, Field> {
        unsafe {
            T::project(&**self, project_field)
        }
    }
}

/// Use [`proj!`] to project a wrapper struct, like [`std::pin::Pin`], onto a field of the wrapped type.
/// 
/// # Example
/// ```rust
/// # use field_project::proj;
/// struct Foo {
///     a: i32,
///     b: &'static str,
/// }
/// 
/// let foo = Box::pin(Foo { a: 42, b: "hello, world" });
///
/// let a: Pin<_> = proj!(foo.a);
/// let b = proj!(foo.b);
/// ```
#[macro_export]
macro_rules! proj {
    ($input:ident.$field:ident) => {{
        unsafe {
            <_ as $crate::Project>::project(&$input, |base| unsafe { core::ptr::addr_of!((*base).$field) })
        }
    }};
    (mut $input:ident.$field:ident) => {{
        unsafe {
            <_ as $crate::ProjectMut>::project_mut(&mut $input, |base| unsafe { core::ptr::addr_of_mut!((*base).$field) })
        }
    }};
}
