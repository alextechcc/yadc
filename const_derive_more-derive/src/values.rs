//! `#[derive(Values)]`: an enum whose variants each name a value - a choice
//! among them, made at run time - implementing the traits it lists as its
//! variant's value does, printing and parsing as it, and converting through
//! it.

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::ParseStream;
use syn::{Data, DeriveInput, Expr, Fields, Ident, Token, Type};

/// What `#[values(..)]` on the enum lists.
#[derive(Default)]
struct Spec {
    /// `delegate(Trait { fn m(&self, ..) -> R; .. })`: a trait the enum
    /// implements as its variant's value, with the methods listed.
    delegates: Vec<(syn::Path, Vec<syn::Signature>)>,
    /// `into(X)`: `From<Enum> for X`, through `X::from(value)`.
    into: Vec<Type>,
    /// `for<..> where ..`: the impls' generics and bounds.
    generics: Option<syn::Generics>,
    /// `impl(..)`: which of `Display`, `FromStr` to write; both by default.
    impls: Option<Vec<String>>,
}

const EXTRAS: &[&str] = &["Display", "FromStr"];

pub(crate) fn expand(input: &DeriveInput, krate: &syn::Path) -> syn::Result<TokenStream2> {
    let name = &input.ident;
    let Data::Enum(e) = &input.data else {
        return Err(syn::Error::new_spanned(
            name,
            "#[derive(Values)] is for an enum whose variants each name a value",
        ));
    };
    if !input.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &input.generics,
            "an enum of values has no generics of its own: the impls' are `#[values(for<..>)]`",
        ));
    }
    let mut spec = Spec::default();
    for attr in input.attrs.iter().filter(|a| a.path().is_ident("values")) {
        attr.parse_args_with(|s: ParseStream<'_>| parse_spec(s, &mut spec))?;
    }
    let mut cases: Vec<(Ident, Expr)> = Vec::new();
    for v in &e.variants {
        if !matches!(v.fields, Fields::Unit) {
            return Err(syn::Error::new_spanned(
                v,
                "a variant of an enum of values has no fields: it names a value, `#[value(MILE)]`",
            ));
        }
        let mut value = None;
        for attr in v.attrs.iter().filter(|a| a.path().is_ident("value")) {
            if value.replace(attr.parse_args::<Expr>()?).is_some() {
                return Err(syn::Error::new_spanned(attr, "a variant names one value"));
            }
        }
        let value = value.ok_or_else(|| {
            syn::Error::new_spanned(&v.ident, "name the value this variant is: `#[value(MILE)]`")
        })?;
        cases.push((v.ident.clone(), value));
    }

    let generics = spec.generics.clone().unwrap_or_default();
    let (gen_impl, _, _) = generics.split_for_impl();
    let preds: Vec<_> = generics
        .where_clause
        .as_ref()
        .map(|w| w.predicates.iter().collect())
        .unwrap_or_default();

    let mut out = TokenStream2::new();
    for (tr, sigs) in &spec.delegates {
        let mut methods = TokenStream2::new();
        for sig in sigs {
            let m = &sig.ident;
            let receiver = match sig.inputs.first() {
                Some(syn::FnArg::Receiver(r))
                    if r.reference.is_some() && r.mutability.is_none() =>
                {
                    quote! { & }
                }
                Some(syn::FnArg::Receiver(r)) if r.reference.is_none() => quote! {},
                _ => {
                    return Err(syn::Error::new_spanned(
                        sig,
                        "a delegated method takes `&self` or `self`: the variant's value is a constant",
                    ));
                }
            };
            let mut args = Vec::new();
            for arg in sig.inputs.iter().skip(1) {
                let syn::FnArg::Typed(t) = arg else {
                    unreachable!("one receiver")
                };
                let syn::Pat::Ident(id) = &*t.pat else {
                    return Err(syn::Error::new_spanned(
                        &t.pat,
                        "name each argument of a delegated method",
                    ));
                };
                args.push(id.ident.clone());
            }
            let arms = cases.iter().map(
                |(v, value)| quote! { #name::#v => <_ as #tr>::#m(#receiver #value, #(#args),*), },
            );
            methods.extend(quote! {
                #sig { match self { #(#arms)* } }
            });
        }
        out.extend(quote! {
            impl #gen_impl #tr for #name where #(#preds,)* { #methods }
        });
    }
    for x in &spec.into {
        let arms = cases.iter().map(
            |(v, value)| quote! { #name::#v => <#x as ::core::convert::From<_>>::from(#value), },
        );
        out.extend(quote! {
            impl ::core::convert::From<#name> for #x {
                fn from(e: #name) -> #x { match e { #(#arms)* } }
            }
        });
    }
    let wants = |x: &str| spec.impls.as_ref().is_none_or(|l| l.iter().any(|i| i == x));
    if wants("Display") {
        let arms = cases
            .iter()
            .map(|(v, value)| quote! { #name::#v => ::core::fmt::Display::fmt(&#value, f), });
        out.extend(quote! {
            impl ::core::fmt::Display for #name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self { #(#arms)* }
                }
            }
        });
    }
    if wants("FromStr") {
        let tries = cases.iter().map(|(v, value)| {
            quote! { if #krate::__prints_as(&#value, s) { return ::core::result::Result::Ok(#name::#v); } }
        });
        out.extend(quote! {
            impl ::core::str::FromStr for #name {
                type Err = #krate::NoValue;
                /// The first variant whose value prints as `s`.
                fn from_str(s: &str) -> ::core::result::Result<Self, #krate::NoValue> {
                    #(#tries)*
                    ::core::result::Result::Err(#krate::NoValue)
                }
            }
        });
    }
    Ok(out)
}

fn parse_spec(input: ParseStream<'_>, spec: &mut Spec) -> syn::Result<()> {
    while !input.is_empty() {
        if input.peek(Ident) && input.fork().parse::<Ident>().is_ok_and(|k| k == "delegate") {
            input.parse::<Ident>()?;
            let content;
            syn::parenthesized!(content in input);
            let tr: syn::Path = content.parse()?;
            let body;
            syn::braced!(body in content);
            let mut sigs = Vec::new();
            while !body.is_empty() {
                let f: syn::TraitItemFn = body.parse()?;
                if f.default.is_some() {
                    return Err(syn::Error::new_spanned(
                        &f.sig,
                        "a delegated method is a signature: `fn m(&self, ..) -> R;`",
                    ));
                }
                sigs.push(f.sig);
            }
            spec.delegates.push((tr, sigs));
        } else if input.peek(Token![for]) {
            let kw = input.parse::<Token![for]>()?;
            let mut generics: syn::Generics = input.parse()?;
            if input.peek(Token![where]) {
                generics.where_clause = Some(input.parse()?);
            }
            if spec.generics.replace(generics).is_some() {
                return Err(syn::Error::new_spanned(kw, "`for<..>` given twice"));
            }
        } else if input.peek(Token![impl]) {
            input.parse::<Token![impl]>()?;
            let content;
            syn::parenthesized!(content in input);
            let list = content.parse_terminated(Ident::parse, Token![,])?;
            let impls = spec.impls.get_or_insert_with(Vec::new);
            for id in list {
                let s = id.to_string();
                if !EXTRAS.contains(&s.as_str()) {
                    return Err(syn::Error::new_spanned(
                        &id,
                        format!(
                            "`{s}` is not an extra of `#[derive(Values)]`; expected one of {}",
                            EXTRAS.join(", ")
                        ),
                    ));
                }
                impls.push(s);
            }
        } else {
            let key: Ident = input.parse()?;
            if key != "into" {
                return Err(syn::Error::new_spanned(
                    &key,
                    "unknown `#[values(..)]` item; expected `delegate(Trait { fn m(&self, ..) -> R; })`, `into(X)`, `impl(..)`, `for<..> where ..`",
                ));
            }
            let content;
            syn::parenthesized!(content in input);
            spec.into
                .extend(content.parse_terminated(Type::parse, Token![,])?);
        }
        if input.is_empty() {
            break;
        }
        input.parse::<Token![,]>()?;
    }
    Ok(())
}

use syn::parse::Parse;
