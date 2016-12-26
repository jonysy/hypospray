use std::marker::PhantomData;
use super::Cache;

pub struct Graph<M> where M: ?Sized {
    cache: Cache,
    _mod: PhantomData<M>,
}

impl<M> Graph<M> where M: ?Sized {
    
    /// Constructs a new, empty `Graph<M>`.
    pub fn new() -> Graph<M> {
        Graph {
            cache: Cache::new(),
            _mod: PhantomData,
        }
    }
    
    pub fn dep<T>(&self) -> ! where T: ?Sized {
        
        unimplemented!()
    }
}