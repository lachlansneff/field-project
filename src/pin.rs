use core::{ops::Deref, pin::Pin};
use std::ops::DerefMut;
use crate::{Project, ProjectMut};

/// [`Project'] is used to "project" a generic type onto a field of that type that's wrapped.
impl<P, Field: ?Sized> Project<Field> for Pin<P> where P: Deref {
    type Base = <P as Deref>::Target;
    type Output<'a> where Self: 'a, Field: 'a = Pin<&'a Field>;

    unsafe fn project<'a>(&'a self, project_field: fn(*const Self::Base) -> *const Field) -> Pin<&'a Field> {
        unsafe {
            self.as_ref().map_unchecked(|base| &*project_field(base))
        }
    }
}

impl<P, Field: ?Sized> ProjectMut<Field> for Pin<P> where P: DerefMut, Field: Unpin {
    type OutputMut<'a> where Self: 'a, Field: 'a = Pin<&'a mut Field>;

    unsafe fn project_mut<'a>(&'a mut self, project_field: fn(*mut Self::Base) -> *mut Field) -> Pin<&'a mut Field> {
        unsafe {
            self.as_mut().map_unchecked_mut(|base| &mut *project_field(base))
        }
    }
}
