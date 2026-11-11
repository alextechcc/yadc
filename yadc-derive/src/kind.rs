//! `#[derive(Dimension)]` on an enum - a label (a kind) - and
//! `#[derive(UcumKind)]`: the label's algebra tables and its UCUM prefix
//! mapping.

use crate::paths::Paths;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Token, parse::ParseStream};

/// A table entry meaning "no result": the combination is rejected (`None`).
const NO_KIND: usize = usize::MAX;

/// The operations a label declaration can talk about: `+` and `-`.
/// Products and quotients are fixed (a label survives only beside a plain
/// number), so they are not declared.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Op {
    Mul,
    Div,
    Add,
    Sub,
}

impl Op {
    fn parse(id: &syn::Ident) -> Option<Self> {
        match id.to_string().as_str() {
            "add" => Some(Op::Add),
            "sub" => Some(Op::Sub),
            _ => None,
        }
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// How a declaration names a label: "no result", or a variant by name.
enum KindRef {
    Invalid,
    Named(syn::Ident),
}

impl syn::parse::Parse for KindRef {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let id: syn::Ident = input.parse()?;
        Ok(if id == "invalid" {
            KindRef::Invalid
        } else {
            KindRef::Named(id)
        })
    }
}

/// One item inside a variant's `#[kind(..)]`: `add(Other) = Result` - one
/// algebra entry - or `affine` / `log`, the behaviour the variant has.
enum KindItem {
    Rule { op: Op, other: KindRef, result: KindRef },
    Affine(syn::Ident),
    Log(syn::Ident),
}

/// A parsed label enum: its variants, which of them is the plain label, and
/// every algebra declaration written on them. Shared by the `Dimension` and `UcumKind` derives.
struct KindModel<'a> {
    variants: Vec<&'a syn::Ident>,
    zero: usize,
    /// `(owner variant, op, other, result)` from `#[kind(op(..) = ..)]`.
    rules: Vec<(usize, Op, KindRef, KindRef)>,
    /// The variant marked `#[kind(affine)]`: a point.
    affine: Option<usize>,
    /// The variant marked `#[kind(log)]`: a logarithm.
    log: Option<usize>,
    /// Per-variant `#[ucum(prefix = ..)]`: `None` where none is declared or
    /// it is declared `none`.
    prefixes: Vec<Option<String>>,
}

impl KindModel<'_> {
    /// Resolve a `KindRef` to a variant index.
    fn resolve(&self, r: &KindRef) -> syn::Result<usize> {
        match r {
            KindRef::Invalid => Ok(NO_KIND),
            KindRef::Named(id) => self.variants.iter().position(|v| *v == id).ok_or_else(|| {
                syn::Error::new_spanned(id, format!("`{id}` is not a variant of this enum"))
            }),
        }
    }
}

