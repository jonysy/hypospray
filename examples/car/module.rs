use engine::Engine;
use engine_gran_cabrio_v8::GranCabrioV8;

#[bind(Engine = "GranCabrioV8#Singleton")]
pub trait Module { }