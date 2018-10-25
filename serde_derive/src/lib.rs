extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

extern crate serde;

mod trait_impl;

use self::proc_macro::TokenStream;
use proc_macro2::{Ident, Span};


#[proc_macro_derive(Serde, attributes(IdGroup))]
pub fn serde_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

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
            #serde_impls
        };
    };

    // TODO: Get some logging infrastructure, so we can use debug prints.
    //println!("Output AST:");
    //println!("{}", result);
    //println!();

    result.into()
}