/// Parse a label enum: the `#[default]` plain label and the `#[kind(..)]`
/// attributes on its variants.
///
/// Every variant must be a unit variant. Nothing depends on a variant's
/// name: the plain label is the one marked `#[default]`, and every other
/// variant is a label with the defaults - declare its sums with
/// `#[kind(add(..) = ..)]` / `sub(..)`. One variant may be marked
/// `#[kind(affine)]` (points) and one `#[kind(log)]` (levels and gains).
fn parse_kind(input: &DeriveInput) -> syn::Result<KindModel<'_>> {
    let name = &input.ident;
    let variants = match &input.data {
        Data::Enum(e) => &e.variants,
        _ => {
            return Err(syn::Error::new_spanned(
                name,
                "a label is an enum of unit variants",
            ));
        }
    };

    let mut idents: Vec<&syn::Ident> = Vec::new();
    let mut rules: Vec<(usize, Op, KindRef, KindRef)> = Vec::new();
    let mut prefixes: Vec<Option<String>> = Vec::new();
    let mut zero: Option<usize> = None;
    let mut affine: Option<usize> = None;
    let mut log: Option<usize> = None;

    for (i, variant) in variants.iter().enumerate() {
        if !matches!(variant.fields, Fields::Unit) {
            return Err(syn::Error::new_spanned(
                variant,
                format!(
                    "a label's variants are unit variants, but `{}` has fields; a label is a \
                     plain tag that must also satisfy `#[derive(Copy, Clone, PartialEq, Eq, \
                     ConstParamTy)]`",
                    variant.ident
                ),
            ));
        }
        idents.push(&variant.ident);
        prefixes.push(ucum_prefix_override(variant)?.flatten());
        if variant.attrs.iter().any(|a| a.path().is_ident("default")) && zero.replace(i).is_some() {
            return Err(syn::Error::new_spanned(
                variant,
                "more than one variant marked `#[default]`",
            ));
        }

        for attr in &variant.attrs {
            if !attr.path().is_ident("kind") {
                continue;
            }
            for item in parse_kind_items(attr)? {
                match item {
                    KindItem::Rule { op, other, result } => rules.push((i, op, other, result)),
                    KindItem::Affine(id) | KindItem::Log(id) => {
                        let slot = if id == "affine" { &mut affine } else { &mut log };
                        if slot.replace(i).is_some() {
                            return Err(syn::Error::new_spanned(
                                &id,
                                format!("only one variant can be `#[kind({id})]`"),
                            ));
                        }
                        if affine == Some(i) && log == Some(i) {
                            return Err(syn::Error::new_spanned(
                                &id,
                                "a variant is a point or a logarithm, not both",
                            ));
                        }
                    }
                }
            }
        }
    }

    let zero = zero.ok_or_else(|| {
        syn::Error::new_spanned(
            name,
            "mark the plain label - the label of a plain number - with `#[default]`",
        )
    })?;
    if affine == Some(zero) || log == Some(zero) {
        return Err(syn::Error::new_spanned(
            idents[zero],
            "the `#[default]` label is a plain number's, an amount: it cannot be \
             `#[kind(affine)]` or `#[kind(log)]`",
        ));
    }
    // A point's and a logarithm's sums are fixed by the dimension: a rule on
    // one, or naming one, would never be read.
    let special = |i: usize| affine == Some(i) || log == Some(i);
    let names_special = |r: &KindRef| match r {
        KindRef::Named(id) => idents.iter().position(|v| *v == id).is_some_and(special),
        KindRef::Invalid => false,
    };
    for (owner, _, other, result) in &rules {
        if special(*owner) || names_special(other) || names_special(result) {
            return Err(syn::Error::new_spanned(
                idents[*owner],
                "a point's and a logarithm's sums are fixed (a point less a point is an \
                 amount, an amount moves a point; logarithms add their exponents): no \
                 `add(..)` / `sub(..)` rule may be written on or name an `affine` or `log` \
                 variant",
            ));
        }
    }

    Ok(KindModel {
        variants: idents,
        zero,
        rules,
        affine,
        log,
        prefixes,
    })
}

/// A variant's `#[ucum(prefix = "rad")]` / `#[ucum(prefix = none)]`.
///
/// `Ok(None)` means nothing was declared (no prefix);
/// `Ok(Some(None))` means `none` was declared explicitly.
fn ucum_prefix_override(variant: &syn::Variant) -> syn::Result<Option<Option<String>>> {
    let mut found = None;
    for attr in &variant.attrs {
        if !attr.path().is_ident("ucum") {
            continue;
        }
        let value = attr.parse_args_with(|input: ParseStream<'_>| {
            let key: syn::Ident = input.parse()?;
            if key != "prefix" {
                return Err(syn::Error::new_spanned(
                    &key,
                    format!("unknown `#[ucum(..)]` argument `{key}`; expected `prefix`"),
                ));
            }
            input.parse::<Token![=]>()?;
            if input.peek(syn::LitStr) {
                Ok(Some(input.parse::<syn::LitStr>()?.value()))
            } else {
                let id: syn::Ident = input.parse()?;
                if id == "none" {
                    Ok(None)
                } else {
                    Err(syn::Error::new_spanned(
                        &id,
                        "expected a string literal or `none`",
                    ))
                }
            }
        })?;
        if found.replace(value).is_some() {
            return Err(syn::Error::new_spanned(
                attr,
                "duplicate `#[ucum(prefix = ..)]` on this variant",
            ));
        }
    }
    Ok(found)
}

