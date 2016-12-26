use super::EngineComponent;

pub struct Vehicle<'co> {
    pub engine: &'co EngineComponent,
}

impl<'co> Vehicle<'co> {
    
    pub fn rev_engine(&self) -> &'static str {
        self.engine.rev()
    }
}