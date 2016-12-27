use syntax::ast::{
    AngleBracketedParameterData,
    Generics, Ident, ImplPolarity, Item, ItemKind, Lifetime, MetaItemKind, MetaItem, NestedMetaItemKind, 
    Path, PathParameters, PathSegment, PolyTraitRef, TraitBoundModifier, 
    TraitRef, Ty, TyKind, TyParam, TyParamBound, Unsafety, Visibility,
    WhereClause
};
use syntax::codemap::{Spanned, Span};
use syntax::ext::base::{Annotatable, ExtCtxt};
use syntax::ptr::P;
use syntax::symbol::Symbol;
use syntax::util::ThinVec;

pub fn expand_inject(_: &mut ExtCtxt, _: Span, 
                     meta_item: &MetaItem, item: Annotatable) 
                     -> Vec<Annotatable>
{
    
    const GEN_T_PARAM: &'static str = "T";
    
    let meta_item_list = meta_item.meta_item_list().expect("Expected `MetaItemKind::List`");
    
    let item = item.expect_item();
    
    let mut co = vec![];
    
    let ty_param_bounds = {
        for nested_meta_item in meta_item_list.iter() {
            match nested_meta_item {
                &Spanned {
                    node: NestedMetaItemKind::MetaItem(MetaItem { 
                        name, 
                        node: MetaItemKind::Word, 
                        ..
                    }), 
                    span 
                } => {
                        
                    co.push((name, span));
                },
                
                _ => panic!("Expected `NestedMetaItemKind::MetaItem` at {:?}", item.span),
            }
        }
        
        P::from_vec(gen_ty_param_bounds(&item, &co))
    };
    
    let generics = Generics {
        lifetimes: vec![],
        ty_params: P::new(),
        where_clause: WhereClause {
            id: item.id,
            predicates: vec![],
        },
        span: item.span
    };
    
    let impl_trait = Item {
        ident: Ident::with_empty_ctxt(Symbol::intern("")),
        attrs: vec![],
        id: item.id,
        node: ItemKind::Impl(
            Unsafety::Normal,
            ImplPolarity::Positive,
            Generics {
                lifetimes: vec![],
                ty_params: P::from_vec(vec![TyParam {
                    attrs: ThinVec::new(),
                    ident: Ident::with_empty_ctxt(Symbol::intern(GEN_T_PARAM)),
                    id: item.id,
                    bounds: P::from_vec(gen_ty_param_bounds(&item, &co)),
                    default: None,
                    span: item.span,
                }]),
                where_clause: WhereClause {
                    id: item.id,
                    predicates: vec![],
                },
                span: item.span,
            },
            Some(TraitRef {
                path: Path {
                    span: item.span,
                    global: false,
                    segments: vec![
                        PathSegment {
                            identifier: item.ident,
                            parameters: PathParameters::none(),
                        }
                    ],
                },
                ref_id: item.id,
            }),
            P(Ty {
                id: item.id,
                node: TyKind::Path(None, Path {
                    span: item.span,
                    global: false,
                    segments: vec![PathSegment {
                        identifier: Ident::with_empty_ctxt(Symbol::intern(GEN_T_PARAM)),
                        parameters: PathParameters::none(),
                    }]
                }),
                span: item.span,
            }),
            vec![]
        ),
        vis: Visibility::Inherited,
        span: item.span, 
    };
    
    let item = item.map(|mut item| {
        item.node = ItemKind::Trait(Unsafety::Normal, generics, ty_param_bounds, vec![]);
        item
    });
    
    vec![Annotatable::Item(item), Annotatable::Item(P(impl_trait))]
}

fn gen_ty_param_bounds(item: &Item, co: &Vec<(Symbol, Span)>) -> Vec<TyParamBound> {
    
    let mut ty_param_bounds = vec![];
    
    for &(name, span) in co.iter() {
        let ty_param_bound = TyParamBound::TraitTyParamBound(
            PolyTraitRef {
                bound_lifetimes: vec![],
                trait_ref: TraitRef {
                    path: Path {
                        span: span,
                        global: true,
                        segments: vec![
                            PathSegment {
                                identifier: Ident::with_empty_ctxt(Symbol::intern("hypospray")),
                                parameters: PathParameters::none(),
                            },
                            PathSegment {
                                identifier: Ident::with_empty_ctxt(Symbol::intern("Component")),
                                parameters: PathParameters::AngleBracketed(
                                    AngleBracketedParameterData {
                                        lifetimes: vec![],
                                        types: P::from_vec(vec![P(Ty {
                                            id: item.id,
                                            node: TyKind::Path(None, Path {
                                                span: span,
                                                global: false,
                                                segments: vec![PathSegment {
                                                    identifier: Ident::with_empty_ctxt(name),
                                                    parameters: PathParameters::none(),
                                                }]
                                            }),
                                            span: span,
                                        })]),
                                        bindings: P::from_vec(vec![]),
                                    }
                                ),
                            },
                        ]
                    },
                    ref_id: item.id,
                },
                span: span,
            },
            TraitBoundModifier::None
        );
        
        ty_param_bounds.push(ty_param_bound);
    }
    
    ty_param_bounds.push(TyParamBound::RegionTyParamBound(Lifetime {
        id: item.id,
        span: item.span,
        name: Symbol::intern("'static")
    }));
    
    ty_param_bounds
}