#![feature(plugin)]
#![plugin(hypospray_extensions)]

extern crate hypospray;

use hypospray::Co;

fn main() {

}

#[inject(Engine, Manufacturer)]
pub trait Deps { }

pub struct SportsCar<M> where M: ?Sized + Deps { engine: Co<M, Engine> }

impl<M> SportsCar<M> where M: ?Sized + Deps {
    
    fn gas(&self) {
        println!("{}", self.engine.rev())
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