/// Parse one `#[kind(..)]` attribute's comma-separated items.
fn parse_kind_items(attr: &syn::Attribute) -> syn::Result<Vec<KindItem>> {
    attr.parse_args_with(|input: ParseStream<'_>| {
        let mut items = Vec::new();
        while !input.is_empty() {
            let id: syn::Ident = input.parse()?;
            if input.peek(syn::token::Paren) {
                let op = Op::parse(&id).ok_or_else(|| {
                    syn::Error::new_spanned(
                        &id,
                        format!(
                            "`{id}` is not a rule: a label declares its sums, `add(..)` and \
                             `sub(..)` (products and quotients are fixed - a label survives \
                             only beside a plain number)"
                        ),
                    )
                })?;
                let content;
                syn::parenthesized!(content in input);
                let other: KindRef = content.parse()?;
                if !content.is_empty() {
                    return Err(content.error("expected exactly one variant name"));
                }
                input.parse::<Token![=]>()?;
                let result: KindRef = input.parse()?;
                items.push(KindItem::Rule { op, other, result });
            } else if id == "affine" {
                items.push(KindItem::Affine(id));
            } else if id == "log" {
                items.push(KindItem::Log(id));
            } else {
                return Err(syn::Error::new_spanned(
                    &id,
                    format!(
                        "`{id}` is not a rule: a label declares its sums, `add(Other) = Result` \
                         and `sub(Other) = Result`, or the behaviour it has, `affine` (a point) \
                         or `log` (a level or a gain)"
                    ),
                ));
            }
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
        }
        Ok(items)
    })
}

