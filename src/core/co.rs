use std::ops::{Deref, DerefMut};
use super::{Component, ComponentImp, };

/// [Ability to automatically derive traits on newtypes (RFC)](https://git.io/vMICm)
/// [repr](https://doc.rust-lang.org/nomicon/repr-rust.html)
#[repr(C)]
pub struct Co<M, T>(pub(super::super) M::ComponentImp)
    where M: ?Sized + Component<T>, 
          T: ?Sized;

impl<M, T> Co<M, T>
    where M: ?Sized + Component<T>, 
          T: ?Sized {
    
    pub(super::super) fn new<'imp>(component_imp: &'imp M::ComponentImp) -> &'imp Co<M, T> {
        
        let ptr = component_imp as *const _ as *const Co<M, T>;
        
        unsafe {
            &*ptr
        }
    }
}

impl<M, T> Clone for Co<M, T>
    where M: ?Sized + Component<T>, 
          M::ComponentImp: Clone,
          T: ?Sized {
              
    fn clone(&self) -> Co<M, T> {
    
        let &Co(ref component_imp) = self;
        Co(component_imp.clone())
    }
       
    /// From [doc.rust-lang][1]: `clone_from` can be overridden to reuse the resources of a to avoid 
    /// unnecessary allocations.
    ///
    /// [1]: https://doc.rust-lang.org/std/clone/trait.Clone.html
    fn clone_from(&mut self, source: &Co<M, T>) {
        
        self.0.clone_from(&source.0);
    }
}

impl<M, T> Copy for Co<M, T>
    where M: ?Sized + Component<T>, 
          M::ComponentImp: Clone + Copy,
          T: ?Sized {
              
}

impl<M, T> Deref for Co<M, T>
    where M: ?Sized + Component<T>, 
          T: ?Sized {
    
    type Target = T;
    
    fn deref(&self) -> &T {

        let &Co(ref component_imp) = self;
        component_imp.__as_ref()
    }
}

impl<M, T> DerefMut for Co<M, T>
    where M: ?Sized + Component<T>, 
          T: ?Sized {
    
    fn deref_mut(&mut self) -> &mut T {

        let &mut Co(ref mut component_imp) = self;
        component_imp.__as_mut()
    }
}