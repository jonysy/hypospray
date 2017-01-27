# Hypospray

A lightweight dependency injection library

[![Build Status](https://travis-ci.org/jonysy/hypospray.svg?branch=master)](https://travis-ci.org/jonysy/hypospray) 
[![License](https://img.shields.io/crates/l/hypospray.svg)](LICENSE)

### Version

```sh
$ rustup -V
rustup 1.0.0 (17b6d21 2016-12-15)
$ rustc -V
rustc 1.16.0-nightly (7821a9b99 2017-01-23)
```

## What is Dependency Injection?

[5-minute introduction.](https://youtu.be/IKD2-MAkXyQ)

## Goals

* Design to allow for focused, reusable, testable components
* A dependency graph checked at compile time

## Cyclic Dependency

DI is not for circular dependency resolution: [Circular dependency is something that is to be avoided][di post].

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

## License

Licensed under either:

* Apache License, Version 2.0, ([LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license ([LICENSE-MIT](http://opensource.org/licenses/MIT))

[di post]: http://misko.hevery.com/2008/08/01/circular-dependency-in-constructors-and-dependency-injection/