// Generates, for an enum of labels: `Dimension` (dimension one: the plain
// label), `Label` (which variant is a point and which a logarithm), the four
// const std operators (total `*` / `/`, `+` / `-` with `Output =
// Option<Self>`), `PartialRoot`, `PartialPowi` and `Display`.
//
// The tables are *data*, not code: start from four universal defaults, then
// the variants' own `#[kind(op(..) = ..)]` declarations, and treat every pair
// left over as `None`. Nothing about a particular variant name is baked into
// the emission.
pub(crate) fn expand_label(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let __paths = Paths::parse(input)?;
    let yadc = &__paths.yadc;
    let name = &input.ident;
    let model = parse_kind(input)?;
    let n = model.variants.len();
    let zero = model.zero;

    // One table per op: (lhs, rhs) -> result, `NO_KIND` for "rejected". A
    // missing entry is rejected too.
    let mut tables: [std::collections::HashMap<(usize, usize), usize>; 4] = Default::default();
    let set = |tables: &mut [std::collections::HashMap<(usize, usize), usize>; 4],
               op: Op,
               a: usize,
               b: usize,
               r: usize| {
        tables[op.index()].insert((a, b), r);
    };

    // Kinds are labels. `*` and `/` are total and fixed: a label survives
    // only beside the plain kind (x * plain, plain * x, x / plain); every
    // other product or quotient is the plain kind. `+` / `-` default to "a
    // label with itself"; a declaration adds or (`= invalid`) revokes `add` /
    // `sub` entries.
    for x in 0..n {
        for y in 0..n {
            let mul = if x == zero {
                y
            } else if y == zero {
                x
            } else {
                zero
            };
            let div = if y == zero { x } else { zero };
            set(&mut tables, Op::Mul, x, y, mul);
            set(&mut tables, Op::Div, x, y, div);
        }
        set(&mut tables, Op::Add, x, x, x);
        set(&mut tables, Op::Sub, x, x, x);
    }

    // The user's own declarations win over the defaults.
    for (owner, op, other, result) in &model.rules {
        let a = model.resolve(other)?;
        if a == NO_KIND {
            return Err(syn::Error::new_spanned(
                name,
                "`invalid` can only be a rule's result (`add(X) = invalid`), not its operand",
            ));
        }
        let r = model.resolve(result)?;
        if matches!(op, Op::Add) && r != NO_KIND && r != *owner {
            return Err(syn::Error::new_spanned(
                model.variants[*owner],
                "a sum keeps its left operand's label - `absolute + step` is absolute - so an \
                 `add(..)` rule's result must be the variant it is written on, or `invalid`",
            ));
        }
        set(&mut tables, *op, *owner, a, r);
    }

    let variants = &model.variants;
    let v_zero = variants[zero];
    let is = |v: Option<usize>| match v {
        Some(i) => {
            let v = variants[i];
            quote! { ::core::matches!(self, #name::#v) }
        }
        None => quote! { false },
    };
    let (is_affine, is_log) = (is(model.affine), is(model.log));

    // Render a partial table (`+`, `-`) as a match returning `Option<Self>`;
    // every rejected pair falls to the `_ => None` arm, omitted when the arms
    // are exhaustive.
    let arms_for = |op: Op| -> TokenStream2 {
        let table = &tables[op.index()];
        let mut arms = Vec::new();
        for a in 0..n {
            for b in 0..n {
                let Some(&r) = table.get(&(a, b)) else {
                    continue;
                };
                if r == NO_KIND {
                    continue;
                }
                let (va, vb, vr) = (variants[a], variants[b], variants[r]);
                arms.push(quote! {
                    (#name::#va, #name::#vb) => ::core::option::Option::Some(#name::#vr),
                });
            }
        }
        let fallback = if arms.len() == n * n {
            quote! {}
        } else {
            quote! { _ => ::core::option::Option::None, }
        };
        quote! { match (self, rhs) { #(#arms)* #fallback } }
    };
    // Render a total table (`*`, `/`: every pair has an entry, by
    // construction above) as a match returning `Self`.
    let total_arms_for = |op: Op| -> TokenStream2 {
        let table = &tables[op.index()];
        let mut arms = Vec::new();
        for a in 0..n {
            for b in 0..n {
                let r = table[&(a, b)];
                let (va, vb, vr) = (variants[a], variants[b], variants[r]);
                arms.push(quote! { (#name::#va, #name::#vb) => #name::#vr, });
            }
        }
        quote! { match (self, rhs) { #(#arms)* } }
    };
    let mul = total_arms_for(Op::Mul);
    let div = total_arms_for(Op::Div);
    let add = arms_for(Op::Add);
    let sub = arms_for(Op::Sub);

    let opt = quote! { ::core::option::Option<Self> };
    // `Display`: the variant's name.
    let display = if crate::util::dimension_impls(&input.attrs)?
        .is_none_or(|l| l.iter().any(|x| x == "Display"))
    {
        let arms = variants
            .iter()
            .map(|v| quote! { #name::#v => ::core::stringify!(#v), });
        quote! {
            impl ::core::fmt::Display for #name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.write_str(match self { #(#arms)* })
                }
            }
        }
    } else {
        quote! {}
    };
    let powi = crate::util::partial_powi_impl(yadc, name);
    Ok(quote! {
        #display
        #powi
        // A label is a dimension with no exponents: its dimension one is the
        // label of a plain number, and its algebra is the four std operators -
        // `*` and `/` total, `+` and `-` `None` where there is no result.
        const impl #yadc::Dimension for #name {
            const DIMENSIONLESS: Self = #name::#v_zero;
            fn plain(self) -> Self { #name::#v_zero }
        }
        const impl #yadc::Label for #name {
            fn is_affine(self) -> bool { #is_affine }
            fn is_log(self) -> bool { #is_log }
        }
        const impl ::core::ops::Mul for #name {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self { #mul }
        }
        const impl ::core::ops::Div for #name {
            type Output = Self;
            fn div(self, rhs: Self) -> Self { #div }
        }
        const impl ::core::ops::Add for #name {
            type Output = #opt;
            fn add(self, rhs: Self) -> #opt { #add }
        }
        const impl ::core::ops::Sub for #name {
            type Output = #opt;
            fn sub(self, rhs: Self) -> #opt { #sub }
        }
        // The n-th root of a label is the label whose n-th power it is (none
        // squares to a label, so past n = 1 only the plain label has roots): a
        // search over the variants, each power `v^n` built from `v` with `*` /
        // `/`.
        // A label's first root is itself; any other root exists only of the
        // plain label (a label's powers are all plain), and is plain.
        const impl #yadc::PartialRoot for #name {
            fn partial_root(self, n: i8) -> #opt {
                if n == 1 {
                    ::core::option::Option::Some(self)
                } else if n != 0 && self == #name::#v_zero {
                    ::core::option::Option::Some(self)
                } else {
                    ::core::option::Option::None
                }
            }
        }
    })
}

// Generates `impl UcumKind` (variant `name` + the declared `#[ucum(prefix)]`
// <-> UCUM prefix round-trip) for a label enum. Separate from `expand_label` so the mapping
// is only emitted - and serde only required - when `#[derive(UcumKind)]` is used.
pub(crate) fn expand_ucum_kind(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let __paths = Paths::parse(input)?;
    let yadc = &__paths.yadc;
    let name = &input.ident;
    let model = parse_kind(input)?;
    let variant_idents = &model.variants;

    // The prefix of each variant: `#[ucum(prefix = "..")]` when declared,
    // none otherwise (`#[ucum(prefix = none)]` says so explicitly). Both
    // halves of the mapping are emitted from this one list, so serialize and
    // deserialize cannot drift.
    let mut prefix_fwd = Vec::new();
    let mut prefix_rev = Vec::new();
    let mut seen: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for (i, v) in model.variants.iter().enumerate() {
        let prefix = model.prefixes[i].clone();
        let Some(prefix) = prefix else { continue };
        if let Some(other) = seen.insert(prefix.clone(), i) {
            let other = model.variants[other];
            return Err(syn::Error::new_spanned(
                v,
                format!("UCUM prefix `{prefix}` is already used by `{other}`"),
            ));
        }
        prefix_fwd.push(quote! { #name::#v => ::core::option::Option::Some(#prefix), });
        prefix_rev.push(quote! { #prefix => ::core::option::Option::Some(#name::#v), });
    }

    // Omit the catch-all when every variant already has a prefix arm.
    let prefix_fwd_rest = if prefix_fwd.len() == model.variants.len() {
        quote! {}
    } else {
        quote! { _ => ::core::option::Option::None, }
    };

    Ok(quote! {
        impl #yadc::ucum::UcumKind for #name {
            fn name(self) -> &'static str {
                match self {
                    #( #name::#variant_idents => ::core::stringify!(#variant_idents), )*
                }
            }

            fn from_name(name: &str) -> ::core::option::Option<Self> {
                match name {
                    #( ::core::stringify!(#variant_idents) => ::core::option::Option::Some(#name::#variant_idents), )*
                    _ => ::core::option::Option::None,
                }
            }

            fn ucum_prefix(self) -> ::core::option::Option<&'static str> {
                match self {
                    #(#prefix_fwd)*
                    #prefix_fwd_rest
                }
            }

            fn from_ucum_prefix(prefix: &str) -> ::core::option::Option<Self> {
                match prefix {
                    #(#prefix_rev)*
                    _ => ::core::option::Option::None,
                }
            }
        }
    })
}
