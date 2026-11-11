//! Helpers shared by several derives.

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{GenericParam, Type};

/// Render one generic parameter declaration for an impl head, dropping this
/// crate's helper attributes (`#[dim]`, `#[value]`) and any default
/// (`Z = Free`), which an impl may not repeat.
pub(crate) fn param_decl(p: &GenericParam) -> TokenStream2 {
    let mut p = p.clone();
    match &mut p {
        GenericParam::Lifetime(l) => l.attrs.clear(),
        GenericParam::Type(t) => {
            t.attrs.clear();
            t.eq_token = None;
            t.default = None;
        }
        GenericParam::Const(c) => {
            c.attrs.clear();
            c.eq_token = None;
            c.default = None;
        }
    }
    quote! { #p }
}

/// `PhantomData<..>`, by its last path segment: a field that holds no data,
/// filled with `PhantomData` wherever a derive builds the struct.
pub(crate) fn is_phantom(ty: &Type) -> bool {
    matches!(ty, Type::Path(tp)
        if tp.qself.is_none()
            && tp.path.segments.last().is_some_and(|s| s.ident == "PhantomData"))
}

/// Whether `ty` is exactly one of the item's own type parameters (a generic
/// number type `N`): then an impl *for* it is an orphan-rule violation.
pub(crate) fn is_own_type_param(ty: &Type, generics: &syn::Generics) -> bool {
    generics.params.iter().any(|p| {
        matches!(p, GenericParam::Type(t)
            if matches!(ty, Type::Path(tp) if tp.qself.is_none() && tp.path.is_ident(&t.ident)))
    })
}

/// Whether an attribute list carries `#[name]` / `#[name(..)]`.
pub(crate) fn has_attr(attrs: &[syn::Attribute], name: &str) -> bool {
    attrs.iter().any(|a| a.path().is_ident(name))
}

/// A field as a member expression: `value`, or `0` for a positional one.
pub(crate) fn member_of(index: usize, field: &syn::Field) -> syn::Member {
    match &field.ident {
        Some(id) => syn::Member::Named(id.clone()),
        None => syn::Member::Unnamed(syn::Index::from(index)),
    }
}

/// `#[dimension(impl(..))]` on a dimension or a label: which extras to write
/// (`None`: all of them - `Display`).
pub(crate) fn dimension_impls(attrs: &[syn::Attribute]) -> syn::Result<Option<Vec<String>>> {
    let mut out: Option<Vec<String>> = None;
    for attr in attrs.iter().filter(|a| a.path().is_ident("dimension")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("impl") {
                let list = out.get_or_insert_with(Vec::new);
                meta.parse_nested_meta(|item| {
                    if item.path.is_ident("Display") {
                        list.push("Display".to_string());
                        Ok(())
                    } else {
                        Err(item.error("`#[derive(Dimension)]`'s only extra is `Display`"))
                    }
                })
            } else {
                Err(meta.error("expected `impl(..)`: `#[dimension(impl(Display))]`"))
            }
        })?;
    }
    Ok(out)
}

/// `#[dimension(symbol = "kg")]` on an axis field: how `Display` names it.
pub(crate) fn axis_symbol(field: &syn::Field) -> syn::Result<Option<String>> {
    let mut out = None;
    for attr in field
        .attrs
        .iter()
        .filter(|a| a.path().is_ident("dimension"))
    {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("symbol") {
                let lit: syn::LitStr = meta.value()?.parse()?;
                out = Some(lit.value());
                Ok(())
            } else {
                Err(meta.error("expected `symbol = \"..\"`"))
            }
        })?;
    }
    Ok(out)
}

/// `const impl PartialPowi`: repeated `*` from dimension one (`self^0` is
/// `self / self`), each step read through `Into<Option<Self>>`, so a product
/// with no result (a point's) makes the power `None`.
pub(crate) fn partial_powi_impl(yadc: &syn::Path, name: &syn::Ident) -> TokenStream2 {
    let into = quote! { ::core::convert::Into::<::core::option::Option<Self>>::into };
    quote! {
        const impl #yadc::PartialPowi for #name {
            fn partial_powi(self, n: i8) -> ::core::option::Option<Self> {
                if n == 0 {
                    return #into(self / self);
                }
                let mut acc = <Self as #yadc::Dimension>::DIMENSIONLESS;
                let mut i = 0;
                while i < n.unsigned_abs() {
                    acc = #into(acc * self)?;
                    i += 1;
                }
                if n < 0 {
                    #into(<Self as #yadc::Dimension>::DIMENSIONLESS / acc)
                } else {
                    ::core::option::Option::Some(acc)
                }
            }
        }
    }
}
