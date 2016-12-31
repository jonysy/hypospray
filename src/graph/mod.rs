pub use self::ext::Resolve;
mod ext;

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use super::{Component, Singleton, };

pub struct Graph<M>(Arc<Shared>, PhantomData<M>) where M: ?Sized;

type Dyn = Box<Any + 'static + Send>;
type Container = HashMap<TypeId, Dyn>;
type Shared = Mutex<Container>;

impl<M> Graph<M> where M: ?Sized {
    
    /// Constructs a new, empty `Graph<M>`.
    pub fn new() -> Graph<M> {
        
        let hash_map = HashMap::new();
        let mutex = Mutex::new(hash_map);
        let arc = Arc::new(mutex);
        
        return Graph(arc, PhantomData);
    }
    
    pub fn dep<T>(&self) -> <Graph<M> as Resolve<T, M::Scope>>::CoImp
        where M: Component<T>, 
              T: 'static + ?Sized, 
              Graph<M>: for<'imp> Resolve<'imp, T, M::Scope> {
        
        self.__resolve()
    }

//    /// Force a component to be created eagerly.
//    ///
//    /// By default, all components are created when needed (i.e., lazily). `eager` allows you to 
//    /// create a component at the beginning of an application.
//    ///
//    /// ## Example
//    ///
//    /// ```rust
//    /// // ..
//    /// ```
//    pub fn eager<T>(&self) -> !
//        where M: Component<T, Scope=Singleton>, 
//              T: 'static + ?Sized, Graph<M>: for<'imp> Resolve<'imp, T, M::Scope> {
//
//        unimplemented!()
//    }
}

impl<M> Clone for Graph<M> where M: ?Sized {
    
    fn clone(&self) -> Self {
        
        let &Graph(ref arc, ..) = self;
        
        Graph(arc.clone(), PhantomData)
    }
}