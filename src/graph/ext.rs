use {Co, Component, ComponentImp, Construct, Graph, Prototype, Singleton, };

pub trait Resolve<'imp, T, K> where T: ?Sized {
    
    type CoImp;
    
    fn __resolve(&'imp self) -> Self::CoImp;
}

impl<'imp, M, T> Resolve<'imp, T, Prototype> for Graph<M>
    where M: ?Sized + Component<T>, 
          T: ?Sized,
          Graph<M>: Dependencies<'imp, Co<M, T>> {
    
    type CoImp = Co<M, T>;
    
    fn __resolve(&'imp self) -> Self::CoImp {
        
        Dependencies::<'imp, Co<M, T>>::__dependencies(self)
    }
}

impl<'imp, M, T> Resolve<'imp, T, Singleton> for Graph<M>
    where M: 'static + ?Sized + Component<T>, 
          T: 'static + ?Sized,
          Graph<M>: Dependencies<'imp, &'imp Co<M, T>> {
    
    type CoImp = &'imp Co<M, T>;
    
    fn __resolve(&'imp self) -> Self::CoImp {
        
        Dependencies::<'imp, &'imp Co<M, T>>::__dependencies(self)
    }
}

pub trait Dependencies<'imp, D> {
    
    fn __dependencies(&'imp self) -> D;
}

impl<'imp, M> Dependencies<'imp, ()> for Graph<M> where M: ?Sized {
    
    fn __dependencies(&'imp self) { }
}

impl<'imp, M, T> Dependencies<'imp, Co<M, T>> for Graph<M>
    where M: ?Sized + Component<T, Scope=Prototype>,
          M::ComponentImp: Construct<'imp>,
          T: ?Sized,
          Graph<M>: Dependencies<'imp, <M::ComponentImp as Construct<'imp>>::Dep> {
    
    fn __dependencies(&'imp self) -> Co<M, T> {
        
        Co(
            <M::ComponentImp as Construct>::__construct(
                self.__dependencies()
            )
        )
    }
}

impl<'imp, M, T> Dependencies<'imp, &'imp Co<M, T>> for Graph<M>
    where M: ?Sized + Component<T, Scope=Singleton>,
          M::ComponentImp: 'static + Construct<'imp>,
          T: 'static + ?Sized,
          Graph<M>: for<'dep> Dependencies<'dep, <M::ComponentImp as Construct<'imp>>::Dep>
{
    
    fn __dependencies(&'imp self) -> &'imp Co<M, T> {
        use std::any::{Any, TypeId};
        use std::cell::{Ref, RefCell};
        use std::collections::HashMap;
        use std::ops::Deref;
        
        let id = TypeId::of::<T>();
        
        let to_borrowed = |ca: &'imp RefCell<HashMap<_, _>>| {
            let de: &'imp HashMap<TypeId, Box<_>> = unsafe { &*ca.as_ptr() };
            let ref_boxed: &'imp Box<Any> = de.get(&id).unwrap();
            let co_imp: &'imp M::ComponentImp = ref_boxed.downcast_ref::<M::ComponentImp>().unwrap();
            
            Co::new(co_imp)
        };
        
        if self.cache.borrow().contains_key(&id) {
            return to_borrowed(&self.cache);
        }
        
        let boxed = Box::new(<M::ComponentImp as Construct>::__construct(self.__dependencies()));
        let _ = self.cache.borrow_mut().insert(id, boxed);
        
        to_borrowed(&self.cache)
    }
}