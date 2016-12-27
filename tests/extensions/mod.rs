use hypospray::Co;

#[inject(Engine, Manufacturer)]
pub trait Mod { }

pub struct SportsCar<'imp, M: ?Sized+Mod> { engine: &'imp Co<M, Engine>, manu: &'imp Co<M, Manufacturer> }

impl<'imp, M> SportsCar<'imp, M> where M: ?Sized + Mod {
    
    fn rev_engine(&self) {
        println!("{}", self.engine.rev());
    }
    
    fn manu(&self) {
        
        println!("{}", self.manu.name())
    }
}

pub trait Engine {
    fn rev(&self) -> &'static str;
}

#[implements(Engine)]
pub struct GranCabrioV8;

impl Engine for GranCabrioV8 {
    
    fn rev(&self) -> &'static str {
        "Vrooom! Vroom! Vroooom!!!"
    }
}

pub trait Manufacturer {
    fn name(&self) -> &'static str;
}