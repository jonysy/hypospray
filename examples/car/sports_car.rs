use engine::Engine;
use hypospray::Co;

#[inject(Engine)]
pub trait Deps { }

pub struct SportsCar<M: ?Sized + Deps> { pub engine: Co<M, Engine> }

impl<M: ?Sized + Deps> SportsCar<M> {
    
    pub fn gas(&self) {

        println!("{}", self.engine.rev());
    }
}