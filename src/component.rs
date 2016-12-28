use std::ops::{Deref, DerefMut};

pub struct Co<M, T> where M: ?Sized + Component<T>, T: ?Sized {
    
    component_imp: M::ComponentImp,
}

impl<M, T> Deref for Co<M, T> where M: ?Sized + Component<T>, T: ?Sized {
    
    type Target = T;
    
    fn deref(&self) -> &T {
        unimplemented!()
    }
}

impl<M, T> DerefMut for Co<M, T> where M: ?Sized + Component<T>, T: ?Sized {
    
    fn deref_mut(&mut self) -> &mut T {
        unimplemented!()
    }
}

pub trait Component<T> where T: ?Sized {
    
    type ComponentImp: 'static + AsRef<T> + AsMut<T> + ComponentImp<Component=T> + for<'dep> ConstructFn<'dep>;
    type Scope;
}

/// Component implementation
pub trait ComponentImp {
    
    type Component: ?Sized;
}

/// Construct
///
/// ## RFC(s)
///
/// * [Associated type constructors #1598](https://git.io/vw10w)
pub trait ConstructFn<'dep> {

    /// Dependenc(y/ies)
    ///
    /// `ComponentImp`s can have dependencies of their own.
    type Dep;
    
    /// Constructs the `ComponentImp`
    ///
    /// # Arguments
    ///
    /// * `dep` - Dependencies
    fn __construct(dep: Self::Dep) -> Self;
}