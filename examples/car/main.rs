#![feature(plugin)]
#![plugin(hypospray_extensions)]

extern crate hypospray;

mod engine;
mod engine_gran_cabrio_v8;
mod module;
mod sports_car;

fn main() {
    use hypospray::Graph;
    use module::Module;
    use sports_car::SportsCar;
    
    let m = Graph::<Module>::new();

    let car = m.construct::<SportsCar<_>>();

    //let car = SportsCar { engine: m.dep::<Engine>() };
    car.gas();
}