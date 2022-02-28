use core::mem::MaybeUninit;
use crate::{Project, ProjectMut};

impl<T, Field> Project<Field> for MaybeUninit<T> {
    type Base = T;
    type Output<'a> where Self: 'a, Field: 'a = &'a MaybeUninit<Field>;

    unsafe fn project<'a>(&'a self, project_field: fn(*const Self::Base) -> *const Field) -> &'a MaybeUninit<Field> {
        unsafe {
            &*(project_field(self.as_ptr()) as *const MaybeUninit<_>)
        }
    }
}

impl<T, Field> ProjectMut<Field> for MaybeUninit<T> {
    type OutputMut<'a> where Self: 'a, Field: 'a = &'a mut MaybeUninit<Field>;

    unsafe fn project_mut<'a>(&'a mut self, project_field: fn(*mut Self::Base) -> *mut Field) -> &'a mut MaybeUninit<Field> {
        unsafe {
            &mut *(project_field(self.as_mut_ptr()) as *mut MaybeUninit<_>)
        }
    }
}
