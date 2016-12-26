use super::EngineComponent;

pub struct V8Engine;

impl EngineComponent for V8Engine {
    
    fn rev(&self) -> &'static str {
        
        "Vrooom! Vroom! Vroooom!!!"
    }
}