/// Construct
pub trait Construct<'dep> {

    // TODO (DEVELOPMENT.md)
    //
    // ## RFC(s)
    //
    // * [Associated type constructors #1598](https://git.io/vw10w)
    
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