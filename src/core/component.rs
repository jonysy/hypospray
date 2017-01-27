use super::ComponentImp;

pub trait Component<T> where T: ?Sized {
    
    type ComponentImp: ComponentImp<Component=T>;
    type Scope;
}