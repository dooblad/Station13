extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

extern crate uniq_id;

use std::cell::RefCell;
use std::collections::HashMap;

use self::proc_macro::TokenStream;

use syn::Attribute;
use syn::Meta::NameValue;
use syn::Lit::Str;

use uniq_id::Id;

const GROUP_LIMIT: Id = std::u8::MAX;
thread_local!(static ID_COUNTER_MAP: RefCell<HashMap<String, Id>> = RefCell::new(HashMap::new()));

// TODO: Rename to `UniqId`, once we know this'll work.
#[proc_macro_derive(UniqId, attributes(UniqGroup))]
pub fn uniq_comp_id_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_uniq_comp_id_macro(&ast)
}

fn impl_uniq_comp_id_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    ID_COUNTER_MAP.with(|cnt_map| {
        let uniq_group = parse_group_attr(&ast.attrs);
        // Use current counter value for this component's ID.
        let mut cnt_map_ref = cnt_map.borrow_mut();
        // Make a counter for this group, if it doesn't already exist.
        if !cnt_map_ref.contains_key(&uniq_group) {
            cnt_map_ref.insert(uniq_group.clone(), 0);
        }
        let comp_id = cnt_map_ref.get(&uniq_group).unwrap().clone();
        // Make sure we're not running out of bits.
        if comp_id == GROUP_LIMIT {
            panic!("reached limit on number of items in group \"{}\" (limit is {})",
                   uniq_group, GROUP_LIMIT);
        }
        // Increment counter.
        cnt_map_ref.insert(uniq_group.clone(), comp_id + 1);
        let gen = quote! {
            impl ::uniq_id::UniqId for #name {
                fn id() -> uniq_id::Id { #comp_id }
            }
        };
        gen.into()
    })
}

/// Parses the `UniqGroup` attribute.
fn parse_group_attr(attrs: &Vec<Attribute>) -> String {
    if attrs.len() != 1 {
        panic!("exactly one attribute (`UniqGroup`) expected (got {})", attrs.len());
    }

    match attrs[0].interpret_meta() {
        Some(NameValue(nv)) => {
            // TODO: Assert the name is `UniqGroup`.
            if nv.ident.to_string() != "UniqGroup" {
                panic!("only \"UniqGroup\" may be set");
            }
            match nv.lit {
                 Str(lit_str) => lit_str.value(),
                _ => panic!("expected string value"),
            }
        },
        _ => panic!("improper attribute format"),
    }
}
