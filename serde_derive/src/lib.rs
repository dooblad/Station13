extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate slog;
extern crate slog_async;
extern crate slog_term;
extern crate syn;

extern crate serde;

mod enum_impl;
mod serde_impl;
mod validate;

use self::proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use slog::*;
use syn::Data::{Enum, Struct};

use self::validate::validate;

type QuoteTokenStream = quote::__rt::TokenStream;

#[proc_macro_derive(Serde, attributes(IdGroup))]
pub fn serde_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    validate(&ast);

    // Add utility methods, if we're being run on an enum.
    let enum_tag_impl_tokens = if let syn::Data::Enum(ref de) = ast.data {
        enum_impl::impl_enum_methods(&ast, de)
    } else {
        quote! {}
    };
    // Add `Serialize` and `Deserialize` implementation.
    let serde_impl_tokens = serde_impl::impl_serde_traits(&ast);

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
            #enum_tag_impl_tokens
            #serde_impl_tokens
        };
    };

    let decorator = slog_term::TermDecorator::new().force_color().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let logger = slog::Logger::root(drain, o!());

    debug!(logger, "Output AST:\n{}\n", result);

    result.into()
}
