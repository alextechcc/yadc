//! `#[derive(Unit)]`: `Unit<Q, N>` for a type of yours - a unit struct, a
//! struct whose fields are the unit's parameters, or an enum of units -
//! by naming the unit value each one is (`by = KILOMETRE`, `by = CELSIUS`),
//! plus `Display` and `FromStr` from the unit's symbol and name.

use crate::paths::Paths;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::ParseStream;
use syn::{Data, DataEnum, DeriveInput, Expr, Fields, Token, Type};

/// The extras `#[derive(Unit)]` writes beside `Unit`: all of them by default,
/// or exactly those `#[unit(impl(..))]` lists.
const EXTRAS: &[&str] = &["Display", "FromStr", "Fn"];

pub(crate) fn expand_unit(input: &DeriveInput) -> syn::Result<TokenStream2> {
    if let Data::Enum(e) = &input.data
        && wraps_one_each(e)
    {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "an enum whose variants each wrap a unit type is a unit set, not a unit: write its \
             `match`es by hand; `#[derive(Unit)]` is for an enum of units",
        ));
    }
    let __paths = Paths::parse(input)?;
    let yadc = &__paths.yadc;
    let name = &input.ident;

    // The type's own `#[unit(..)]`: options, and - on a struct - the unit.
    let mut top = Spec::default();
    for attr in input.attrs.iter().filter(|a| a.path().is_ident("unit")) {
        attr.parse_args_with(|s: ParseStream<'_>| parse_items(s, &mut top, Place::Type))?;
    }

    // One `Unit` per struct, or per variant: its pattern and its description.
    let mut units: Vec<UnitCase> = Vec::new();
    match &input.data {
        Data::Struct(s) => {
            let mut spec = top.unit_part();
            let by = by_field(&s.fields, &mut spec)?;
            units.push(UnitCase::new(quote! { #name }, &s.fields, by, spec, &input.ident)?);
        }
        Data::Enum(e) => {
            if top.has_unit_part() {
                return Err(syn::Error::new_spanned(
                    name,
                    "an enum describes each unit on its variant: `#[unit(\"symbol\", \"name\", \
                     by = ..)]`; the enum's own `#[unit(..)]` takes `of`, `number`, \
                     `non_const` and `impl(..)`",
                ));
            }
            for variant in &e.variants {
                let mut spec = Spec::default();
                for attr in variant.attrs.iter().filter(|a| a.path().is_ident("unit")) {
                    attr.parse_args_with(|s: ParseStream<'_>| {
                        parse_items(s, &mut spec, Place::Variant)
                    })?;
                }
                let by = by_field(&variant.fields, &mut spec)?;
                let ident = &variant.ident;
                units.push(UnitCase::new(
                    quote! { #name::#ident },
                    &variant.fields,
                    by,
                    spec,
                    ident,
                )?);
            }
        }
        Data::Union(_) => {
            return Err(syn::Error::new_spanned(
                name,
                "#[derive(Unit)] works on a struct or an enum",
            ));
        }
    }

    if top.of.is_empty() {
        return Err(syn::Error::new_spanned(
            name,
            "say what this unit measures: `#[unit(of = Q)]` (repeatable)",
        ));
    }
    let number = top.number.clone().unwrap_or_else(|| syn::parse_quote!(f64));
    // `const` unless `#[unit(non_const)]` says a delegated-to value's impl
    // is not.
    let konst = !top.non_const;
    let c = if konst {
        quote! { const }
    } else {
        quote! {}
    };

    let mut out = TokenStream2::new();
    for q in &top.of {
        let tr = quote! { #yadc::Unit<#q, #number> };
        let mut to_arms = Vec::new();
        let mut from_arms = Vec::new();
        for u in &units {
            let pat = &u.bind;
            let by = &u.by;
            // `via = P`: the unit value measures `P`, and `Q` converts to and
            // from it with std `From` (a level's scale reading a power).
            let (to, from) = match &u.via {
                None => (
                    quote! { <_ as #tr>::quantity(#by, value) },
                    quote! { <_ as #tr>::value(#by, quantity) },
                ),
                Some(p) => {
                    let via = quote! { #yadc::Unit<#p, #number> };
                    (
                        quote! {
                            <#q as ::core::convert::From<#p>>::from(<_ as #via>::quantity(#by, value))
                        },
                        quote! {
                            <_ as #via>::value(#by, <#p as ::core::convert::From<#q>>::from(quantity))
                        },
                    )
                }
            };
            to_arms.push(quote! { #pat => #to, });
            from_arms.push(quote! { #pat => #from, });
        }
        // No `where` clause: `Q` and `N` are concrete, so each unit's
        // description is checked right here.
        out.extend(quote! {
            #[allow(unused_variables)]
            #c impl #tr for #name {
                fn quantity(&self, value: #number) -> #q {
                    match self { #(#to_arms)* }
                }
                fn value(&self, quantity: #q) -> #number {
                    match self { #(#from_arms)* }
                }
            }
        });
    }

    let wants = |x: &str| match &top.impls {
        None => true,
        Some(list) => list.iter().any(|i| i == x),
    };
    // A unit is called like the function it is: on a quantity it reads; on a
    // number it builds, when it measures one type (several would each be a
    // possible result). The three std function traits, each over `Unit`.
    if wants("Fn") {
        let mut directions: Vec<(TokenStream2, TokenStream2, TokenStream2)> = Vec::new();
        // A unit of the numbers themselves has one argument type for both
        // directions, so it is not called.
        let same = |q: &Type| quote! { #q }.to_string() == quote! { #number }.to_string();
        for q in top.of.iter().filter(|q| !same(q)) {
            let tr = quote! { #yadc::Unit<#q, #number> };
            directions.push((
                quote! { #q },
                quote! { #number },
                quote! { <#name as #tr>::value(this, x) },
            ));
        }
        if let [q] = top.of.as_slice()
            && !same(q)
        {
            let tr = quote! { #yadc::Unit<#q, #number> };
            directions.push((
                quote! { #number },
                quote! { #q },
                quote! { <#name as #tr>::quantity(this, x) },
            ));
        }
        for (arg, ret, call) in directions {
            out.extend(quote! {
                #c impl ::core::ops::FnOnce<(#arg,)> for #name {
                    type Output = #ret;
                    extern "rust-call" fn call_once(self, (x,): (#arg,)) -> #ret {
                        let this = &self;
                        #call
                    }
                }
                #c impl ::core::ops::FnMut<(#arg,)> for #name {
                    extern "rust-call" fn call_mut(&mut self, (x,): (#arg,)) -> #ret {
                        let this = &*self;
                        #call
                    }
                }
                #c impl ::core::ops::Fn<(#arg,)> for #name {
                    extern "rust-call" fn call(&self, (x,): (#arg,)) -> #ret {
                        let this = self;
                        #call
                    }
                }
            });
        }
    }
    let labelled = |what: &str| -> syn::Result<Vec<(&UnitCase, &(String, String))>> {
        units
            .iter()
            .map(|u| {
                u.labels.as_ref().map(|l| (u, l)).ok_or_else(|| {
                    syn::Error::new_spanned(
                        &u.ident,
                        format!(
                            "`{what}` needs this unit's symbol and name: `#[unit(\"km\", \
                             \"kilometres\", ..)]` - or leave `{what}` out with \
                             `#[unit(impl(..))]`"
                        ),
                    )
                })
            })
            .collect()
    };
    if wants("Display") {
        // `{}` is the symbol, `{:#}` the name, through `Formatter::pad` so
        // width / fill / alignment apply (`{:>6}`) - no allocation, `no_std`.
        let arms = labelled("Display")?.into_iter().map(|(u, (sym, long))| {
            let pat = &u.bare;
            quote! { #pat => if f.alternate() { #long } else { #sym }, }
        });
        out.extend(quote! {
            impl ::core::fmt::Display for #name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.pad(match self { #(#arms)* })
                }
            }
        });
    }
    // `FromStr` parses a symbol back to its unit: by default only when every
    // unit can be built from its name alone (no fields).
    let fieldless = units.iter().all(|u| u.fieldless);
    let explicit_from_str = top
        .impls
        .as_ref()
        .is_some_and(|l| l.iter().any(|i| i == "FromStr"));
    if explicit_from_str && !fieldless {
        return Err(syn::Error::new_spanned(
            name,
            "`FromStr` builds a unit from its symbol alone, so no unit may have fields",
        ));
    }
    if wants("FromStr") && fieldless {
        let arms = labelled("FromStr")?.into_iter().map(|(u, (sym, _))| {
            let ctor = &u.ctor;
            quote! { #sym => ::core::result::Result::Ok(#ctor), }
        });
        out.extend(quote! {
            impl ::core::str::FromStr for #name {
                type Err = #yadc::UnknownUnit;
                // Two units sharing a symbol: the first wins.
                #[allow(unreachable_patterns)]
                fn from_str(s: &str) -> ::core::result::Result<Self, #yadc::UnknownUnit> {
                    match s {
                        #(#arms)*
                        _ => ::core::result::Result::Err(#yadc::UnknownUnit),
                    }
                }
            }
        });
    }
    Ok(out)
}

/// An enum each of whose variants wraps exactly one field, not marked
/// `#[unit(by)]`: the shape of a unit set (`enum Units {
/// Pressure(PressureUnit), .. }`), not of a unit.
fn wraps_one_each(e: &DataEnum) -> bool {
    !e.variants.is_empty()
        && e.variants.iter().all(|v| {
            matches!(&v.fields, Fields::Unnamed(f) if f.unnamed.len() == 1
                && !f.unnamed[0].attrs.iter().any(|a| a.path().is_ident("unit")))
        })
}

/// Where a `#[unit(..)]` is written, for what it may say.
#[derive(Clone, Copy, PartialEq)]
enum Place {
    /// On the struct or enum.
    Type,
    /// On an enum variant.
    Variant,
}

/// Where the unit value comes from: a constant expression, or a field of the
/// unit marked `#[unit(by)]`.
#[derive(Clone)]
enum Source {
    Expr(Expr),
    Field,
}

/// Everything one or more `#[unit(..)]`s said.
#[derive(Default, Clone)]
struct Spec {
    labels: Option<(String, String)>,
    /// `by = ..` or a `#[unit(by)]` field: the unit value this unit is.
    by: Option<Source>,
    /// `via = P`: the type the unit value measures, converted with `From`.
    via: Option<Type>,
    // Type-level options.
    of: Vec<Type>,
    number: Option<Type>,
    non_const: bool,
    impls: Option<Vec<String>>,
}

impl Spec {
    /// The part of a struct's own `#[unit(..)]` that describes its unit.
    fn unit_part(&self) -> Spec {
        Spec {
            labels: self.labels.clone(),
            by: self.by.clone(),
            via: self.via.clone(),
            ..Spec::default()
        }
    }
    fn has_unit_part(&self) -> bool {
        self.labels.is_some() || self.by.is_some() || self.via.is_some()
    }
    fn set_by(&mut self, key: &syn::Ident, source: Source) -> syn::Result<()> {
        if self.by.replace(source).is_some() {
            return Err(syn::Error::new_spanned(key, "`by` given twice: a unit is one unit value"));
        }
        Ok(())
    }
}

/// Parse one `#[unit(..)]`'s items into `spec`.
fn parse_items(input: ParseStream<'_>, spec: &mut Spec, place: Place) -> syn::Result<()> {
    let mut strings: Vec<syn::LitStr> = Vec::new();
    while !input.is_empty() {
        if input.peek(syn::LitStr) {
            strings.push(input.parse()?);
        } else if input.peek(Token![impl]) {
            let kw = input.parse::<Token![impl]>()?;
            if place != Place::Type {
                return Err(syn::Error::new_spanned(kw, "`impl(..)` goes on the type"));
            }
            let content;
            syn::parenthesized!(content in input);
            let list =
                content.parse_terminated(<syn::Ident as syn::parse::Parse>::parse, Token![,])?;
            let impls = spec.impls.get_or_insert_with(Vec::new);
            for id in list {
                let s = id.to_string();
                if !EXTRAS.contains(&s.as_str()) {
                    return Err(syn::Error::new_spanned(
                        &id,
                        format!(
                            "`{s}` is not an extra of `#[derive(Unit)]`; expected one of {}",
                            EXTRAS.join(", ")
                        ),
                    ));
                }
                impls.push(s);
            }
        } else {
            let key = <syn::Ident as syn::ext::IdentExt>::parse_any(input)?;
            let k = key.to_string();
            let type_only = |key: &syn::Ident| -> syn::Result<()> {
                if place == Place::Type {
                    Ok(())
                } else {
                    Err(syn::Error::new_spanned(
                        key,
                        format!("`{key}` goes on the enum, not on a variant"),
                    ))
                }
            };
            match k.as_str() {
                "of" => {
                    type_only(&key)?;
                    input.parse::<Token![=]>()?;
                    spec.of.push(input.parse()?);
                }
                "number" => {
                    type_only(&key)?;
                    input.parse::<Token![=]>()?;
                    if spec.number.replace(input.parse()?).is_some() {
                        return Err(syn::Error::new_spanned(
                            &key,
                            "`number` given twice: a unit's numbers have one type",
                        ));
                    }
                }
                "non_const" => {
                    type_only(&key)?;
                    spec.non_const = true;
                }
                "via" => {
                    input.parse::<Token![=]>()?;
                    if spec.via.replace(input.parse()?).is_some() {
                        return Err(syn::Error::new_spanned(&key, "`via` given twice"));
                    }
                }
                "by" => {
                    input.parse::<Token![=]>()?;
                    spec.set_by(&key, Source::Expr(input.parse()?))?;
                }
                other => {
                    return Err(syn::Error::new_spanned(
                        &key,
                        format!(
                            "unknown `#[unit(..)]` item `{other}`; expected the symbol and \
                             name, `by = ..`, `via = ..`, or on the \
                             type `of = ..`, `number = ..`, `non_const`, `impl(..)`"
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
    match strings.as_slice() {
        [] => {}
        [s, n] => {
            if spec.labels.replace((s.value(), n.value())).is_some() {
                return Err(syn::Error::new_spanned(
                    s,
                    "a unit has one symbol and one name",
                ));
            }
        }
        [s, ..] => {
            return Err(syn::Error::new_spanned(
                s,
                "a unit's labels are its symbol and its name: `#[unit(\"km\", \"kilometres\", ..)]`",
            ));
        }
    }
    Ok(())
}

/// The field of a struct or variant marked `#[unit(by)]`, if any, recorded
/// in `spec`. Other fields are the unit's own business.
fn by_field(fields: &Fields, spec: &mut Spec) -> syn::Result<Option<syn::Member>> {
    let mut out = None;
    for (i, f) in fields.iter().enumerate() {
        for attr in f.attrs.iter().filter(|a| a.path().is_ident("unit")) {
            let key: syn::Ident = attr.parse_args()?;
            if key != "by" {
                return Err(syn::Error::new_spanned(
                    &key,
                    "a field marked `#[unit(by)]` is the unit value this unit is",
                ));
            }
            spec.set_by(&key, Source::Field)?;
            out = Some(crate::util::member_of(i, f));
        }
    }
    Ok(out)
}

/// One unit of the type: the struct, or one variant.
struct UnitCase {
    ident: syn::Ident,
    /// The pattern binding its `#[unit(by)]` field (`Self { scale: __by, .. }`).
    bind: TokenStream2,
    /// The pattern binding nothing.
    bare: TokenStream2,
    /// The expression building it (fieldless units only).
    ctor: TokenStream2,
    fieldless: bool,
    labels: Option<(String, String)>,
    /// The unit value it is: a reference to a constant, or a field.
    by: TokenStream2,
    via: Option<Type>,
}

impl UnitCase {
    fn new(
        path: TokenStream2,
        fields: &Fields,
        by_field: Option<syn::Member>,
        spec: Spec,
        ident: &syn::Ident,
    ) -> syn::Result<Self> {
        let bind = match &by_field {
            Some(m) => quote! { #path { #m: __by, .. } },
            None => quote! { #path { .. } },
        };
        let bare = quote! { #path { .. } };
        let by = match &spec.by {
            Some(Source::Expr(e)) => quote! { &(#e) },
            Some(Source::Field) => quote! { __by },
            None => {
                return Err(syn::Error::new_spanned(
                    ident,
                    "say which unit value this is: `by = U` - a constant (`by = KILOMETRE`, a \
                     scale `by = CELSIUS`), or a field marked `#[unit(by)]`",
                ));
            }
        };
        Ok(UnitCase {
            ident: ident.clone(),
            ctor: path.clone(),
            fieldless: fields.is_empty(),
            bind,
            bare,
            labels: spec.labels,
            by,
            via: spec.via,
        })
    }
}
