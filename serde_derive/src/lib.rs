extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

extern crate serde;

use self::proc_macro::TokenStream;
use std::cell::RefCell;
use std::collections::HashMap;

use quote::__rt::Span;
use syn::Attribute;
use syn::Data::Struct;
use syn::Ident;
use syn::Lit::Str;
use syn::Meta::NameValue;
use syn::Type::*;

use serde::Id;

type QuoteTokenStream = quote::__rt::TokenStream;

const GROUP_LIMIT: Id = std::u8::MAX;
thread_local!(static ID_COUNTER_MAP: RefCell<HashMap<String, Id>> = RefCell::new(HashMap::new()));

#[proc_macro_derive(Serde, attributes(IdGroup))]
pub fn serde_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    // Add `UniqId` implementation (if `IdGroup` is specified).
    let uniq_id_impl = match parse_group_attr(&ast.attrs) {
        Some(ug) => impl_uniq_id_trait(&ast, ug),
        None => quote! {},
    };
    // Add `Serialize` and `Deserialize` implementation.
    let serde_impls = impl_serde_traits(&ast);

    // In order to get all of the necessary imports, without making the user import them, we put
    // all of our code and imports into a const block.  That way, we don't interfere with existing
    // imports in surrounding code.
    let block_name = Ident::new(
        &format!("IMPL_SHIT_FOR_{}", &ast.ident.to_string()),
        Span::call_site(),
    );
    (quote! {
        // TODO: Can we fukn pls just use `_` here?
        pub const #block_name: () = {
            #uniq_id_impl
            #serde_impls
        };
    })
    .into()
}

/// Generates an implementation for the `UniqId` trait.
fn impl_uniq_id_trait(ast: &syn::DeriveInput, uniq_group: String) -> QuoteTokenStream {
    ID_COUNTER_MAP.with(|cnt_map| {
        // Use current counter value for this component's ID.
        let mut cnt_map_ref = cnt_map.borrow_mut();
        // Make a counter for this group, if it doesn't already exist.
        if !cnt_map_ref.contains_key(&uniq_group) {
            cnt_map_ref.insert(uniq_group.clone(), 0);
        }
        let comp_id = cnt_map_ref.get(&uniq_group).unwrap().clone();
        // Make sure we're not running out of bits.
        if comp_id == GROUP_LIMIT {
            panic!(
                "reached limit on number of items in group \"{}\" (limit is {})",
                uniq_group, GROUP_LIMIT
            );
        }
        // Increment counter.
        cnt_map_ref.insert(uniq_group.clone(), comp_id + 1);

        let type_name = &ast.ident;
        quote! {
            impl ::serde::UniqId for #type_name {
                fn id() -> serde::Id { #comp_id }
            }
        }
    })
}

/// Generates implementations for the `Serialize` and `Deserialize` traits.
fn impl_serde_traits(ast: &syn::DeriveInput) -> QuoteTokenStream {
    let data_struct = match ast.data {
        Struct(ref ds) => ds,
        _ => panic!("macro only runnable on structs"),
    };

    // Iterate through struct fields and build method bodies.
    let mut ser_body = quote!{};
    let mut deser_body = quote!{};
    for field in data_struct.fields.iter() {
        let ident = field.ident.clone().unwrap();
        ser_body.extend(quote! {
            result.append(&mut self.#ident.serialize());
        });
        let type_tokens = match field.ty {
            Slice(_) => panic!("slice types not allowed in struct def"),
            Array(ref a) => {
                let elem_ty = &(*a.elem);
                let arr_len = &a.len;
                quote! { [#elem_ty; #arr_len] }
            }
            Ptr(_) => panic!("pointer types not allowed in struct def"),
            Reference(_) => panic!("reference types not allowed in struct def"),
            BareFn(_) => panic!("bare function types not allowed in struct def"),
            Never(_) => panic!("never types not allowed in struct def"),
            Tuple(_) => panic!("tuple types not allowed in struct def"),
            Path(ref p) => {
                quote! { #p }
            }
            TraitObject(_) => panic!("dyn trait objects not allowed in struct def"),
            ImplTrait(_) => panic!("impl trait objects not allowed in struct def"),
            Paren(_) => panic!("parenthesized types not allowed in struct def"),
            Group(_) => panic!("type groups not allowed in struct def"),
            Infer(_) => panic!("underscore types not allowed in struct def"),
            Macro(_) => panic!("macro types not allowed in struct def"),
            Verbatim(_) => panic!("verbatim types not allowed in struct def"),
        };
        deser_body.extend(quote! {
            let deser_data = <#type_tokens>::deserialize(&data[bytes_read..]);
            bytes_read += deser_data.0;
            ::std::ptr::write(&mut result.#ident as *mut #type_tokens, deser_data.1);
        });
    }

    // Generate trait implementations.
    let type_name = &ast.ident;
    quote! {
        impl ::serde::Serialize for #type_name {
            fn serialize(&self) -> ::std::vec::Vec<u8> {
                let mut result: Vec<u8> = Vec::new();
                #ser_body
                result
            }
        }

        impl ::serde::Deserialize for #type_name {
            fn deserialize(data: &[u8]) -> (usize, Self) {
                unsafe {
                    let mut result: Self = unsafe { ::std::mem::uninitialized() };
                    let mut bytes_read: usize = 0;
                    #deser_body
                    (bytes_read, result)
                }
            }
        }
    }
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
