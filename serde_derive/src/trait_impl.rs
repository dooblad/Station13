use std::cell::RefCell;
use std::collections::HashMap;

use crate::proc_macro2::{Ident, Span};

use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Data::{Enum, Struct};
use syn::Fields::*;
use syn::Type::*;
use syn::{DataEnum, DataStruct, Field, Fields};

type QuoteTokenStream = quote::__rt::TokenStream;

/// Generates implementations for the `Serialize` and `Deserialize` traits.
///
/// Assumption: people don't suffix struct field names with underscores.  For those who do,
/// kindly die.  :^)
pub fn impl_serde_traits(ast: &syn::DeriveInput) -> QuoteTokenStream {
    match ast.data {
        Enum(ref de) => impl_serde_traits_enum(ast, de),
        Struct(ref ds) => impl_serde_traits_struct(ast, ds),
        _ => panic!("macro only runnable on enums and structs"),
    }
}

fn impl_serde_traits_enum(ast: &syn::DeriveInput, data_enum: &DataEnum) -> QuoteTokenStream {
    // Find a large enough type to identify all variants in this enum.
    let num_variants = data_enum.variants.len();
    let enum_ident = &ast.ident;
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
        let variant_ident = &variant.ident;
        let (ser_deconstruct, deser_construct) = ser_deconstruct_deser_construct(
            &variant.fields,
            quote! { #enum_ident::#variant_ident },
        );

        let mut ser_arm_body = impl_ser_body(&variant.fields);
        let mut deser_arm_body = impl_deser_body(&variant.fields);
        ser_body.extend(quote! {
            #ser_deconstruct => {
                result_.append(&mut (#variant_num as #tag_type).serialize());
                #ser_arm_body
            },
        });
        deser_body.extend(quote! {
            #variant_num => {
                #deser_arm_body
                (bytes_read_, #deser_construct)
            },
        });
    }

    // Generate trait implementations.
    let type_name = &ast.ident;
    quote! {
        impl ::serde::Serialize for #type_name {
            fn serialize(&self) -> ::std::vec::Vec<u8> {
                let mut result_: Vec<u8> = Vec::new();
                match *self {
                    #ser_body
                };
                result_
            }
        }

        impl ::serde::Deserialize for #type_name {
            fn deserialize(data: &[u8]) -> (usize, Self) {
                // Anything that shadows `data` could cause issues.
                let data_ = data;
                let (mut bytes_read_, variant_num_) = <#tag_type>::deserialize(data_);
                match (variant_num_ as usize) {
                    #deser_body
                    _ => panic!("invalid ID for enum variant (ID was {})", variant_num_),
                }
            }
        }
    }
}

