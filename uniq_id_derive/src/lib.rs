extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

extern crate uniq_id;

use std::cell::RefCell;
use std::collections::HashMap;
use std::iter;
use self::proc_macro::TokenStream;

use quote::__rt::Span;
use syn::Attribute;
use syn::Data::Struct;
use syn::Field;
use syn::Fields::{Named, Unnamed, Unit};
use syn::Ident;
use syn::Meta::NameValue;
use syn::Lit::Str;
use syn::Type::*;

use uniq_id::Id;

type QuoteTokenStream = quote::__rt::TokenStream;

const GROUP_LIMIT: Id = std::u8::MAX;
thread_local!(static ID_COUNTER_MAP: RefCell<HashMap<String, Id>> = RefCell::new(HashMap::new()));

#[proc_macro_derive(UniqId, attributes(UniqGroup))]
pub fn uniq_id_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    // Add `UniqId` implementation.
    let uniq_id_impl = impl_uniq_id_trait(&ast);
    // Add `Serialize` implementation.
    let serialize_impl = impl_serialize_trait(&ast);
    // TODO: Add `Deserialize` implementation.

    // In order to get all of the necessary imports, without making the user import them, we put
    // all of our code and imports into a const block.  That way, we don't interfere with existing
    // imports in surrounding code.
    let block_name = Ident::new(&format!("IMPL_SHIT_FOR_{}", &ast.ident.to_string()),
                                Span::call_site());
    let result = quote! {
        // TODO: Can we fukn pls just use `_` here?
        pub const #block_name: () = {
            use ::uniq_id::serde::{Serialize, Deserialize};
            #uniq_id_impl
            #serialize_impl
        };
    };

    //println!("Output AST:");
    //println!("{}\n", result);

    result.into()
}

fn impl_uniq_id_trait(ast: &syn::DeriveInput) -> QuoteTokenStream {
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

        let type_name = &ast.ident;
        quote! {
            impl ::uniq_id::UniqId for #type_name {
                fn id() -> uniq_id::Id { #comp_id }
            }
        }
    })
}

fn impl_serialize_trait(ast: &syn::DeriveInput) -> QuoteTokenStream {
   let data_struct = match ast.data {
        Struct(ref ds) => ds,
        _ => panic!("macro only runnable on structs"),
    };

    let method_body = {
        let mut method_body = quote! {};
        for field in data_struct.fields.iter() {
            let ident = field.ident.clone().unwrap();
            match field.ty {
                Slice(ref s) => {
                    panic!("piss in my ass");
                },
                Array(ref a) => {
                    let arr_len = &a.len;
                    let stmt = quote! {
                        for i in range #arr_len {
                            result.append(&mut self[i].serialize());
                        }
                    };
                    method_body.extend(stmt);
                },
                Ptr(ref p) => panic!("pointer types not allowed in struct def"),
                Reference(ref r) => panic!("reference types not allowed in struct def"),
                BareFn(ref bf) => panic!("bare function types not allowed in struct def"),
                Never(ref n) => panic!("never types not allowed in struct def"),
                Tuple(ref t) => panic!("parenthesized types not allowed in struct def"),
                Path(ref p) => {
                    let stmt = quote! {
                        result.append(&mut self.#ident.serialize());
                    };
                    method_body.extend(stmt);
                },
                TraitObject(ref to) => panic!("dyn trait objects not allowed in struct def"),
                ImplTrait(ref it) => panic!("impl trait objects not allowed in struct def"),
                Paren(ref p) => panic!("parenthesized types not allowed in struct def"),
                Group(ref g) => println!("type groups not allowed in struct def"),
                Infer(ref i) => println!("underscore types not allowed in struct def"),
                Macro(ref m) => println!("macro types not allowed in struct def"),
                Verbatim(ref v) => println!("verbatim types not allowed in struct def"),
            };
        }
        method_body
    };

    let type_name = &ast.ident;
    quote! {
        impl ::uniq_id::serde::Serialize for #type_name {
            fn serialize(&self) -> ::std::vec::Vec<u8> {
                let mut result: Vec<u8> = Vec::new();
                #method_body
                result
            }
        }
    }
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
