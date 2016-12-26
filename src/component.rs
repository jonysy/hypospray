pub trait Component<T> where T: ?Sized {
    
    type ComponentImp: for<'dep> ComponentImp<'dep, Component=T>;
    type Scope;
}

pub trait ComponentImp<'dep>
    // [Can't relate two associated types](https://github.com/rust-lang/rust/issues/24092)
    where Self: AsRef<<Self as ComponentImp<'dep>>::Component>,
          Self: AsMut<<Self as ComponentImp<'dep>>::Component>,
{
    
    type Component: ?Sized;
    type Dep;
    
    /// Constructs the `ComponentImp`
    ///
    /// # Arguments
    ///
    /// * `dep` - Dependencies
    fn __construct(dep: Self::Dep) -> Self;
}