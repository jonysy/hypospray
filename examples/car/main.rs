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
    
    pub type ModuleDependencies = Graph<Module>;
}