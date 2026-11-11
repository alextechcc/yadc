//! The derives behind `dynamickit`: [`Dynamic`](macro@Dynamic) for the dynamic
//! type, [`Visit`](macro@Visit), [`Variants`](macro@Variants),
//! [`Members`](macro@Members) and [`Via`](macro@Via) for closed sets.
//! `Visit` and `Variants` write the dynamickit trait they are named after; the
//! others write std impls only. See the `dynamickit` crate for the traits and
//! the examples.
//!
//! Every derive takes `#[dynamickit(crate = "path")]` to name the `dynamickit` crate
//! (default `::dynamickit`), and passes the type's generic parameters and where
//! clause through to every impl. Each derive's attributes are namespaced by
//! it: `#[dynamic(..)]`, `#[members(..)]`, `#[via(..)]`.

mod dynamic;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::ext::IdentExt;
use syn::{
    Data, DeriveInput, Fields, GenericParam, Ident, Path, Token, Type, parse::ParseStream,
    parse_macro_input, punctuated::Punctuated,
};

/// What a field of a variant is marked as, by `#[members(..)]` or `#[via(..)]`.
#[derive(Default, Clone, Copy)]
struct Marks {
    /// `#[members(member)]`: the variant's member.
    member: bool,
    /// `#[members(default)]`: filled with `Default` when the variant is built
    /// from its member.
    default: bool,
    /// `#[via(keep)]`: kept from the left operand when a result is the same
    /// variant.
    keep: bool,
}

struct Variant {
    ident: Ident,
    named: bool,
    fields: Vec<(Option<Ident>, Type)>,
    marks: Vec<Marks>,
}

fn field_marks(f: &syn::Field) -> syn::Result<Marks> {
    let mut m = Marks::default();
    for a in &f.attrs {
        if a.path().is_ident("members") {
            a.parse_nested_meta(|meta| {
                if meta.path.is_ident("member") {
                    m.member = true;
                    Ok(())
                } else if meta.path.is_ident("default") {
                    m.default = true;
                    Ok(())
                } else {
                    Err(meta.error(
                        "on a field, `#[members(..)]` takes `member` (the variant's member) or \
                         `default` (filled with `Default` when the variant is built from its \
                         member)",
                    ))
                }
            })?;
        } else if a.path().is_ident("via") {
            a.parse_nested_meta(|meta| {
                if meta.path.is_ident("keep") {
                    m.keep = true;
                    Ok(())
                } else {
                    Err(meta.error(
                        "on a field, `#[via(..)]` takes `keep`: kept from the left operand when \
                         a result is the same variant",
                    ))
                }
            })?;
        }
    }
    if m.member && (m.default || m.keep) {
        return Err(syn::Error::new_spanned(
            f,
            "the member is what the operation computes: it is not `default` or `keep`",
        ));
    }
    Ok(m)
}

fn variants(input: &DeriveInput, derive: &str) -> syn::Result<Vec<Variant>> {
    let Data::Enum(e) = &input.data else {
        return Err(syn::Error::new_spanned(
            &input.ident,
            format!("`{derive}` is for enums"),
        ));
    };
    if e.variants.is_empty() {
        return Err(syn::Error::new_spanned(
            &input.ident,
            format!("`{derive}` needs at least one variant"),
        ));
    }
    e.variants
        .iter()
        .map(|v| {
            Ok(Variant {
                ident: v.ident.clone(),
                named: matches!(v.fields, Fields::Named(_)),
                fields: v
                    .fields
                    .iter()
                    .map(|f| (f.ident.clone(), f.ty.clone()))
                    .collect(),
                marks: v
                    .fields
                    .iter()
                    .map(field_marks)
                    .collect::<syn::Result<_>>()?,
            })
        })
        .collect()
}

/// `#[dynamickit(crate = path)]`, default `::dynamickit`.
pub(crate) fn krate(input: &DeriveInput) -> syn::Result<Path> {
    for a in &input.attrs {
        if a.path().is_ident("dynamickit") {
            let mut found = None;
            a.parse_nested_meta(|m| {
                if m.path.is_ident("crate") {
                    found = Some(m.value()?.parse::<syn::LitStr>()?.parse::<Path>()?);
                    Ok(())
                } else {
                    Err(m.error("expected `crate = \"path\"`"))
                }
            })?;
            if let Some(p) = found {
                return Ok(p);
            }
        }
    }
    Ok(syn::parse_quote!(::dynamickit))
}

