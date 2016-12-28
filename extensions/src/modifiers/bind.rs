use syntax::ast::{Ident, LitKind, MetaItemKind, MetaItem, NestedMetaItemKind, StrStyle};
use syntax::codemap::{Spanned, Span};
use syntax::ext::base::{Annotatable, ExtCtxt};

pub fn expand_bind(ecx: &mut ExtCtxt, _: Span, 
                         meta_item: &MetaItem, item: &Annotatable, 
                         push: &mut FnMut(Annotatable))
{
    
    let module = item.clone().expect_item().ident;
    
    let list = {
        meta_item
            .meta_item_list()
            .expect("Expected `MetaItemKind::List`")
    };
    
    for item in list.iter() {
        match item {
            &Spanned {
                node: NestedMetaItemKind::MetaItem(MetaItem {
                    name: tr_name,
                    node: MetaItemKind::NameValue(Spanned {
                        node: LitKind::Str(symbol, StrStyle::Cooked),
                        ..
                    }),
                    ..
                }),
                ..
            } => {
                
                match &symbol.to_string().split('#').collect::<Vec<&str>>()[..] {
                    &[scope, imp] => {
                        
                        let scope = Ident::from_str(scope);
                        let imp = Ident::from_str(imp);
                        let tr = Ident::with_empty_ctxt(tr_name);
                        
                        push(Annotatable::Item(
                            quote_item!(ecx,

                                impl ::hypospray::Component<$tr> for $module {

                                    type ComponentImp = $imp;
                                    type Scope = ::hypospray::$scope;
                                }
                            ).unwrap(),
                        ));
                    },
                    
                    _ => {
                        panic!("Expected pattern `<kind>#<impl>`")
                    }
                }
            },
            
            _ => {
                panic!("Expected `NestedMetaItemKind::MetaItem`")
            }
        }
    }
}