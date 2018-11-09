use crate::proc_macro2::{Ident, Span};

use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Data::{Enum, Struct};
use syn::Fields::*;
use syn::Type::*;
use syn::{DataEnum, DataStruct, Fields};

use crate::QuoteTokenStream;
use crate::enum_impl::calc_enum_tag_type;

/// Generates implementations for the `Serialize` and `Deserialize` traits.
///
/// Assumption: people don't suffix struct field names with underscores.  For those who do,
/// kindly die.  :^)
pub fn impl_serde_traits(ast: &syn::DeriveInput) -> QuoteTokenStream {
    match ast.data {
        Enum(ref de) => impl_serde_traits_enum(ast, de),
        Struct(ref ds) => impl_serde_traits_struct(ast, ds),
        _ => unreachable!(),
    }
}

fn impl_serde_traits_enum(ast: &syn::DeriveInput, data_enum: &DataEnum) -> QuoteTokenStream {
    // Find a large enough type to identify all variants in this enum.
    let num_variants = data_enum.variants.len();
    let enum_ident = &ast.ident;

    // Iterate through enum fields and build method bodies.
    let mut ser_body = quote!{};
    let mut deser_body = quote!{};
    for (variant_num, variant) in (0..num_variants).zip(data_enum.variants.iter()) {
        let variant_ident = &variant.ident;
        let (ser_deconstruct, deser_construct) = ser_deconstruct_deser_construct(
            &variant.fields,
            quote! { #enum_ident::#variant_ident },
        );

        let ser_arm_body = impl_ser_body(&variant.fields);
        let deser_arm_body = impl_deser_body(&variant.fields);
        ser_body.extend(quote! {
            #ser_deconstruct => {
                result_.append(&mut self.enum_tag().serialize());
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
    let tag_type = calc_enum_tag_type(num_variants).unwrap();
    quote! {
        impl ::serde::Serialize for #enum_ident {
            fn serialize(&self) -> ::std::vec::Vec<u8> {
                let mut result_: Vec<u8> = Vec::new();
                match *self {
                    #ser_body
                };
                result_
            }
        }

        impl ::serde::Deserialize for #enum_ident {
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
    let ser_body = impl_ser_body(&data_struct.fields);
    let deser_body = impl_deser_body(&data_struct.fields);

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
            let mut pattern_list: Punctuated<QuoteTokenStream, Comma> = Punctuated::new();
            let mut arg_list: Punctuated<QuoteTokenStream, Comma> = Punctuated::new();
            for (i, _) in fields.iter().enumerate() {
                let field_ident = i.to_internal_ident();
                pattern_list.push(quote!{ ref #field_ident });
                arg_list.push(quote!{ #field_ident });
            }
            (
                quote! { #type_ident(#pattern_list) },
                quote! { #type_ident(#arg_list) },
            )
        }
        // Unit Struct/Variant
        Unit => (quote! { #type_ident }, quote! { #type_ident }),
    }
}

fn impl_ser_body(fields: &Fields) -> QuoteTokenStream {
    let mut result = quote!{};
    for (i, field) in fields.iter().enumerate() {
        let binding_ident = field.ident.clone().unwrap_or(i.to_internal_ident());
        result.extend(quote! {
            result_.append(&mut #binding_ident.serialize());
        });
    }
    result
}

fn impl_deser_body(fields: &Fields) -> QuoteTokenStream {
    let mut result = quote!{};
    for (i, field) in fields.iter().enumerate() {
        let type_tokens = match field.ty {
            Array(ref a) => {
                let elem_ty = &(*a.elem);
                let arr_len = &a.len;
                quote! { [#elem_ty; #arr_len] }
            }
            Path(ref p) => {
                quote! { #p }
            }
            _ => unreachable!(),
        };
        let binding_ident = (i, field.ident.clone()).to_internal_ident();
        result.extend(quote! {
            let deser_data_ = <#type_tokens>::deserialize(&data_[bytes_read_..]);
            bytes_read_ += deser_data_.0;
            let #binding_ident = deser_data_.1;
        });
    }
    result
}

trait ToInternalIdent {
    /// Adds an underscore suffix and, if the base type "starts with" a number, we also need a prefix
    /// underscore, because an identifier can't start with a number.
    ///
    /// Just for the record, I fucking hate this, but as long as our meta-variables are in the same
    /// environment as base-level variables, idk how else to go about this.
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

// For abstracting over whether we're generating bindings for a named-field struct or an
// unnamed-field (tuple) struct.
impl ToInternalIdent for (usize, Option<Ident>) {
    fn to_internal_ident(&self) -> Ident {
        match self.1 {
            Some(ref id) => id.to_internal_ident(),
            None => self.0.to_internal_ident(),
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
