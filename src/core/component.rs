use super::{ComponentImp, };
use super::super::{Prototype, };

pub trait Component<T> where T: ?Sized {
    
    type ComponentImp: ComponentImp<Component=T>;
    type Scope = Prototype;
}