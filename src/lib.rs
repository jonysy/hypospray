//! Dependency Injection
//!
//! ## Cyclic Dependency
//!
//! DI is not for circular dependency resolution. Circular dependency is something that is to be 
//! avoided: [Circular Dependency in constructors and Dependency Injection][1]
//!
//! ## Issues
//!
//! * [The value of the associated type `_` must be specified](https://git.io/vMf3W)
//! * [Associated type constraints on supertraits don't propagate to usage sites](https://git.io/vMf3l)
//! * [Incorrect "associated type must be specified" for supertrait](https://git.io/vMf3l)
//!
//! [1]: http://misko.hevery.com/2008/08/01/circular-dependency-in-constructors-and-dependency-injection/
#![allow(warnings)]
#![feature(field_init_shorthand)]

pub use core::{Co, Component, ComponentImp, ConstructFn, };
pub use ownership::{PerCall, PerGraph};
pub use registry::Graph;

mod core;
mod ownership;
mod registry;