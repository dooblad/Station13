extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

extern crate uniq_comp_id;

use std::cell::Cell;

use self::proc_macro::TokenStream;

use uniq_comp_id::CompId;

const COMP_LIMIT: CompId = std::u8::MAX;
thread_local!(static ID_COUNTER: Cell<CompId> = Cell::new(0));

#[proc_macro_derive(UniqCompId)]
pub fn uniq_comp_id_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_uniq_comp_id_macro(&ast)
}

fn impl_uniq_comp_id_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    ID_COUNTER.with(|id| {
        // Use current counter value for this component's ID.
        let comp_id = id.get();
        // Make sure we're not running out of bits.
        if comp_id == COMP_LIMIT {
            panic!("reached limit on number of components (limit == {})", COMP_LIMIT);
        }
        // Increment counter.
        id.set(comp_id + 1);
        let gen = quote! {
            impl uniq_comp_id::UniqCompId for #name {
                fn id() -> uniq_comp_id::CompId { #comp_id }
            }
        };
        gen.into()
    })
}
