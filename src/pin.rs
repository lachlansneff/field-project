use core::{ops::Deref, pin::Pin};
use std::ops::DerefMut;
use crate::{Project, ProjectMut};

/// [`Project'] is used to "project" a generic type onto a field of that type that's wrapped.
impl<P> Project for Pin<P> where P: Deref {
    type Base = <P as Deref>::Target;
    type Output<'a, Field: 'a> where Self: 'a = Pin<&'a Field>;

    unsafe fn project<'a, Field: ?Sized>(&'a self, project_field: fn(*const Self::Base) -> *const Field) -> Pin<&'a Field> {
        unsafe {
            self.as_ref().map_unchecked(|base| &*project_field(base))
        }
    }
}

impl<P> ProjectMut for Pin<P> where P: DerefMut {
    type OutputMut<'a, Field: 'a> where Self: 'a = Pin<&'a mut Field>;

    unsafe fn project_mut<'a, Field: ?Sized>(&'a mut self, project_field: fn(*mut Self::Base) -> *mut Field) -> Pin<&'a mut Field> {
        unsafe {
            self.as_mut().map_unchecked_mut(|base| &mut *project_field(base))
        }
    }
}

// impl<T> ProjectMut for Pin<&'_ mut T> {
//     type OutputMut<'a, Field: 'a> where Self: 'a = Pin<&'a mut Field>;

//     fn project_mut<'a, Field: ?Sized>(&'a mut self, project_field: fn(*mut Self::Base) -> *mut Field) -> Pin<&'a mut Field> {
//         unsafe {
//             self.as_mut().map_unchecked_mut(|base| &mut *project_field(base))
//         }
//     }
// }
