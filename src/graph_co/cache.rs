use std::cell::RefCell;
use std::collections::HashMap;
use super::{ComponentKey, Dyn};

pub struct Cache {
    ref_cell: RefCell<HashMap<ComponentKey, Dyn>>,
}

impl Cache {
    
    pub fn new() -> Cache {
        let hash_map = HashMap::new();
        let ref_cell = RefCell::new(hash_map);
        
        Cache { ref_cell }
    }
}