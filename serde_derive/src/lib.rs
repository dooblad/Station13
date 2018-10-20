extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

extern crate serde;

mod trait_impl;

use self::proc_macro::TokenStream;

use quote::__rt::Span;
use syn::Attribute;
use syn::Ident;
use syn::Lit::Str;
use syn::Meta::NameValue;

#[proc_macro_derive(Serde, attributes(IdGroup))]
pub fn serde_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    // Add `UniqId` implementation (if `IdGroup` is specified).
    let uniq_id_impl = match parse_group_attr(&ast.attrs) {
        Some(ug) => trait_impl::impl_uniq_id_trait(&ast, ug),
        None => quote! {},
    };
    // Add `Serialize` and `Deserialize` implementation.
    let serde_impls = trait_impl::impl_serde_traits(&ast);

    // In order to get all of the necessary imports, without making the user import them, we put
    // all of our code and imports into a const block.  That way, we don't interfere with existing
    // imports in surrounding code.
    let block_name = Ident::new(
        &format!("IMPL_SHIT_FOR_{}", &ast.ident.to_string()),
        Span::call_site(),
    );
    let result = quote! {
        // TODO: Can we fukn pls just use `_` here?
        pub const #block_name: () = {
            #uniq_id_impl
            #serde_impls
        };
    };

    // TODO: Get some logging infrastructure.
    //debug!("Output AST:");
    //debug!("{}", result);

    result.into()
}

/// Parses the `IdGroup` attribute.
fn parse_group_attr(attrs: &Vec<Attribute>) -> Option<String> {
    if attrs.len() > 1 {
        panic!(
            "at most one attribute (`IdGroup`) allowed (got {})",
            attrs.len()
        );
    } else if attrs.len() == 0 {
        return None;
    }

    Some(match attrs[0].interpret_meta() {
        Some(NameValue(nv)) => {
            if nv.ident.to_string() != "IdGroup" {
                panic!("only \"IdGroup\" may be set");
            }
            match nv.lit {
                Str(lit_str) => lit_str.value(),
                _ => panic!("expected string value"),
            }
        }
        _ => panic!("improper attribute format"),
    })
}
