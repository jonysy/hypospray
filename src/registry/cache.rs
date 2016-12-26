use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use super::ComponentKey;

pub struct Cache {
    ref_cell: RefCell<HashMap<ComponentKey, Box<Any>>>,
}

impl Cache {
    
    pub fn new() -> Cache {
        let hash_map = HashMap::new();
        let ref_cell = RefCell::new(hash_map);
        
        Cache { ref_cell }
    }
}