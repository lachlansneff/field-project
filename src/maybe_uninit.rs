use core::mem::MaybeUninit;
use crate::{Project, ProjectMut};

impl<T> Project for MaybeUninit<T> {
    type Base = T;
    type Output<'a, Field: 'a> where Self: 'a = &'a MaybeUninit<Field>;

    unsafe fn project<'a, Field>(&'a self, project_field: fn(*const Self::Base) -> *const Field) -> &'a MaybeUninit<Field> {
        unsafe {
            &*(project_field(self.as_ptr()) as *const MaybeUninit<_>)
        }
    }
}

impl<T> ProjectMut for MaybeUninit<T> {
    type OutputMut<'a, Field: 'a> where Self: 'a = &'a mut MaybeUninit<Field>;

    unsafe fn project_mut<'a, Field>(&'a mut self, project_field: fn(*mut Self::Base) -> *mut Field) -> Self::OutputMut<'a, Field> {
        unsafe {
            &mut *(project_field(self.as_mut_ptr()) as *mut MaybeUninit<_>)
        }
    }
}
