#![allow(warnings)]
#![feature(plugin)]
#![plugin(hypospray_extensions)]

extern crate hypospray;

mod simple {
	use hypospray::{Construct, Graph};

	trait Foo { fn foo(&self) -> u8; }

	#[implements(Foo)]
	struct Bar;
	impl Foo for Bar { fn foo(&self) -> u8 { 10 } }
	impl<'d> Construct<'d> for Bar { type Dep = (); fn __construct(_: ()) -> Bar { Bar } }

	#[bind(Foo = "Bar#Prototype")]
	pub trait Module { }

	#[test]
	fn test() {
		let m = Graph::<Module>::new();
		let bar = m.construct::<Bar>();

		assert_eq!(bar.foo(), 10);
	}
}

mod inject {
	use hypospray::{Co, Construct, Graph};
	
	trait Foo { fn foo(&self) -> u8; }

	#[implements(Foo)]
	struct Bar;
	impl Foo for Bar { fn foo(&self) -> u8 { 12 } }
	impl<'d> Construct<'d> for Bar { type Dep = (); fn __construct(_: ()) -> Bar { Bar } }

	#[inject(Foo)]
	trait QuxDeps { }

	struct Qux<M: ?Sized + QuxDeps> { pub foo: Co<M, Foo> }

	impl<'dep, M> Construct<'dep> for Qux<M> where M: ?Sized + QuxDeps {
    
	    type Dep = Co<M, Foo>;
	    
	    fn __construct(foo: Self::Dep) -> Qux<M> {
	        Qux {
	        	foo: foo,
	        }
	    }
	}

	#[bind(Foo = "Bar#Prototype")]
	pub trait Module { }

	#[test]
	fn test() {
		let m = Graph::<Module>::new();
		let qux = m.construct::<Qux<_>>();

		assert_eq!(qux.foo.foo(), 12);
	}
}

mod reference_singleton {
	use hypospray::{Co, Construct, Graph};
	
	trait Foo { fn foo(&self) -> u8; }

	#[implements(Foo)]
	struct Bar;
	impl Foo for Bar { fn foo(&self) -> u8 { 14 } }
	impl<'d> Construct<'d> for Bar { type Dep = (); fn __construct(_: ()) -> Bar { Bar } }

	#[inject(Foo)]
	trait QuxDeps { }

	struct Qux<'a, M: ?Sized + QuxDeps> { pub foo: &'a Co<M, Foo> }

	impl<'dep, M> Construct<'dep> for Qux<'dep, M> where M: ?Sized + QuxDeps {
    
	    type Dep = &'dep Co<M, Foo>;
	    
	    fn __construct(foo: Self::Dep) -> Qux<M> {
	        Qux {
	        	foo: foo,
	        }
	    }
	}

	#[bind(Foo = "Bar#Singleton")]
	pub trait Module { }

	#[test]
	fn test() {
		let m = Graph::<Module>::new();
		let qux = m.construct::<Qux<_>>();

		assert_eq!(qux.foo.foo(), 14);
	}
}