use std::cell::RefCell;
use std::collections::HashMap;

use syn::Data::{Enum, Struct};
use syn::{DataEnum, DataStruct};
use syn::Type::*;

use serde::Id;

const GROUP_LIMIT: Id = std::u8::MAX;
thread_local!(static ID_COUNTER_MAP: RefCell<HashMap<String, Id>> = RefCell::new(HashMap::new()));

type QuoteTokenStream = quote::__rt::TokenStream;

/// Generates an implementation for the `UniqId` trait.
pub fn impl_uniq_id_trait(ast: &syn::DeriveInput, uniq_group: String) -> QuoteTokenStream {
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
pub fn impl_serde_traits(ast: &syn::DeriveInput) -> QuoteTokenStream {
    match ast.data {
        Enum(ref de) => impl_serde_traits_enum(ast, de),
        Struct(ref ds) => impl_serde_traits_struct(ast, ds),
        _ => panic!("macro only runnable on structs"),
    }
}

fn impl_serde_traits_enum(ast: &syn::DeriveInput, data_enum: &DataEnum) -> QuoteTokenStream {
    // Find a large enough type to identify all variants in this enum.
    let num_variants = data_enum.variants.len();
    let tag_type = match calc_enum_discriminant_type(num_variants) {
        Some(tt) => tt,
        None => panic!("empty enums not supported"),
    };

    // Iterate through enum fields and build method bodies.
    let mut ser_body = quote!{};
    let mut deser_body = quote!{};
    for (variant_num, variant) in (0..num_variants).zip(data_enum.variants.iter()) {
        if variant.discriminant.is_some() {
            panic!("enum discriminants not supported");
        }

        let enum_ident = &ast.ident;
        let variant_ident = &variant.ident;
        if variant.fields.iter().next().is_some() {
            panic!("fields on enum variants not supported");
        }
        ser_body.extend(quote! {
            #enum_ident::#variant_ident => {
                result.append(&mut (#variant_num as #tag_type).serialize());
            },
        });
        deser_body.extend(quote! {
            #variant_num => {
                #enum_ident::#variant_ident
            },
        });
    }

    // Generate trait implementations.
    let type_name = &ast.ident;
    quote! {
        impl ::serde::Serialize for #type_name {
            fn serialize(&self) -> ::std::vec::Vec<u8> {
                let mut result: Vec<u8> = Vec::new();
                match *self {
                    #ser_body
                };
                result
            }
        }

        impl ::serde::Deserialize for #type_name {
            fn deserialize(data: &[u8]) -> (usize, Self) {
                let (bytes_read, variant_num) = <#tag_type>::deserialize(data);
                let variant = match (variant_num as usize) {
                    #deser_body
                    _ => panic!("invalid ID for enum variant (ID was {})", variant_num),
                };
                (bytes_read, variant)
            }
        }
    }
}

/// Returns the number of bytes required to tag a union with `num_variants` variants.  If the enum
/// has no variants, returns `None`.
fn calc_enum_discriminant_type(num_variants: usize) -> Option<QuoteTokenStream> {
    match num_bytes_required(num_variants as u64) {
        0 => None,
        1 => Some(quote!{ u8 }),
        2 => Some(quote!{ u16 }),
        3 | 4 => Some(quote!{ u32 }),
        5 | 6 | 7 | 8 => Some(quote!{ u64 }),
        _ => panic!("impossibru!"),
    }
}

fn num_bytes_required(val: u64) -> usize {
    for shift in (0..64).rev() {
        if (val >> shift) & 1 == 1 {
            return ((shift / 8) + 1) as usize;
        }
    }
    0
}


fn impl_serde_traits_struct(ast: &syn::DeriveInput,
                            data_struct: &DataStruct) -> QuoteTokenStream {
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

#[cfg(test)]
mod tests {
    #[test]
    fn zero_bytes_required() {
        assert_eq!(super::num_bytes_required(0), 0);
    }

    #[test]
    fn one_byte_required_low() {
        assert_eq!(super::num_bytes_required(1), 1);
    }

    #[test]
    fn one_byte_required_high() {
        assert_eq!(super::num_bytes_required(255), 1);
    }

    #[test]
    fn two_bytes_required_low() {
        assert_eq!(super::num_bytes_required(256), 2);
    }

    #[test]
    fn two_bytes_required_high() {
        assert_eq!(super::num_bytes_required(511), 2);
    }
}
