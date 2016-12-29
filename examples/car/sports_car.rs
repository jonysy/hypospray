use engine::Engine;
use hypospray::Co;

#[inject(Engine)]
pub trait Deps { }

pub struct SportsCar<M: ?Sized + Deps> { engine: Co<M, Engine> }

impl<M: ?Sized + Deps> SportsCar<M> {
    
    pub fn new(engine: Co<M, Engine>) -> Self {
        
        SportsCar { engine: engine }
    }
    
    pub fn gas(&self) {

        println!("{}", self.engine.rev());
    }
}