use syn::{
    parse_macro_input,
    Attribute, Data, DataEnum, DeriveInput, Expr, Fields, Generics, Ident, Type, Variant,
};
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};

struct ConstEnumInfo {
    name: Ident,
    const_ty: Type,
    variants: Vec<VariantInfo>,
}

struct VariantInfo {
    name: Ident,
    const_val: ConstVal,
}

// Don't like the duplication between ConstVal and ConstEnumKind but it's pretty harmless
// and gets the job done, so seems like pointless to agonize over optimizing
enum ConstVal {
    Discrim,
    Assoc(Expr),
}

enum ConstEnumKind {
    Discrim,
    Assoc,
}

#[proc_macro_derive(TryFrom, attributes(const_ty, const_val))]
pub fn derive_try_from(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let ConstEnumInfo { name: enum_name, const_ty, variants } = unpack_input(input);

    let const_vals = variants.iter().map(|v| {
        let const_val_name = generated_const_name(&v.name);
        let expr = generate_const_val_expr(&enum_name, &const_ty, v);

        quote! {
            const #const_val_name: #const_ty = #expr;
        }
    });

    let match_arms = variants.iter().map(|v| {
        let variant_name = &v.name;
        let const_val_name = generated_const_name(&v.name);

        quote! {
            #const_val_name => Ok(#enum_name::#variant_name),
        }
    });

    let expanded = quote! {
        impl ::core::convert::TryFrom<#const_ty> for #enum_name
        where
            #const_ty: ::core::cmp::PartialEq + ::core::cmp::Eq,
        {
            type Error = #const_ty;

            #[allow(non_upper_case_globals)]
            fn try_from(val: #const_ty) -> Result<Self, Self::Error> {
                #(#const_vals)*

                match val {
                    #(#match_arms)*
                    other => Err(other),
                }
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(Into, attributes(const_ty, const_val))]
pub fn derive_into(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let ConstEnumInfo { name: enum_name, const_ty, variants } = unpack_input(input);
    let const_vals = variants.iter().map(|v| {
        let const_val_name = generated_const_name(&v.name);
        let expr = generate_const_val_expr(&enum_name, &const_ty, v);

        quote! {
            const #const_val_name: #const_ty = #expr;
        }
    });

    let match_arms = variants.iter().map(|v| {
        let variant_name = &v.name;
        let const_val_name = generated_const_name(&v.name);

        quote! {
            #enum_name::#variant_name => #const_val_name,
        }
    });

    let expanded = quote! {
        impl ::core::convert::Into<#const_ty> for #enum_name
        where
            #const_ty: ::core::cmp::PartialEq + ::core::cmp::Eq,
        {
            #[allow(non_upper_case_globals)]
            fn into(self) -> #const_ty {
                #(#const_vals)*

                match self {
                    #(#match_arms)*
                }
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(Display, attributes(const_ty))]
pub fn derive_display(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let ConstEnumInfo { name: enum_name, const_ty, .. } = unpack_input(input);
    let expanded = quote! {
        impl ::core::fmt::Display for #enum_name
        where
            #const_ty: ::core::fmt::Display,
            Self: ::core::convert::Into<#const_ty> + ::core::marker::Copy,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let val: #const_ty = (*self).into();
                write!(f, "{}", val)
            }
        }
    };

    expanded.into()
}

fn unpack_input(input: DeriveInput) -> ConstEnumInfo {
    let data = extract_and_verify_enum(input.data);

    if has_generics(&input.generics) {
        panic!("generic enums are not supported");
    }

    let (const_ty, const_kind) = extract_const_ty(input.attrs.into_iter());

    ConstEnumInfo {
        name: input.ident,
        const_ty,
        variants: extract_variants(data.variants.into_iter(), const_kind),
    }
}

fn extract_and_verify_enum(data: Data) -> DataEnum {
    match data {
        Data::Enum(data) => {
            if data.variants.is_empty() {
                panic!("enum must have at least one variant");
            }

            data
        },

        _ => panic!("input must be an enum"),
    }
}

fn extract_const_ty(attrs: impl Iterator<Item = Attribute>) -> (Type, ConstEnumKind) {
    let mut result = None;

    for attr in attrs {
        if attr.path().is_ident("const_ty") {
            let const_ty = attr.parse_args().expect("failed to parse type from #[const_ty(...)] attribute");
            result = Some((const_ty, ConstEnumKind::Assoc));
        } else if attr.path().is_ident("repr") && result.is_none() {
                let repr_ty = attr.parse_args().expect("failed to parse type from #[repr(...)] attribute");
                result = Some((repr_ty, ConstEnumKind::Discrim));
        }
    }

    result.expect("missing either #[const_ty(...)] or #[repr(...)] attribute to specify associated constant type")
}

fn extract_variants(variants: impl Iterator<Item = Variant>, kind: ConstEnumKind) -> Vec<VariantInfo> {
    variants.map(|v| {
        if !is_fieldless(&v) {
            panic!("only fieldless enum variants are allowed");
        }

        VariantInfo {
            const_val: match kind {
                ConstEnumKind::Discrim => ConstVal::Discrim,
                ConstEnumKind::Assoc => {
                    let expr = v.attrs.into_iter()
                        .find_map(|attr| {
                            if attr.path().is_ident("const_val") {
                                let val = attr.parse_args()
                                    .expect("failed to parse constant value from #[const_val(...)] attribute");
                                Some(val)
                            } else {
                                None
                            }
                        })
                        .unwrap_or_else(|| panic!(
                            "missing associated constant value for variant {}",
                            v.ident,
                        ));

                    ConstVal::Assoc(expr)
                },
            },
            name: v.ident,
        }
    }).collect()
}

fn has_generics(generics: &Generics) -> bool {
    !generics.params.is_empty() || generics.where_clause.is_some()
}

fn is_fieldless(variant: &Variant) -> bool {
    match &variant.fields {
        Fields::Unit => true,
        Fields::Named(f) => f.named.is_empty(),
        Fields::Unnamed(f) => f.unnamed.is_empty(),
    }
}

fn generated_const_name(variant_name: &Ident) -> Ident {
    format_ident!("C_{}", variant_name)
}

// Couldn't think of a better way to return the result of a quote
fn generate_const_val_expr(enum_name: &Ident, const_ty: &Type, variant: &VariantInfo) -> impl ToTokens {
    match variant.const_val {
        ConstVal::Discrim => {
            let variant_name = &variant.name;

            quote! {
                #enum_name::#variant_name as #const_ty
            }
        },

        ConstVal::Assoc(ref expr) => quote! { 
            #expr 
        },
    }
}
