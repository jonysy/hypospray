pub trait Component<T> where T: ?Sized {
    
    type ComponentImp: 'static + ComponentImp<Component=T>;
    type Scope;
}

/// Component implementation
pub trait ComponentImp: for<'dep> ConstructFn<'dep> {
    
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