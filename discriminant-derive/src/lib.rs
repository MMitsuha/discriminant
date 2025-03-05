use proc_macro::TokenStream;
use proc_macro::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Ident, Meta, Token, punctuated::Punctuated};

/// Derive macro for `Discriminant<T>` trait
///
/// This macro will check whether if it is a enum and has `#[repr(...)]` attribute
#[proc_macro_derive(Discriminant)]
pub fn discriminant_derive(t: TokenStream) -> TokenStream {
    let ty = TokenStream2::from(t);
    let ast = syn::parse(ty).unwrap();
    let repr_type = find_repr_type(&ast).unwrap();

    ensure_enum_valid(&ast);

    impl_discriminant_macro(&ast, &repr_type)
}

fn impl_discriminant_macro(ast: &DeriveInput, repr_type: &Ident) -> TokenStream {
    let name = &ast.ident;
    let imp = quote! {
        impl Discriminant<#repr_type> for #name {
            fn discriminant(&self) -> #repr_type {
                // Should be safe here
                unsafe { *<*const #name>::from(self).cast::<#repr_type>() }
            }
        }
    };
    imp.into()
}

fn ensure_enum_valid(ast: &DeriveInput) {
    if let Data::Enum(data) = &ast.data {
        if data.variants.is_empty() == false {
            return;
        }

        panic!("Can't derive PrimitiveRepr on a zero variant enum");
    }

    panic!("Discriminant can only be derived for enums");
}

fn find_repr_type(ast: &DeriveInput) -> Option<Ident> {
    for meta in ast
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("repr"))
        .filter_map(|attr| {
            attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .ok()
        })
        .flatten()
    {
        if let Meta::Path(path) = meta {
            if let Some(ident) = path.get_ident() {
                return Some(ident.clone());
            }
        }
    }

    None
}
