pub use self::ext::Resolve;
mod ext;

use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use super::{Co, Component, };

pub struct Graph<M> where M: ?Sized { cache: Cache, _mod: PhantomData<M> }

type Cache = RefCell<HashMap<TypeId, Box<Any>>>;

impl<M> Graph<M> where M: ?Sized {
    
    /// Constructs a new, empty `Graph<M>`.
    pub fn new() -> Graph<M> {
        
        Graph {
            cache: RefCell::new(HashMap::new()), 
            _mod: PhantomData
        }
    }
    
    pub fn dep<T>(&self) -> Co<M, T>
        where M: Component<T>, 
              T: 'static + ?Sized, 
              Graph<M>: for<'imp> Resolve<'imp, T, M::Scope, CoImp = Co<M, T>> {
        
        self.__resolve()
    }

    /// Force a component to be created eagerly.
    ///
    /// By default, all components are created when needed (i.e., lazily). `eager` allows you to 
    /// create a component at the beginning of an application.
    ///
    /// ## Example
    ///
    /// ```rust
    /// // ..
    /// ```
    pub fn eager<T>(&self) -> ! where T: ?Sized {

        unimplemented!()
    }
}