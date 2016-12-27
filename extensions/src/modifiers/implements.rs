use aster::AstBuilder;
use syntax::ast::{
    AngleBracketedParameterData,
    Generics, Ident, ImplPolarity, Item, ItemKind, Lifetime, MetaItemKind, MetaItem, NestedMetaItemKind, 
    NodeId, Path, PathParameters, PathSegment, PolyTraitRef, TraitBoundModifier, 
    TraitItem, TraitRef, Ty, TyKind, TyParam, TyParamBound, Unsafety, Visibility, WhereBoundPredicate, 
    WhereClause, WherePredicate
};
use syntax::codemap::{Spanned, Span};
use syntax::ext::base::{Annotatable, ExtCtxt};
use syntax::ptr::P;
use syntax::symbol::Symbol;
use syntax::util::ThinVec;

pub fn expand_implements(ecx: &mut ExtCtxt, span: Span, 
                         meta_item: &MetaItem, item: &Annotatable, 
                         push: &mut FnMut(Annotatable))
{
    
    let list = {
        meta_item
            .meta_item_list()
            .expect("Expected `MetaItemKind::List`")
    };
    
    let meta = match list {
        &[Spanned {
            node: NestedMetaItemKind::MetaItem(MetaItem {
                name, 
                node: MetaItemKind::Word, 
                span
            }),
            .. 
        }] => {
            MetaItem {
                name: name,
                node: MetaItemKind::Word,
                span: span,
            }
        },
        _ => panic!("Expected `NestedMetaItemKind::MetaItem`"),
    };
    
    let mut item = item.clone().expect_item();
    let ident = item.ident;
    
    let generics = match item.node {
        ItemKind::Struct(_, ref generics) => {
            generics
        },
        
        _ => {
            panic!("Something went wrong!");
        }
    };
    
    let ref where_clause = generics.where_clause;
    
    let builder = AstBuilder::new();
    
    let ty = builder.ty().path()
		.segment(item.ident).with_generics(generics.clone()).build().build();
    
    let co = builder.ty().path()
		.segment(meta.name).build().build();
    
    push(Annotatable::Item(
        quote_item!(ecx,
            
            impl$generics AsRef<$co> for $ty $where_clause {
                
                fn as_ref(&self) -> &($co + 'static) { self }
            }
        ).unwrap(),
    ));
    
    push(Annotatable::Item(
        quote_item!(ecx,
            
            impl$generics AsMut<$co> for $ty $where_clause {
                
                fn as_mut(&mut self) -> &mut($co + 'static) { self }
            }
        ).unwrap(),
    ));
}