impl Variant {
    /// `Name::V(p0, p1)` / `Name::V { a: p0, b: p1 }` / `Name::V`, binding `prefixN`.
    fn pattern(&self, name: &Ident, prefix: &str) -> TokenStream2 {
        let binds: Vec<TokenStream2> = (0..self.fields.len())
            .map(|i| {
                let b = format_ident!("{prefix}{i}");
                quote! { #b }
            })
            .collect();
        self.build(name, &binds)
    }
    /// Construct this variant from one expression per field.
    fn build(&self, name: &Ident, exprs: &[TokenStream2]) -> TokenStream2 {
        let v = &self.ident;
        if self.fields.is_empty() {
            quote! { #name::#v }
        } else if self.named {
            let names = self.fields.iter().map(|(n, _)| n.as_ref().unwrap());
            quote! { #name::#v { #(#names: #exprs),* } }
        } else {
            quote! { #name::#v(#(#exprs),*) }
        }
    }
    /// The type a visitor sees: `()`, the single field's type, or a tuple -
    /// of references, with `lt`.
    fn contents_ty(&self, lt: Option<&TokenStream2>) -> TokenStream2 {
        let tys: Vec<TokenStream2> = self
            .fields
            .iter()
            .map(|(_, t)| match lt {
                Some(lt) => quote! { &#lt #t },
                None => quote! { #t },
            })
            .collect();
        match tys.len() {
            0 => quote! { () },
            1 => tys[0].clone(),
            _ => quote! { (#(#tys),*) },
        }
    }
    /// Construct this variant from its contents (`()`, the single field, or a
    /// tuple of the fields).
    fn build_from_contents(&self, name: &Ident, c: TokenStream2) -> TokenStream2 {
        match self.fields.len() {
            0 => {
                let b = self.build(name, &[]);
                quote! {{ let () = #c; #b }}
            }
            1 => self.build(name, &[c]),
            n => {
                let binds: Vec<TokenStream2> = (0..n)
                    .map(|i| {
                        let b = format_ident!("__c{i}");
                        quote! { #b }
                    })
                    .collect();
                let b = self.build(name, &binds);
                quote! {{ let (#(#binds),*) = #c; #b }}
            }
        }
    }
    fn contents_expr(&self, prefix: &str) -> TokenStream2 {
        let binds: Vec<Ident> = (0..self.fields.len())
            .map(|i| format_ident!("{prefix}{i}"))
            .collect();
        match binds.len() {
            0 => quote! { () },
            1 => {
                let b = &binds[0];
                quote! { #b }
            }
            _ => quote! { (#(#binds),*) },
        }
    }
    /// The member: the only field, or the one marked `#[members(member)]`.
    fn member(&self, derive: &str) -> syn::Result<usize> {
        let marked: Vec<usize> = (0..self.fields.len())
            .filter(|i| self.marks[*i].member)
            .collect();
        match (self.fields.len(), marked.as_slice()) {
            (0, _) => Err(syn::Error::new_spanned(
                &self.ident,
                format!("`{derive}` needs a field in every variant: its member"),
            )),
            (1, _) => Ok(0),
            (_, [m]) => Ok(*m),
            (_, []) => Err(syn::Error::new_spanned(
                &self.ident,
                format!(
                    "`{derive}`: this variant has several fields - mark its member \
                     `#[members(member)]`"
                ),
            )),
            _ => Err(syn::Error::new_spanned(
                &self.ident,
                "at most one `#[members(member)]` field per variant",
            )),
        }
    }
    /// Whether the variant can be built from its member alone: every other
    /// field is `#[members(default)]`.
    fn builds_from_member(&self, m: usize) -> bool {
        (0..self.fields.len()).all(|i| i == m || self.marks[i].default)
    }
}

/// The enum's own generics, split for an impl that adds parameters of its own.
struct Generics {
    ty_args: TokenStream2,
    preds: Vec<TokenStream2>,
}
fn generics(input: &DeriveInput) -> Generics {
    let g = &input.generics;
    let (_, ty_args, _) = g.split_for_impl();
    Generics {
        ty_args: quote! { #ty_args },
        preds: g
            .where_clause
            .iter()
            .flat_map(|w| w.predicates.iter())
            .map(|p| quote! { #p })
            .collect(),
    }
}
/// `impl<'a, extra lifetimes, T, extra..>` - lifetimes first, as Rust
/// requires.
fn impl_head(
    extra_lts: &[TokenStream2],
    extra: &[TokenStream2],
    input: &DeriveInput,
) -> TokenStream2 {
    let lts: Vec<_> = input
        .generics
        .params
        .iter()
        .filter(|p| matches!(p, GenericParam::Lifetime(_)))
        .map(|p| quote! { #p })
        .collect();
    let rest: Vec<_> = input
        .generics
        .params
        .iter()
        .filter(|p| !matches!(p, GenericParam::Lifetime(_)))
        .map(|p| quote! { #p })
        .collect();
    quote! { impl<#(#lts,)* #(#extra_lts,)* #(#rest,)* #(#extra),*> }
}

// ===== Visit =================================================================

/// Case analysis on an enum: `dynamickit::Visit<X>` for every `X` that is a
/// `Visitor` of each variant's contents (`()`, the single field, or a tuple of
/// the fields), all with the same output - and for `&Enum`, handing
/// references (`&T`, or a tuple of references).
///
/// ```ignore
/// #[derive(Visit)]
/// enum Shape { Circle(Circle), Segment(Line, u32), Dot }
/// shape.visit_with(Area)    // `Area: Visitor<Circle> + Visitor<(Line, u32)> + Visitor<()>`
/// (&shape).visit_with(Name) // `Name: Visitor<&Circle> + Visitor<(&Line, &u32)> + Visitor<()>`
/// ```
///
/// By hand, a `match` with one arm per variant:
///
/// ```ignore
/// impl<X> Visit<X> for Shape where X: Visitor<Circle> + Visitor<(Line, u32), Output = ..> + .. {
///     type Output = <X as Visitor<Circle>>::Output;
///     fn visit_with(self, x: X) -> Self::Output {
///         match self { Shape::Circle(c) => x.visit(c), Shape::Segment(l, n) => x.visit((l, n)), .. }
///     }
/// }
/// ```
///
/// Ordinary impls, not `const`: several `[const]` bounds of one trait over
/// types with const-expression arguments (`Qty<{ .. }>`) are ambiguous to
/// today's trait solver.
#[proc_macro_derive(Visit, attributes(dynamickit))]
pub fn derive_visit(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_visit(&input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn expand_visit(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let k = krate(input)?;
    let name = &input.ident;
    let vs = variants(input, "Visit")?;
    let g = generics(input);
    let ty_args = &g.ty_args;
    let preds = &g.preds;
    let me = quote! { #name #ty_args };
    let x = quote! { __X };
    let lt = quote! { '__dynamickit };

    let one = |by_ref: bool| {
        let l = if by_ref { Some(&lt) } else { None };
        let c0 = vs[0].contents_ty(l);
        let out = quote! { <#x as #k::Visitor<#c0>>::Output };
        let mut bounds: Vec<TokenStream2> = Vec::new();
        let mut seen: Vec<String> = Vec::new();
        for (i, v) in vs.iter().enumerate() {
            let c = v.contents_ty(l);
            if seen.contains(&c.to_string()) {
                continue;
            }
            seen.push(c.to_string());
            bounds.push(if i == 0 {
                quote! { #k::Visitor<#c> }
            } else {
                quote! { #k::Visitor<#c, Output = #out> }
            });
        }
        let arms = vs.iter().map(|v| {
            let pat = v.pattern(name, "__f");
            let e = v.contents_expr("__f");
            let c = v.contents_ty(l);
            quote! { #pat => <#x as #k::Visitor<#c>>::visit(x, #e), }
        });
        let (lts, target) = if by_ref {
            (vec![lt.clone()], quote! { &#lt #me })
        } else {
            (vec![], me.clone())
        };
        let head = impl_head(&lts, std::slice::from_ref(&x), input);
        // An ordinary impl: several `[const]` bounds of one trait over types
        // with const-expression arguments (`Qty<{ .. }>`) are ambiguous to
        // today's trait solver; plain ones are not.
        quote! {
            #head #k::Visit<#x> for #target
            where
                #(#preds,)*
                #x: #(#bounds)+*
            {
                type Output = #out;
                #[allow(unused_variables)]
                fn visit_with(self, x: #x) -> Self::Output {
                    match self { #(#arms)* }
                }
            }
        }
    };
    let by_value = one(false);
    let by_ref = one(true);
    Ok(quote! { #by_value #by_ref })
}

// ===== Members ===============================================================

/// Conversions between an enum and its variants' *members*: the only field
/// of a variant, or the one marked `#[members(member)]`.
///
/// ```ignore
/// #[derive(Members)]
/// #[members(convert(AnyBag))]
/// enum Cart {
///     Apples(Bag<Apple>),
///     Oranges(#[members(member)] Bag<Orange>, #[members(default)] Note),
/// }
/// ```
///
/// Writes std impls only, per variant: `TryFrom<Enum> for M` (`Error =
/// Enum`: the enum back on a mismatch), and `From<M> for Enum` when the
/// variant's other fields are all `#[members(default)]` (filled with
/// `Default`; a variant with an unmarked other field has no `From<M>`). Each
/// member type may appear once. `#[members(convert(X, ..))]` chains them
/// through another type: `From<Enum> for X` (`X: From<M>` each) and
/// `TryFrom<X> for Enum` (each member's `TryFrom<X, Error = X>` in turn) -
/// which builds every variant from its member, so it needs every `From<M>`.
///
/// By hand, per variant:
///
/// ```ignore
/// impl From<Bag<Apple>> for Cart { fn from(b: Bag<Apple>) -> Self { Cart::Apples(b) } }
/// impl TryFrom<Cart> for Bag<Apple> {
///     type Error = Cart;
///     fn try_from(c: Cart) -> Result<Self, Cart> {
///         match c { Cart::Apples(b) => Ok(b), c => Err(c) }
///     }
/// }
/// ```
///
/// All `const`.
#[proc_macro_derive(Members, attributes(dynamickit, members))]
pub fn derive_members(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_members(&input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// One entry of `#[members(..)]` on the enum: `convert(X, ..)`.
struct MembersItem(Vec<Type>);
impl syn::parse::Parse for MembersItem {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let key: Ident = input.parse()?;
        if key != "convert" {
            return Err(syn::Error::new_spanned(
                key,
                "on the enum, `#[members(..)]` takes `convert(X, ..)`; on a field, `member` or \
                 `default`",
            ));
        }
        let inner;
        syn::parenthesized!(inner in input);
        let list: Punctuated<Type, Token![,]> = Punctuated::parse_terminated(&inner)?;
        Ok(MembersItem(list.into_iter().collect()))
    }
}

fn expand_members(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let name = &input.ident;
    let vs = variants(input, "Members")?;
    let g = generics(input);
    let head = impl_head(&[], &[], input);
    let ty_args = &g.ty_args;
    let preds = &g.preds;
    let me = quote! { #name #ty_args };
    let mut out = TokenStream2::new();

    let mut converts: Vec<Type> = Vec::new();
    for a in input.attrs.iter().filter(|a| a.path().is_ident("members")) {
        let list: Punctuated<MembersItem, Token![,]> =
            a.parse_args_with(Punctuated::parse_terminated)?;
        for MembersItem(xs) in list {
            converts.extend(xs);
        }
    }

    // Each member type may belong to one variant only, or the conversions
    // would be ambiguous.
    let mut seen: Vec<(String, &Ident)> = Vec::new();
    let mut members = Vec::new();
    for v in &vs {
        let m = v.member("Members")?;
        let key = {
            let t = &v.fields[m].1;
            quote! { #t }.to_string()
        };
        if let Some((_, other)) = seen.iter().find(|(k, _)| *k == key) {
            return Err(syn::Error::new_spanned(
                &v.ident,
                format!(
                    "`{}` and `{other}` have the same member type, so the conversions would be \
                     ambiguous",
                    v.ident
                ),
            ));
        }
        seen.push((key, &v.ident));
        members.push(m);
    }

    for (v, &m) in vs.iter().zip(&members) {
        let mty = &v.fields[m].1;
        // Out: the member, or the enum handed back.
        let pat = v.pattern(name, "__f");
        let fm = format_ident!("__f{m}");
        out.extend(quote! {
            const #head ::core::convert::TryFrom<#me> for #mty
            where #(#preds,)* #me: [const] ::core::marker::Destruct
            {
                type Error = #me;
                #[allow(unused_variables, unreachable_patterns)]
                fn try_from(e: #me) -> ::core::result::Result<Self, #me> {
                    match e {
                        #pat => ::core::result::Result::Ok(#fm),
                        e => ::core::result::Result::Err(e),
                    }
                }
            }
        });
        // In: the member, the other fields from `Default` - only where they
        // say so.
        if v.builds_from_member(m) {
            let exprs: Vec<TokenStream2> = (0..v.fields.len())
                .map(|i| {
                    if i == m {
                        quote! { member }
                    } else {
                        quote! { ::core::default::Default::default() }
                    }
                })
                .collect();
            let build = v.build(name, &exprs);
            let defaults = (0..v.fields.len()).filter(|i| *i != m).map(|i| {
                let t = &v.fields[i].1;
                quote! { #t: [const] ::core::default::Default }
            });
            out.extend(quote! {
                const #head ::core::convert::From<#mty> for #me where #(#preds,)* #(#defaults,)* {
                    fn from(member: #mty) -> Self { #build }
                }
            });
        }
    }

    // `convert(X)`: to and from `X`, through the members.
    for x in converts {
        for (v, &m) in vs.iter().zip(&members) {
            if !v.builds_from_member(m) {
                return Err(syn::Error::new_spanned(
                    &v.ident,
                    "`convert(..)` builds every variant from its member: mark this variant's \
                     other fields `#[members(default)]`",
                ));
            }
        }
        let out_arms = vs.iter().zip(&members).map(|(v, &m)| {
            let pat = v.pattern(name, "__f");
            let fm = format_ident!("__f{m}");
            let mty = &v.fields[m].1;
            quote! { #pat => <#x as ::core::convert::From<#mty>>::from(#fm), }
        });
        let tries = vs.iter().zip(&members).map(|(v, &m)| {
            let mty = &v.fields[m].1;
            quote! {
                let x = match <#mty as ::core::convert::TryFrom<#x>>::try_from(x) {
                    ::core::result::Result::Ok(m) => {
                        return ::core::result::Result::Ok(<#me as ::core::convert::From<#mty>>::from(m));
                    }
                    ::core::result::Result::Err(x) => x,
                };
            }
        });
        let member_tys: Vec<&Type> = vs
            .iter()
            .zip(&members)
            .map(|(v, &m)| &v.fields[m].1)
            .collect();
        // The members' conversions as bounds - needed only when the enum is
        // generic. On a concrete enum they are facts the impl checks anyway,
        // and several `[const]` bounds of one trait over types with
        // const-expression arguments (`Qty<{ .. }>`) are ambiguous to today's
        // trait solver, so they are left out.
        let (from_bounds, try_bounds) = if input.generics.params.is_empty() {
            (quote! {}, quote! {})
        } else {
            (
                quote! { #(#x: [const] ::core::convert::From<#member_tys>,)* },
                quote! {
                    #(#member_tys: [const] ::core::convert::TryFrom<#x, Error = #x>,)*
                    #(#me: [const] ::core::convert::From<#member_tys>,)*
                },
            )
        };
        out.extend(quote! {
            const #head ::core::convert::From<#me> for #x
            where #(#preds,)* #from_bounds
            {
                #[allow(unused_variables)]
                fn from(e: #me) -> Self { match e { #(#out_arms)* } }
            }
            /// Each member is asked in turn; `x` comes back if none takes it.
            const #head ::core::convert::TryFrom<#x> for #me
            where
                #(#preds,)*
                #try_bounds
            {
                type Error = #x;
                fn try_from(x: #x) -> ::core::result::Result<Self, #x> {
                    #(#tries)*
                    ::core::result::Result::Err(x)
                }
            }
        });
    }

    Ok(out)
}

// ===== Variants ==============================================================

/// Every variant of an enum, without a value: `dynamickit::Variants<X>` hands
/// each variant's name and constructor, in order, to any `X` that is a
/// `VariantVisitor` of each variant's contents (`()`, the single field, or a
/// tuple of the fields).
///
/// ```ignore
/// #[derive(Variants)]
/// enum Theme { Light, Dark }
/// #[derive(Variants)]
/// enum Setting { Theme(Theme), FontSize(FontSize) }
///
/// Theme::each_variant(&mut names)   // names.visit("Light", |()| Theme::Light), ..
/// ```
///
/// By hand, one call per variant:
///
/// ```ignore
/// impl<X> Variants<X> for Setting
/// where X: VariantVisitor<Setting, Theme> + VariantVisitor<Setting, FontSize>
/// {
///     fn each_variant(x: &mut X) {
///         x.visit("Theme", Setting::Theme);
///         x.visit("FontSize", Setting::FontSize);
///     }
/// }
/// ```
#[proc_macro_derive(Variants, attributes(dynamickit))]
pub fn derive_variants(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_variants(&input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn expand_variants(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let k = krate(input)?;
    let name = &input.ident;
    let vs = variants(input, "Variants")?;
    let g = generics(input);
    let ty_args = &g.ty_args;
    let preds = &g.preds;
    let me = quote! { #name #ty_args };
    let x = quote! { __X };
    let mut bounds: Vec<TokenStream2> = Vec::new();
    let mut seen: Vec<String> = Vec::new();
    for v in &vs {
        let c = v.contents_ty(None);
        if !seen.contains(&c.to_string()) {
            seen.push(c.to_string());
            bounds.push(quote! { #k::VariantVisitor<#me, #c> });
        }
    }
    let calls = vs.iter().map(|v| {
        let key = v.ident.unraw().to_string();
        let c = v.contents_ty(None);
        let make = v.build_from_contents(name, quote! { __c });
        quote! {
            <#x as #k::VariantVisitor<#me, #c>>::visit(x, #key, |__c: #c| -> #me { #make });
        }
    });
    let head = impl_head(&[], std::slice::from_ref(&x), input);
    Ok(quote! {
        #head #k::Variants<#x> for #me
        where
            #(#preds,)*
            #x: #(#bounds)+*
        {
            fn each_variant(x: &mut #x) {
                #(#calls)*
            }
        }
    })
}

// ===== Via ===================================================================

/// Borrow another type's arithmetic: each listed operator on the enum
/// converts both operands into `X`, uses `X`'s operator, and converts back;
/// each listed scaling acts on the member itself.
///
/// ```ignore
/// #[derive(Members, Via)]
/// #[members(convert(AnyBag))]
/// #[via(AnyBag, Add, Sub, Mul<u32>)]
/// enum Cart { Apples(Bag<Apple>), Oranges(Bag<Orange>) }
/// a + b  // Option<Cart>: `AnyBag`'s `+`, back if the result is a member
/// a * 3  // Cart: the same variant, its member times 3
/// ```
///
/// Writes std operators only, exactly those listed after `X`:
///
/// - `Add`, `Sub`, `Mul`, `Div`, `Rem`: `Self ∘ Self` with `Output =
///   Option<Self>` - `None` when `X`'s operator refuses or the result is not
///   a member. Needs `X: From<Enum>` and `Enum: TryFrom<X>` (`Members` with
///   `convert(X)`); `X`'s operator may be total or partial (`Output:
///   Into<Option<X>>`).
/// - `Mul<K>`, `Div<K>`: scaling with `Output = Self` - it cannot change the
///   variant, so each member `M` scales by its own `M: Mul<K, Output = M>`.
///
/// The member of a variant with several fields is the one marked
/// `#[members(member)]`. A result of `Self ∘ Self` is built by `TryFrom<X>`,
/// so its other fields are whatever that fills in; a field marked
/// `#[via(keep)]` is instead kept from the left operand when the result is
/// the left operand's variant (which needs `Enum: Clone`). Not `const`.
///
/// By hand:
///
/// ```ignore
/// impl Add for Cart {
///     type Output = Option<Cart>;
///     fn add(self, r: Cart) -> Option<Cart> {
///         let x: Option<AnyBag> = (AnyBag::from(self) + AnyBag::from(r)).into();
///         Cart::try_from(x?).ok()
///     }
/// }
/// impl Mul<u32> for Cart {
///     type Output = Cart;
///     fn mul(self, k: u32) -> Cart {
///         match self { Cart::Apples(b) => Cart::Apples(b * k), Cart::Oranges(b) => Cart::Oranges(b * k) }
///     }
/// }
/// ```
#[proc_macro_derive(Via, attributes(dynamickit, members, via))]
pub fn derive_via(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_via(&input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

const VIA_HELP: &str = "`#[via(X, ..)]` lists the operators to borrow from `X` after it: `Add`, \
     `Sub`, `Mul`, `Div`, `Rem` (`Self ∘ Self -> Option<Self>`, through `X`), and scalings \
     `Mul<K>`, `Div<K>` (on the member, `-> Self`)";

/// `X, Op, Op<K>, ..`
struct ViaAttr {
    through: Type,
    /// `(operator, method, scaling type)`.
    ops: Vec<(Ident, Ident, Option<Type>)>,
}
impl syn::parse::Parse for ViaAttr {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let through: Type = input.parse()?;
        let mut ops = Vec::new();
        while input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
            let op: Ident = input.parse()?;
            let name = op.to_string();
            if !["Add", "Sub", "Mul", "Div", "Rem"].contains(&name.as_str()) {
                return Err(syn::Error::new_spanned(op, VIA_HELP));
            }
            let by = if input.peek(Token![<]) {
                if name != "Mul" && name != "Div" {
                    return Err(syn::Error::new_spanned(
                        op,
                        "only `Mul<K>` and `Div<K>` take a right-hand type (scaling)",
                    ));
                }
                input.parse::<Token![<]>()?;
                let t: Type = input.parse()?;
                input.parse::<Token![>]>()?;
                Some(t)
            } else {
                None
            };
            let method = format_ident!("{}", name.to_lowercase());
            ops.push((op, method, by));
        }
        if ops.is_empty() {
            return Err(syn::Error::new(input.span(), VIA_HELP));
        }
        Ok(ViaAttr { through, ops })
    }
}

fn expand_via(input: &DeriveInput) -> syn::Result<TokenStream2> {
    krate(input)?;
    let name = &input.ident;
    let vs = variants(input, "Via")?;
    let attr = input
        .attrs
        .iter()
        .find(|a| a.path().is_ident("via"))
        .ok_or_else(|| syn::Error::new_spanned(name, VIA_HELP))?
        .parse_args::<ViaAttr>()?;
    let x = &attr.through;
    let g = generics(input);
    let head = impl_head(&[], &[], input);
    let ty_args = &g.ty_args;
    let preds = &g.preds;
    let me = quote! { #name #ty_args };
    let members: Vec<usize> = vs
        .iter()
        .map(|v| v.member("Via"))
        .collect::<syn::Result<_>>()?;
    let keeps = vs.iter().any(|v| v.marks.iter().any(|m| m.keep));

    // A result of the left operand's variant keeps the left operand's
    // `#[via(keep)]` fields.
    let back_ok = if keeps {
        let keep_arms = vs
            .iter()
            .filter(|v| v.marks.iter().any(|m| m.keep))
            .map(|v| {
                let l = v.pattern(name, "__l");
                let r = v.pattern(name, "__r");
                let exprs: Vec<TokenStream2> = (0..v.fields.len())
                    .map(|i| {
                        let b = if v.marks[i].keep {
                            format_ident!("__l{i}")
                        } else {
                            format_ident!("__r{i}")
                        };
                        quote! { #b }
                    })
                    .collect();
                let build = v.build(name, &exprs);
                quote! { (#l, #r) => #build, }
            });
        quote! {
            ::core::option::Option::Some(match (left, r) { #(#keep_arms)* (_, r) => r })
        }
    } else {
        quote! { ::core::option::Option::Some(r) }
    };
    let left = if keeps {
        quote! { let left = ::core::clone::Clone::clone(&self); }
    } else {
        quote! {}
    };
    let clone = if keeps {
        quote! { #me: ::core::clone::Clone, }
    } else {
        quote! {}
    };
    let mut out = TokenStream2::new();
    for (tr, m, by) in &attr.ops {
        match by {
            // `Self ∘ Self`, through `X`.
            None => out.extend(quote! {
                #head ::core::ops::#tr for #me
                where
                    #(#preds,)*
                    #clone
                    #me: ::core::convert::TryFrom<#x>,
                    #x: ::core::convert::From<#me>,
                    #x: ::core::ops::#tr<Output: ::core::convert::Into<::core::option::Option<#x>>>,
                {
                    type Output = ::core::option::Option<#me>;
                    #[allow(unused_variables, unreachable_patterns)]
                    fn #m(self, rhs: #me) -> ::core::option::Option<#me> {
                        #left
                        let x: ::core::option::Option<#x> = ::core::convert::Into::into(::core::ops::#tr::#m(
                            <#x as ::core::convert::From<#me>>::from(self),
                            <#x as ::core::convert::From<#me>>::from(rhs),
                        ));
                        match x {
                            ::core::option::Option::Some(x) => match <#me as ::core::convert::TryFrom<#x>>::try_from(x) {
                                ::core::result::Result::Ok(r) => #back_ok,
                                ::core::result::Result::Err(_) => ::core::option::Option::None,
                            },
                            ::core::option::Option::None => ::core::option::Option::None,
                        }
                    }
                }
            }),
            // Scaling: the member by its own operator, the variant kept.
            Some(n) => {
                let arms = vs.iter().zip(&members).map(|(v, &mi)| {
                    let pat = v.pattern(name, "__f");
                    let exprs: Vec<TokenStream2> = (0..v.fields.len())
                        .map(|i| {
                            let b = format_ident!("__f{i}");
                            if i == mi {
                                quote! { ::core::ops::#tr::#m(#b, by) }
                            } else {
                                quote! { #b }
                            }
                        })
                        .collect();
                    let build = v.build(name, &exprs);
                    quote! { #pat => #build, }
                });
                let bounds = if input.generics.params.is_empty() {
                    quote! {}
                } else {
                    let mtys = vs.iter().zip(&members).map(|(v, &mi)| &v.fields[mi].1);
                    quote! { #(#mtys: ::core::ops::#tr<#n, Output = #mtys>,)* }
                };
                out.extend(quote! {
                    #head ::core::ops::#tr<#n> for #me
                    where #(#preds,)* #bounds
                    {
                        type Output = #me;
                        fn #m(self, by: #n) -> #me {
                            match self { #(#arms)* }
                        }
                    }
                });
            }
        }
    }
    Ok(out)
}

// ===== Dynamic ===============================================================

/// One struct of yours holding any of several static types: each *mapping*
/// fills every field from a const parameter or a constant expression (a
/// *fact*) or a field path of the static value (*payload*); the `impl(..)`
/// list lifts operators and methods over the fields, each with its output
/// stated.
///
/// ```ignore
/// #[derive(Copy, Clone, PartialEq, Debug, Dynamic)]
/// #[dynamic(Qty<const D: Dim> => { dim: D, value: .0 })]
/// #[dynamic(shared f64 => { dim: Dim::DIMENSIONLESS, value: self })]
/// #[dynamic(impl(
///     Mul -> Self, Div -> Self, Add -> Option<Self>, Sub -> Option<Self>,
///     Mul<f64> -> Option<Self>, PartialOrd,
///     my::Root::root(self, n: i8) -> Option<Self>,
/// ))]
/// pub struct DynQty { dim: Dim, value: f64 }
///
/// #[derive(Copy, Clone, PartialEq, Debug, Dynamic)]
/// #[dynamic(Bag<F: Fruit> { fruit: PhantomData<F> } => { fruit: F::KIND, count: .count })]
/// #[dynamic(impl(Add -> Option<Self>, Mul<u32> -> Self))]
/// pub struct AnyBag { #[dynamic(same)] fruit: FruitKind, count: u32 }
/// ```
///
/// Writes std impls only. Per mapping: `From<Static>` (moves the payload in,
/// records the facts) and the way back, `TryFrom<Self> for Static` (`Error =
/// Self`: checks every fact, moves the payload back, rebuilding the static
/// value). Generic over the mapping's parameters.
///
/// A mapping is `Static => { field: source, .. }`, `Static` a type with its
/// parameters declared (`Qty<const D: Dim>`, `Bag<F: Fruit>`; other
/// arguments are passed as written) or a plain type (`Instant`, `f64`). A
/// source is a declared const parameter, a field path (`.0`, `.since.0`),
/// `self` (the whole static value), or any constant expression (`TIME`,
/// `F::KIND`). The static value's other fields are declared after its type:
/// the types a deep path passes through (`Instant { since: Time } => { ..,
/// value: .since.0 }`), and fields filled with `Default` (`Bag<F: Fruit>
/// { fruit: PhantomData<F> }`).
///
/// Checked here: every field filled once in every mapping, each field one
/// role (fact or payload) in every mapping, payload paths distinct, every
/// declared parameter in some fact, and the static types told apart by their
/// facts - two mappings whose facts could be equal are refused, naming both;
/// where only values can tell (`TIME` vs `LENGTH`, `F::KIND` vs `MIXED`), an
/// assertion checks at compile time that they differ (for a constant naming
/// a parameter, in that mapping's impls, for each member used). A mapping
/// marked `shared` (`shared f64 => { .. }`) is the same value as another
/// and may hold its facts: the way back gives either.
///
/// `#[dynamic(impl(..))]` lists the lifted items. Each does one thing to
/// every field and builds `Self` from the results; its output is stated and
/// checked against the fields' own impls. A field does one of two things:
///
/// - by default, **its own operator**: `a.f ∘ b.f`, `a.f ∘ k`, `-a.f`,
///   `Trait::method(a.f, ..)` - so its type has that operator (a dimension
///   multiplies, a count adds);
/// - marked `#[dynamic(same)]`, **equality**: two operands must be equal in
///   it (otherwise there is no result, so the output is `Option<Self>`), and
///   an operator with one `Self` operand keeps it - for a field that names
///   what the value is and has no arithmetic (a fruit, a colour space).
///
/// The items:
///
/// - `TryFrom`: the way back (always written; may be listed);
/// - `Mul -> Self`, `Add -> Option<Self>`, ... (`Mul`, `Div`, `Add`, `Sub`,
///   `Rem`): `Self ∘ Self`; with `-> Option<Self>` each field's result goes
///   through `Into<Option<_>>` and any `None` makes the result `None`;
/// - `Mul<K> -> ..`, `Div<K> -> ..`, `Neg -> ..`: one `Self` operand;
/// - `path::Trait::method(self, n: T, ..) -> Self` or `-> Option<Self>`: the
///   trait's method on every field (`&self` / `&mut self` pass `&field` /
///   `&mut field`; the extra arguments are passed to each, so they are
///   `Copy`); associated types as bindings, `Trait<A = X>`. An ordinary impl;
/// - `path::Trait::getter(&self) -> T = self.field`: the method returns the
///   field - a `const impl` (so the trait is a `const trait`);
/// - `PartialOrd`: the payload compared, in field order, when the facts are
///   equal; `None` otherwise. `const` when `Self`'s `PartialEq` is.
///
/// Operators are `const impl`s with `[const]` bounds, so a field type whose
/// operators aren't `const` still works at run time. The payload must be
/// visible here (same module, or `pub` fields).
#[proc_macro_derive(Dynamic, attributes(dynamickit, dynamic))]
pub fn derive_dynamic(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    dynamic::expand(&input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

