use engine::Engine;
use hypospray::*;

#[inject(Engine)]
pub trait Deps { }

pub struct SportsCar<M: ?Sized + Deps> { engine: Co<M, Engine> }

impl<M: ?Sized + Deps> SportsCar<M> {
    
    pub fn gas(&self) {

        println!("{}", self.engine.rev());
    }
}