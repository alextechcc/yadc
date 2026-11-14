//! `#[derive(Unit)]`: `Unit<Q, N>` for an enum of unit types - a choice of
//! units, for a menu, a config file, a parser - each variant naming the unit
//! type it is (`#[unit(Mile)]`), plus `Display` and `FromStr` from those
//! types' own `Display`.

use crate::paths::Paths;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::ParseStream;
use syn::{Data, DeriveInput, Fields, Token, Type};

/// The extras `#[derive(Unit)]` writes beside `Unit`: all of them by default,
/// or exactly those `#[unit(impl(..))]` lists.
const EXTRAS: &[&str] = &["Display", "FromStr"];

pub(crate) fn expand_unit(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let paths = Paths::parse(input)?;
    let yadc = &paths.yadc;
    let name = &input.ident;
    let Data::Enum(e) = &input.data else {
        return Err(syn::Error::new_spanned(
            name,
            "#[derive(Unit)] is for an enum of unit types (`#[unit(Mile)] Mile`); a unit type \
             of its own has a hand-written `impl Unit`",
        ));
    };
    if !input.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &input.generics,
            "an enum of units has no generics of its own: the impl's are `#[unit(for<..>)]`",
        ));
    }

    let mut top = Top::default();
    for attr in input.attrs.iter().filter(|a| a.path().is_ident("unit")) {
        attr.parse_args_with(|s: ParseStream<'_>| parse_top(s, &mut top))?;
    }
    if top.of.is_empty() {
        return Err(syn::Error::new_spanned(
            name,
            "say what these units measure and in which numbers: `#[unit(of = Q, number = N)]`",
        ));
    }
    let Some(number) = &top.number else {
        return Err(syn::Error::new_spanned(
            name,
            "say which numbers these units count in: `#[unit(number = N)]` (with `for<N>` to stay \
             generic)",
        ));
    };

    // Each variant and the unit type it is.
    let mut cases: Vec<(syn::Ident, Type)> = Vec::new();
    for v in &e.variants {
        if !matches!(v.fields, Fields::Unit) {
            return Err(syn::Error::new_spanned(
                v,
                "a variant of a unit enum has no fields: it names a unit type, `#[unit(Mile)]`",
            ));
        }
        let mut ty = None;
        for attr in v.attrs.iter().filter(|a| a.path().is_ident("unit")) {
            if ty.replace(attr.parse_args::<Type>()?).is_some() {
                return Err(syn::Error::new_spanned(attr, "a variant is one unit type"));
            }
        }
        let ty = ty.ok_or_else(|| {
            syn::Error::new_spanned(&v.ident, "name the unit type this variant is: `#[unit(Mile)]`")
        })?;
        cases.push((v.ident.clone(), ty));
    }

    let generics = top.generics.clone().unwrap_or_default();
    let (gen_impl, _, _) = generics.split_for_impl();
    let user_preds = generics.where_clause.as_ref().map(|w| w.predicates.iter().collect::<Vec<_>>()).unwrap_or_default();

    let mut out = TokenStream2::new();
    for q in &top.of {
        let tr = quote! { #yadc::Unit<#q, #number> };
        let build = cases.iter().map(|(v, t)| {
            let t = value_of(t);
            quote! { #name::#v => <_ as #tr>::quantity(&#t, value), }
        });
        let read = cases.iter().map(|(v, t)| {
            let t = value_of(t);
            quote! { #name::#v => <_ as #tr>::value(&#t, quantity), }
        });
        out.extend(quote! {
            const impl #gen_impl #tr for #name where #(#user_preds,)* {
                fn quantity(&self, value: #number) -> #q {
                    match self { #(#build)* }
                }
                fn value(&self, quantity: #q) -> #number {
                    match self { #(#read)* }
                }
            }
        });
    }

    let wants = |x: &str| top.impls.as_ref().is_none_or(|l| l.iter().any(|i| i == x));
    if wants("Display") {
        let arms = cases.iter().map(|(v, t)| {
            let t = value_of(t);
            quote! { #name::#v => ::core::fmt::Display::fmt(&#t, f), }
        });
        out.extend(quote! {
            impl ::core::fmt::Display for #name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self { #(#arms)* }
                }
            }
        });
    }
    if wants("FromStr") {
        let tries = cases.iter().map(|(v, t)| {
            let t = value_of(t);
            quote! { if shows(&#t, s) { return ::core::result::Result::Ok(#name::#v); } }
        });
        out.extend(quote! {
            impl ::core::str::FromStr for #name {
                type Err = #yadc::UnknownUnit;
                /// The first unit whose `Display` is `s`.
                fn from_str(s: &str) -> ::core::result::Result<Self, #yadc::UnknownUnit> {
                    // Whether `u` prints as `s`: compared as it is written, no allocation.
                    struct Rest<'a>(&'a str, bool);
                    impl ::core::fmt::Write for Rest<'_> {
                        fn write_str(&mut self, part: &str) -> ::core::fmt::Result {
                            match self.0.strip_prefix(part) {
                                ::core::option::Option::Some(r) => self.0 = r,
                                ::core::option::Option::None => self.1 = false,
                            }
                            ::core::result::Result::Ok(())
                        }
                    }
                    fn shows(u: &dyn ::core::fmt::Display, s: &str) -> bool {
                        let mut rest = Rest(s, true);
                        let _ = ::core::fmt::write(&mut rest, ::core::format_args!("{}", u));
                        rest.1 && rest.0.is_empty()
                    }
                    #(#tries)*
                    ::core::result::Result::Err(#yadc::UnknownUnit)
                }
            }
        });
    }
    Ok(out)
}

/// The value of a unit type: `Mile {}`, `SIUnit::<A, B> {}`.
fn value_of(t: &Type) -> TokenStream2 {
    match t {
        Type::Path(p) => {
            let mut p = p.clone();
            for seg in &mut p.path.segments {
                if let syn::PathArguments::AngleBracketed(a) = &mut seg.arguments {
                    a.colon2_token = Some(Default::default());
                }
            }
            quote! { #p {} }
        }
        other => quote! { <#other>::default() },
    }
}

/// The enum's own `#[unit(..)]`.
#[derive(Default)]
struct Top {
    of: Vec<Type>,
    number: Option<Type>,
    /// `for<N> where ..`: the impls are generic.
    generics: Option<syn::Generics>,
    impls: Option<Vec<String>>,
}

fn parse_top(input: ParseStream<'_>, top: &mut Top) -> syn::Result<()> {
    while !input.is_empty() {
        if input.peek(Token![for]) {
            // `for<N> where ..`: the rest of the attribute, as a where clause
            // takes everything after it.
            let kw = input.parse::<Token![for]>()?;
            let mut generics: syn::Generics = input.parse()?;
            if input.peek(Token![where]) {
                generics.where_clause = Some(input.parse()?);
            }
            if top.generics.replace(generics).is_some() {
                return Err(syn::Error::new_spanned(kw, "`for<..>` given twice"));
            }
        } else if input.peek(Token![impl]) {
            input.parse::<Token![impl]>()?;
            let content;
            syn::parenthesized!(content in input);
            let list = content.parse_terminated(<syn::Ident as syn::parse::Parse>::parse, Token![,])?;
            let impls = top.impls.get_or_insert_with(Vec::new);
            for id in list {
                let s = id.to_string();
                if !EXTRAS.contains(&s.as_str()) {
                    return Err(syn::Error::new_spanned(
                        &id,
                        format!("`{s}` is not an extra of `#[derive(Unit)]`; expected one of {}", EXTRAS.join(", ")),
                    ));
                }
                impls.push(s);
            }
        } else {
            let key = <syn::Ident as syn::ext::IdentExt>::parse_any(input)?;
            match key.to_string().as_str() {
                "of" => {
                    input.parse::<Token![=]>()?;
                    top.of.push(input.parse()?);
                }
                "number" => {
                    input.parse::<Token![=]>()?;
                    if top.number.replace(input.parse()?).is_some() {
                        return Err(syn::Error::new_spanned(&key, "`number` given twice: units count in one number type"));
                    }
                }
                other => {
                    return Err(syn::Error::new_spanned(
                        &key,
                        format!(
                            "unknown `#[unit(..)]` item `{other}`; expected `of = ..`, `number = ..`, \
                             `impl(..)`, `for<..> where ..`"
                        ),
                    ));
                }
            }
        }
        if input.is_empty() {
            break;
        }
        input.parse::<Token![,]>()?;
    }
    Ok(())
}
