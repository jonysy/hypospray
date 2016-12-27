use hypospray::Co;

#[inject(Engine, Manufacturer)]
pub trait Mod { }

pub trait Engine {
    fn rev(&self) -> &'static str;
}

pub trait Manufacturer {
    fn name(&self) -> &'static str;
}

pub struct SportsCar<'imp, M> where M: ?Sized + Mod {
    
    engine: &'imp Co<M, Engine>,
    //manu: &'imp Co<M, Manufacturer>,
}

impl<'imp, M> SportsCar<'imp, M> where M: ?Sized + Mod {
    
    fn rev_engine(&self) {
        println!("{}", self.engine.rev());
    }
}