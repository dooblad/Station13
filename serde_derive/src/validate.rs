use syn::Data::{Enum, Struct};
use syn::Fields::*;
use syn::Type::*;
use syn::{DataEnum, DataStruct, Fields};

// TODO: Just log errors and don't panic immediately.

/// Checks `ast` to ensure this proc macro is only used on valid constructs.  If any checks fail,
/// we panic and let the spaghetti spill out of our pockets.
pub fn validate(ast: &syn::DeriveInput) {
    match ast.data {
        Enum(ref de) => validate_enum(de),
        Struct(ref ds) => validate_struct(ds),
        _ => panic!("macro only runnable on enums and structs"),
    }
}

fn validate_enum(data_enum: &DataEnum) {
    if data_enum.variants.len() == 0 {
        panic!("empty enums not supported");
    }
    for variant in data_enum.variants.iter() {
        if variant.discriminant.is_some() {
            panic!("enum discriminants not supported");
        }
        validate_fields(&variant.fields);
    }
}

fn validate_struct(data_struct: &DataStruct) {
    validate_fields(&data_struct.fields);
}

fn validate_fields(fields: &Fields) {
    for field in fields.iter() {
        match field.ty {
            Slice(_) => panic!("slice types not allowed in struct/variant def"),
            Ptr(_) => panic!("pointer types not allowed in struct/variant def"),
            Reference(_) => panic!("reference types not allowed in struct/variant def"),
            BareFn(_) => panic!("bare function types not allowed in struct/variant def"),
            Never(_) => panic!("never types not allowed in struct/variant def"),
            Tuple(_) => panic!("tuple types not allowed in struct/variant def"),
            TraitObject(_) => panic!("dyn trait objects not allowed in struct/variant def"),
            ImplTrait(_) => panic!("impl trait objects not allowed in struct/variant def"),
            Paren(_) => panic!("parenthesized types not allowed in struct/variant def"),
            Group(_) => panic!("type groups not allowed in struct/variant def"),
            Infer(_) => panic!("underscore types not allowed in struct/variant def"),
            Macro(_) => panic!("macro types not allowed in struct/variant def"),
            Verbatim(_) => panic!("verbatim types not allowed in struct/variant def"),
            _ => (),
        };
    }
}
