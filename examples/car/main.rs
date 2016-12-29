#![feature(plugin)]
#![plugin(hypospray_extensions)]

extern crate hypospray;

mod engine;
mod engine_gran_cabrio_v8;
mod module;
mod sports_car;

fn main() {
    use hypospray::Graph;
    use engine::Engine;
    use module::Module;
    use sports_car::SportsCar;
    
    let m = Graph::<Module>::new();
    let engine = m.dep::<Engine>();
    
    let car = SportsCar { engine: engine };
    car.gas()
}