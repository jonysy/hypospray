//! Dependency Injection (DI)
//!
//! # Goals
//!
//! * Focused, reusable, testable components
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
//! # Example
//!
//! ```rust
//! #![feature(plugin)]
//! #![plugin(hypospray_extensions)]
//! extern crate hypospray;
//!
//! use hypospray::{Co, Construct, Graph};
//!

// work around: https://github.com/rust-lang/cargo/issues/960

//! # fn main() {
//! trait Engine {
//!
//!     fn rev(&self) -> &'static str;
//! }
//!
//! #[implements(Engine)]
//! struct GranCabrioV8;
//!
//! impl Engine for GranCabrioV8 {
//!
//!     fn rev(&self) -> &'static str {
//!
//!         return "Vrooom! Vroom! Vroooom!!!";
//!     }
//! }
//!
//! impl<'dep> Construct<'dep> for GranCabrioV8 {
//!
//!     type Dep = ();
//!
//!     fn __construct(_: Self::Dep) -> GranCabrioV8 {
//!         GranCabrioV8
//!     }
//! }
//!
//! #[inject(Engine)]
//! trait Deps { }
//!
//! struct SportsCar<M: ?Sized + Deps> { engine: Co<M, Engine> }
//!
//! impl<M: ?Sized + Deps> SportsCar<M> {
//!
//!     fn gas(&self) {
//!
//!         println!("{}", self.engine.rev());
//!     }
//! }
//!
//! impl<'dep, M> Construct<'dep> for SportsCar<M> where M: ?Sized + Deps {
//!
//!     type Dep = Co<M, Engine>;
//!
//!     fn __construct(engine: Self::Dep) -> SportsCar<M> {
//!         SportsCar {
//!             engine: engine,
//!         }
//!     }
//! }
//!
//! #[bind(Engine = "GranCabrioV8#Prototype")]
//! trait Module { }
//!
//! type ModuleDependencies = Graph<Module>;
//!
//! let m = ModuleDependencies::new();
//!
//! let car: SportsCar<_> = m.construct();
//!
//! car.gas();
//! # }
//! ```
//!
//! [1]: http://misko.hevery.com/2008/08/01/circular-dependency-in-constructors-and-dependency-injection/

pub use core::{Co, Component, ComponentImp, Construct};
pub use graph::Graph;
pub use lifecycle::{Prototype, Singleton};

mod core;
mod graph;
mod lifecycle;