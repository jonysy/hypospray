use engine::Engine;
use hypospray::{Co, Construct, };

#[inject(Engine)]
pub trait Deps { }

pub struct SportsCar<M: ?Sized + Deps> { pub engine: Co<M, Engine> }

impl<M: ?Sized + Deps> SportsCar<M> {
    
    pub fn gas(&self) {

        println!("{}", self.engine.rev());
    }
}

impl<'dep, M> Construct<'dep> for SportsCar<M> where M: ?Sized + Deps {
    
    type Dep = Co<M, Engine>;
    
    fn __construct(engine: Self::Dep) -> SportsCar<M> {
        SportsCar {
        	engine: engine,
        }
    }
}