#![feature(field_init_shorthand)]

pub use component::{Component, ComponentImp};
pub use graph_co::Graph;

mod component;
mod graph_co;
mod lifetime;