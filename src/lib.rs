//! Dependency Injection (DI)
//!
//! # Goals
//!
//! * Design to allow for focused, reusable, testable components
//! * A dependency graph checked at compile time
//!
//! # What is Dependency Injection?
//!
//! [5-minute introduction.](https://youtu.be/IKD2-MAkXyQ)
//!
//! # Cyclic Dependency
//!
//! DI is not for circular dependency resolution: [Circular dependency is something that is to be 
//! avoided][1].
//!
//! [1]: http://misko.hevery.com/2008/08/01/circular-dependency-in-constructors-and-dependency-injection/

pub use core::{Co, Component, ComponentImp, Construct, };
pub use graph::Graph;
pub use lifecycle::{Prototype, Singleton, };

mod core;
mod graph;
mod lifecycle;