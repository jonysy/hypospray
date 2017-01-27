#![feature(box_syntax, plugin_registrar, quote, rustc_private, slice_patterns)]

extern crate quote;
extern crate syntax;
extern crate aster;
extern crate rustc_plugin;

mod decorator;
mod modifier;

use rustc_plugin::Registry;
use syntax::ext::base::SyntaxExtension::MultiDecorator;
use syntax::ext::base::SyntaxExtension::MultiModifier;
use syntax::symbol::Symbol;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    
    reg.register_syntax_extension(
        Symbol::intern("inject"),
        MultiModifier(box modifier::expand_inject)
    );
    
    reg.register_syntax_extension(
        Symbol::intern("implements"),
        MultiDecorator(box decorator::expand_implements)
    );
    
    reg.register_syntax_extension(
        Symbol::intern("bind"),
        MultiDecorator(box decorator::expand_bind)
    );
}