use engine::Engine;
use hypospray::Construct;

#[implements(Engine)]
pub struct GranCabrioV8;

impl Engine for GranCabrioV8 {
    
    fn rev(&self) -> &'static str {
        
        return "Vrooom! Vroom! Vroooom!!!";
    }
}

impl<'dep> Construct<'dep> for GranCabrioV8 {
    
    type Dep = ();
    
    fn __construct(_: Self::Dep) -> GranCabrioV8 {
        GranCabrioV8
    }
}