use aster::AstBuilder;
use syntax::ast::{ItemKind, MetaItemKind, MetaItem, NestedMetaItemKind};
use syntax::codemap::{Spanned, Span};
use syntax::ext::base::{Annotatable, ExtCtxt};

pub fn expand_bind(ecx: &mut ExtCtxt, span: Span, 
                         meta_item: &MetaItem, item: &Annotatable, 
                         push: &mut FnMut(Annotatable))
{
    
    println!("`MetaItem`: {:#?}\n\n", meta_item);
    println!("`Annotatable`: {:#?}\n\n", item);
}