fn impl_serde_traits_struct(ast: &syn::DeriveInput, data_struct: &DataStruct) -> QuoteTokenStream {
    let mut ser_body = impl_ser_body(&data_struct.fields);
    let mut deser_body = impl_deser_body(&data_struct.fields);

    let struct_ident = &ast.ident;
    let (ser_deconstruct, deser_construct) = {
        let (ser_deconstruct, deser_construct) =
            ser_deconstruct_deser_construct(&data_struct.fields, quote! { #struct_ident });
        // Need to modify the results to tailor it to the struct use case.
        match data_struct.fields {
            Named(_) => (quote! { let #ser_deconstruct = self; }, deser_construct),
            Unnamed(_) => (quote! { let #ser_deconstruct = self; }, deser_construct),
            Unit => (quote!{}, quote! { #deser_construct { } }),
        }
    };

    // Generate trait implementations.
    quote! {
        impl ::serde::Serialize for #struct_ident {
            fn serialize(&self) -> ::std::vec::Vec<u8> {
                let mut result_: Vec<u8> = Vec::new();
                #ser_deconstruct
                #ser_body
                result_
            }
        }

        impl ::serde::Deserialize for #struct_ident {
            fn deserialize(data: &[u8]) -> (usize, Self) {
                // Anything that shadows `data` could cause issues.
                let data_ = data;
                let mut bytes_read_: usize = 0;
                #deser_body
                (bytes_read_, #deser_construct)
            }
        }
    }
}

// Helpers

// TODO: This name is progressing my cancer.
/// Returns a tuple of pattern deconstruction for serializing and struct/enum construction for
/// deserializing (to promote code reuse between the handling of structs and enums).
fn ser_deconstruct_deser_construct(
    fields: &Fields,
    type_ident: QuoteTokenStream,
) -> (QuoteTokenStream, QuoteTokenStream) {
    match fields {
        // Struct/Variant
        Named(_) => {
            let mut field_list: Punctuated<QuoteTokenStream, Comma> = Punctuated::new();
            let mut assign_list: Punctuated<QuoteTokenStream, Comma> = Punctuated::new();
            for field in fields.iter() {
                let field_ident = field.ident.clone().unwrap();
                let binding_ident = field_ident.to_internal_ident();
                field_list.push(quote!{ ref #field_ident });
                assign_list.push(quote!{ #field_ident: #binding_ident });
            }
            (
                quote! { #type_ident { #field_list } },
                quote! { #type_ident { #assign_list } },
            )
        }
        // Tuple Struct/Variant
        Unnamed(_) => {
            // For tuples, we can use the same `field_list` for deconstructing and for
            // constructing.
            let mut field_list: Punctuated<QuoteTokenStream, Comma> = Punctuated::new();
            for (i, _) in fields.iter().enumerate() {
                let field_ident = i.to_internal_ident();
                field_list.push(quote!{ #field_ident });
            }
            (
                quote! { #type_ident(#field_list) },
                quote! { #type_ident(#field_list) },
            )
        }
        // Unit Struct/Variant
        Unit => (quote! { #type_ident }, quote! { #type_ident }),
    }
}

fn impl_ser_body(fields: &Fields) -> QuoteTokenStream {
    match fields {
        Named(_) => impl_ser_body_named(fields),
        Unnamed(_) => impl_ser_body_unnamed(fields),
        Unit => quote!{},
    }
}

fn impl_ser_body_named(fields: &Fields) -> QuoteTokenStream {
    let mut result = quote!{};
    for field in fields.iter() {
        let ident = field.ident.clone().unwrap();
        result.extend(quote! {
            result_.append(&mut #ident.serialize());
        });
    }
    result
}

fn impl_ser_body_unnamed(fields: &Fields) -> QuoteTokenStream {
    let mut result = quote!{};
    for (i, _) in fields.iter().enumerate() {
        let field_binding_ident = i.to_internal_ident();
        result.extend(quote! {
            result_.append(&mut #field_binding_ident.serialize());
        });
    }
    result
}

fn impl_deser_body(fields: &Fields) -> QuoteTokenStream {
    match fields {
        Named(_) => impl_deser_body_named(fields),
        Unnamed(_) => impl_deser_body_unnamed(fields),
        Unit => quote!{},
    }
}

fn impl_deser_body_named(fields: &Fields) -> QuoteTokenStream {
    let mut result = quote!{};
    for field in fields.iter() {
        let field_ident = field.ident.clone().unwrap();
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
        let binding_ident = field_ident.to_internal_ident();
        result.extend(quote! {
            let deser_data_ = <#type_tokens>::deserialize(&data_[bytes_read_..]);
            bytes_read_ += deser_data_.0;
            let #binding_ident = deser_data_.1;
        });
    }
    result
}

fn impl_deser_body_unnamed(fields: &Fields) -> QuoteTokenStream {
    let mut result = quote!{};
    for (i, field) in fields.iter().enumerate() {
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
        let internal_ident = i.to_internal_ident();
        result.extend(quote! {
            let deser_data_ = <#type_tokens>::deserialize(&data_[bytes_read_..]);
            bytes_read_ += deser_data_.0;
            let #internal_ident = deser_data_.1;
        });
    }
    result
}

trait ToInternalIdent {
    // Adds an underscore suffix and, if the base type "starts with" a number, we also need a prefix
    // underscore, because an identifier can't start with a number.
    fn to_internal_ident(&self) -> Ident;
}

impl ToInternalIdent for usize {
    fn to_internal_ident(&self) -> Ident {
        Ident::new(&format!("_{}_", *self), Span::call_site())
    }
}

impl ToInternalIdent for Ident {
    fn to_internal_ident(&self) -> Ident {
        let base_ident_str = self.to_string();
        // Can't have an empty identifier so the `unwrap` can't fail.
        if base_ident_str.chars().next().unwrap().is_numeric() {
            Ident::new(&format!("_{}_", *self), Span::call_site())
        } else {
            Ident::new(&format!("{}_", *self), Span::call_site())
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
