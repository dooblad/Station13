use syn::{DataEnum, Ident};

use crate::QuoteTokenStream;

/// Generates implementations for `enum_tag` and `num_variants`.
pub fn impl_enum_methods(ast: &syn::DeriveInput, data_enum: &DataEnum) -> QuoteTokenStream {
    let enum_ident = &ast.ident;
    let enum_tag_impl = impl_enum_tag(enum_ident, data_enum);
    let num_variants_impl = impl_num_variants(data_enum);

    quote! {
        impl #enum_ident {
            #enum_tag_impl
            #num_variants_impl
        }
    }
}

/// Generates an implementation of `enum_tag` (a method returning the tag value for the current
/// variant).
pub fn impl_enum_tag(enum_ident: &Ident, data_enum: &DataEnum) -> QuoteTokenStream {
    // Find a large enough type to identify all variants in this enum.
    let num_variants = data_enum.variants.len();
    let tag_type = calc_enum_tag_type(num_variants).unwrap();

    // Iterate through enum fields and build match arms.
    let mut match_arms = quote!{};
    for (variant_num, variant) in (0..num_variants).zip(data_enum.variants.iter()) {
        let variant_ident = &variant.ident;

        match_arms.extend(quote! {
            #enum_ident::#variant_ident { .. } => (#variant_num as #tag_type),
        });
    }

    quote! {
        fn enum_tag(&self) -> #tag_type {
            match *self {
                #match_arms
            }
        }
    }
}

/// Generates an implementation of `num_variants` (a method returning the tag value for the
/// current variant).
pub fn impl_num_variants(data_enum: &DataEnum) -> QuoteTokenStream {
    let num_variants = data_enum.variants.len();
    quote! {
        fn num_variants() -> usize {
            #num_variants
        }
    }
}

/// Returns the appropriate uint type token to tag an enum with `num_variants` variants.  If the
/// enum has no variants, returns `None`.
pub fn calc_enum_tag_type(num_variants: usize) -> Option<QuoteTokenStream> {
    match num_bytes_required(num_variants as u64) {
        1 => Some(quote!{ u8 }),
        2 => Some(quote!{ u16 }),
        3 | 4 => Some(quote!{ u32 }),
        5 | 6 | 7 | 8 => Some(quote!{ u64 }),
        _ => unreachable!(),
    }
}

/// Returns the number of bytes required to store `val`.
fn num_bytes_required(val: u64) -> usize {
    for shift in (0..64).rev() {
        if (val >> shift) & 1 == 1 {
            return ((shift / 8) + 1) as usize;
        }
    }
    0
}

