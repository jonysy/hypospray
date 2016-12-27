#![allow(warnings)]
#![feature(plugin_registrar, quote, rustc_private)]

#[macro_use] extern crate quote;
#[macro_use] extern crate syntax;
extern crate rustc_plugin;

mod modifiers;

use rustc_plugin::Registry;
use syntax::ext::base::{MultiItemModifier, SyntaxExtension};
use syntax::symbol::Symbol;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    let name = Symbol::intern("inject");
    let ext = SyntaxExtension::MultiModifier(Box::new(modifiers::expand_inject));
    
    reg.register_syntax_extension(name, ext);
}