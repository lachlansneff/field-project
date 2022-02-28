#![feature(generic_associated_types)]
#![deny(unsafe_op_in_unsafe_fn)]

mod pin;
mod maybe_uninit;

pub trait Project<Field: ?Sized> {
    type Base: ?Sized;
    type Output<'a> where Self: 'a, Field: 'a;
    
    unsafe fn project<'a>(&'a self, project_field: fn(*const Self::Base) -> *const Field) -> Self::Output<'a>;
}

pub trait ProjectMut<Field: ?Sized>: Project<Field> {
    type OutputMut<'a> where Self: 'a, Field: 'a;
    
    unsafe fn project_mut<'a>(&'a mut self, project_field: fn(*mut Self::Base) -> *mut Field) -> Self::OutputMut<'a>;
}

impl<T, Field: ?Sized> Project<Field> for &'_ T where T: Project<Field> {
    type Base = T::Base;
    type Output<'a> where Self: 'a, Field: 'a = T::Output<'a>;

    unsafe fn project<'a>(&'a self, project_field: fn(*const Self::Base) -> *const Field) -> Self::Output<'a> {
        unsafe {
            T::project(*self, project_field)
        }
    }
}

impl<T, Field: ?Sized> Project<Field> for &'_ mut T where T: Project<Field> {
    type Base = T::Base;
    type Output<'a> where Self: 'a, Field: 'a = T::Output<'a>;

    unsafe fn project<'a>(&'a self, project_field: fn(*const Self::Base) -> *const Field) -> Self::Output<'a> {
        unsafe {
            T::project(*self, project_field)
        }
    }
}

impl<T, Field: ?Sized> ProjectMut<Field> for &'_ mut T where T: ProjectMut<Field> {
    type OutputMut<'a> where Self: 'a, Field: 'a = T::OutputMut<'a>;

    unsafe fn project_mut<'a>(&'a mut self, project_field: fn(*mut Self::Base) -> *mut Field) -> Self::OutputMut<'a> {
        unsafe {
            T::project_mut(*self, project_field)
        }
    }
}

impl<T, Field: ?Sized> Project<Field> for Box<T> where T: Project<Field> {
    type Base = T::Base;
    type Output<'a> where Self: 'a, Field: 'a = T::Output<'a>;

    unsafe fn project<'a>(&'a self, project_field: fn(*const Self::Base) -> *const Field) -> Self::Output<'a> {
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
            <_ as $crate::Project<_>>::project(&$input, |base| unsafe { core::ptr::addr_of!((*base).$field) })
        }
    }};
    (mut $input:ident.$field:ident) => {{
        unsafe {
            <_ as $crate::ProjectMut<_>>::project_mut(&mut $input, |base| unsafe { core::ptr::addr_of_mut!((*base).$field) })
        }
    }};
}
