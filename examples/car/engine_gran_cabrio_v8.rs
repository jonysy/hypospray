use engine::Engine;

#[implements(Engine)]
pub struct GranCabrioV8;

impl Engine for GranCabrioV8 {
    
    fn rev(&self) -> &'static str {
        
        return "Vrooom! Vroom! Vroooom!!!";
    }
}