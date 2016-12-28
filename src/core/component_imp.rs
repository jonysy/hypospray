/// Component implementation
pub trait ComponentImp {
    
    type Component: ?Sized;

    fn __as_ref(&self) -> &Self::Component;
    fn __as_mut(&mut self) -> &mut Self::Component;
}