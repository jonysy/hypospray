#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc_plugin;

use rustc_plugin::Registry;
use syntax::ast::MetaItem;
use syntax::codemap::Span;
use syntax::ext::base::{Annotatable, ExtCtxt, MultiItemModifier, SyntaxExtension};
use syntax::symbol::Symbol;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    let name = Symbol::intern("inject");
    let ext = SyntaxExtension::MultiModifier(Box::new(expand_inject) as Box<MultiItemModifier>);
    
    reg.register_syntax_extension(name, ext);
}

fn expand_inject(
    ecx: &mut ExtCtxt, 
    span: Span, 
    meta_item: &MetaItem, 
    item: Annotatable) -> Annotatable {
    
    println!("meta: {:?}\n\n, item: {:?}\n\n", meta_item, item);
    item
}