use hypospray::Co;

#[inject(Engine, Manufacturer)]
pub trait Mod { }

//pub trait Mod: ::hypospray::Component<Engine> where Co<Self, Engine>: Engine { }

pub trait Engine {
    fn rev(&self) -> &'static str;
}

pub trait Manufacturer {
    fn name(&self) -> &'static str;
}

pub struct SportsCar<'imp, M: ?Sized + Mod> {
    
    engine: &'imp Co<M, Engine>,
    //manu: &'imp Co<M, Manufacturer>,
}

impl<'imp, M: ?Sized + Mod> SportsCar<'imp, M> where Co<M, Engine>: Engine {
    
    fn rev_engine(&self) {
        println!("{}", self.engine.rev());
    }
}