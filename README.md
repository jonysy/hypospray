# Hypospray

A lightweight dependency injection library

[![Build Status](https://travis-ci.org/jonysy/hypospray.svg?branch=master)](https://travis-ci.org/jonysy/hypospray) [![License](https://img.shields.io/crates/l/hypospray.svg)](LICENSE)

## Usage

To use `hypospray`, add the following to `Cargo.toml`:

```toml
[dependencies]
hypospray = "0.1.0"
hypospray_extensions = "0.1.0"
```

Then, add the following to your crate root:

```rust
#![feature(plugin)]
#![plugin(hypospray_extensions)]

extern crate hypospray;

..
```

### A quick look at Hypospray basics:

Add the following to `src/main.rs`:

```rust
#![feature(plugin)]
#![plugin(hypospray_extensions)]

extern crate hypospray;

use hypospray::{Co, Construct, Graph};

fn main() {
    let m = Graph::<Module>::new();

    let car = m.construct::<SportsCar<_>>();

    car.gas();
}

trait Engine {
    
    fn rev(&self) -> &'static str;
}

#[implements(Engine)]
struct GranCabrioV8;

impl Engine for GranCabrioV8 {
    
    fn rev(&self) -> &'static str {
        
        return "Vrooom! Vroom! Vroooom!!!";
    }
}

impl<'dep> Construct<'dep> for GranCabrioV8 {
    
    type Dep = ();
    
    fn __construct(_: Self::Dep) -> GranCabrioV8 {
        GranCabrioV8
    }
}

#[bind(Engine = "GranCabrioV8#Prototype")]
pub trait Module { }

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
```

## License

Licensed under either:

* Apache License, Version 2.0, ([LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license ([LICENSE-MIT](http://opensource.org/licenses/MIT))