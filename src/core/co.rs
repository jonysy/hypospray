use std::ops::{Deref, DerefMut};
use super::{Component, ComponentImp, };

pub struct Co<M, T> where M: ?Sized + Component<T>, T: ?Sized {
    
    component_imp: M::ComponentImp,
}

impl<M, T> Deref for Co<M, T>
    where M: ?Sized + Component<T>, 
          T: ?Sized {
    
    type Target = T;
    
    fn deref(&self) -> &T {

        self.component_imp.__as_ref()
    }
}

impl<M, T> DerefMut for Co<M, T>
    where M: ?Sized + Component<T>, 
          T: ?Sized {
    
    fn deref_mut(&mut self) -> &mut T {

        self.component_imp.__as_mut()
    }
}