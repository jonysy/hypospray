use {Co, Component, Construct, Graph, Prototype, Singleton, };

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
          M::ComponentImp: 'static + Send + Construct<'imp>,
          T: 'static + ?Sized,
          Graph<M>: for<'dep> Dependencies<'dep, <M::ComponentImp as Construct<'imp>>::Dep> {
    
    fn __dependencies(&'imp self) -> &'imp Co<M, T> {
        use std::any::{Any, TypeId};
        use std::collections::HashMap;
        use std::mem;
        use std::sync::MutexGuard;
        use super::{Arc, Container, Shared};
        
        let id = TypeId::of::<T>();
        
        let to_borrowed = |arc: &'imp Arc<Shared>| -> &'imp Co<M, T> {
            let re: &'imp Shared = arc.as_ref();
            let sh: MutexGuard<'imp, _> = re.lock().expect("todo");
            let hash: &Container = &sh;
            let hash = unsafe { mem::transmute::<&Container, &'imp Container>(hash) };
            let dyn: &'imp _ = hash.get(&id).expect("Something unexpected went wrong!");
            
            let co_imp: &'imp M::ComponentImp = dyn
                .downcast_ref::<M::ComponentImp>()
                .expect("Something unexpected went wrong!");
            
            Co::new(co_imp)
        };
        
        let contains = {
            let sh = self.0.lock().expect("todo");
            sh.contains_key(&id)
        };
        
        if contains {
            return to_borrowed(&self.0);
        }
        
        let boxed = Box::new(<M::ComponentImp as Construct>::__construct(self.__dependencies()));
        
        {
            let mut sh = self.0.lock().expect("todo");
            let _ = sh.insert(id, boxed);
        }
        
        to_borrowed(&self.0)
    }
}

macro_rules! deps {
    ($($dep:ident),*) => {
        
        impl<'imp, M, $($dep,)*> Dependencies<'imp, ($($dep,)*)> for Graph<M>
            where M: ?Sized,
                  $(Graph<M>: for<'dep> Dependencies<'dep, $dep>,)* {
                      
            fn __dependencies(&'imp self) -> ($($dep,)*) {
                
                ($(
                    Dependencies::<'imp, $dep>::__dependencies(self),
                )*)
            }
        }
    }
}

deps!(D1, D2);
deps!(D1, D2, D3);
deps!(D1, D2, D3, D4);
deps!(D1, D2, D3, D4, D5);