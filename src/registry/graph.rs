use std::marker::PhantomData;
use super::Cache;

pub struct Graph<M> where M: ?Sized { cache: Cache, _mod: PhantomData<M> }

impl<M> Graph<M> where M: ?Sized {
    
    /// Constructs a new, empty `Graph<M>`.
    pub fn new() -> Graph<M> {

        Graph { cache: Cache::new(), _mod: PhantomData }
    }
    
    pub fn dep<T>(&self) -> ! where T: ?Sized {
        
        unimplemented!()
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