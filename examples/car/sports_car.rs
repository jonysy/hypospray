use engine::Engine;
use hypospray::Co;

#[inject(Engine)]
pub trait Deps { }

pub struct SportsCar<'imp, M: ?Sized + Deps> { pub engine: &'imp Co<M, Engine> }

impl<'imp, M: ?Sized + Deps> SportsCar<'imp, M> {
    
    pub fn gas(&self) {

        println!("{}", self.engine.rev());
    }
}