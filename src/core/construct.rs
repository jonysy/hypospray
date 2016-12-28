/// Construct
pub trait Construct<'dep> {

    /// Dependenc(y/ies)
    ///
    /// `ComponentImp`s can have dependencies of their own.
    ///
    /// ## RFC(s)
    ///
    /// * [Associated type constructors #1598](https://git.io/vw10w)
    type Dep;
    
    /// Constructs the `ComponentImp`
    ///
    /// # Arguments
    ///
    /// * `dep` - Dependencies
    fn __construct(dep: Self::Dep) -> Self;
}