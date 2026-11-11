//! `#[derive(Quantity)]`: a static family's dimension, `Quantity`, the
//! cross-type operators and their refusals, and the extras (powers, roots,
//! number casts, ..) that `#[quantity(impl(..))]` picks.

use crate::paths::Paths;
use crate::util::{has_attr, is_phantom, member_of, param_decl};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{Data, DeriveInput, GenericParam, Type};

/// A static family: a struct with a dimension const parameter.
pub(crate) fn expand_quantity(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let __paths = Paths::parse(input)?;
    let yadc = &__paths.yadc;
    let opts = Options::parse(input)?;
    let name = &input.ident;
    let generics = &input.generics;

    let value = value_field(&input.data, name)?;
    let value_ty = &value.ty;
    let (d_ident, d_ty) = dim_const_param(generics, name)?;

    let qa = syn::Ident::new("__QA", Span::call_site());
    let qb = syn::Ident::new("__QB", Span::call_site());

    // --- generics passthrough ------------------------------------------------
    // The struct may carry any generics (a generic number type, lifetimes, other
    // const params); only the `#[dim]` const param is ours. Every emitted impl
    // repeats the rest verbatim and substitutes the dim slot with whatever that
    // impl needs - `__QA` / `__QB` for a two-operand op, a `{ ZERO }` const
    // expression for the dimensionless ones.
    let others: Vec<TokenStream2> = generics
        .params
        .iter()
        .filter(|p| !matches!(p, GenericParam::Const(c) if c.ident == *d_ident))
        .map(param_decl)
        .collect();
    // Lifetimes must be declared before type and const parameters, so an impl's
    // own borrow lifetime is spliced ahead of the struct's parameters.
    // Every impl is a `const impl` - so quantities compose in `const` items
    // (`const LIMIT: Velocity = 50.0 * KILOMETRE / HOUR;`) - except the few
    // whose traits cannot be const (`Display`, `Sum`, `num_traits::Inv`,
    // `Root`), which use
    // `head_rt`. Value-type bounds are `[const]`: const only when used in a
    // const context, so a number type with ordinary impls still works.
    let head_with =
        |konst: bool, extra_lts: &[TokenStream2], extra: &[TokenStream2]| -> TokenStream2 {
            let all: Vec<&TokenStream2> = extra_lts
                .iter()
                .chain(others.iter())
                .chain(extra.iter())
                .collect();
            let c = if konst {
                quote! { const }
            } else {
                quote! {}
            };
            if all.is_empty() {
                quote! { #c impl }
            } else {
                quote! { #c impl<#(#all),*> }
            }
        };
    let head =
        |extra_lts: &[TokenStream2], extra: &[TokenStream2]| head_with(true, extra_lts, extra);
    let head_rt =
        |extra_lts: &[TokenStream2], extra: &[TokenStream2]| head_with(false, extra_lts, extra);
    // Every type the impls name is this family at some dimension (and, for a
    // cast or a primitive's impl, some number type).
    // The number type, when it is one of the family's own parameters
    // (`Qty<const D: Dim, V = f64>`): a cast replaces it.
    let v_ident: Option<&syn::Ident> = generics.params.iter().find_map(|p| match p {
        GenericParam::Type(t)
            if matches!(value_ty, Type::Path(tp)
            if tp.qself.is_none() && tp.path.is_ident(&t.ident)) =>
        {
            Some(&t.ident)
        }
        _ => None,
    });
    let ty_v = |dim: &TokenStream2, val: Option<&TokenStream2>| -> TokenStream2 {
        let args: Vec<TokenStream2> = generics
            .params
            .iter()
            .map(|p| match p {
                GenericParam::Lifetime(l) => {
                    let lt = &l.lifetime;
                    quote! { #lt }
                }
                GenericParam::Type(t) if Some(&t.ident) == v_ident && val.is_some() => {
                    val.cloned().unwrap()
                }
                GenericParam::Type(t) => {
                    let i = &t.ident;
                    quote! { #i }
                }
                GenericParam::Const(c) => {
                    if c.ident == *d_ident {
                        dim.clone()
                    } else {
                        let i = &c.ident;
                        quote! { #i }
                    }
                }
            })
            .collect();
        quote! { #name<#(#args),*> }
    };
    let ty_of = |dim: &TokenStream2| -> TokenStream2 { ty_v(dim, None) };
    let user_preds: Vec<TokenStream2> = generics
        .where_clause
        .iter()
        .flat_map(|w| w.predicates.iter())
        .map(|p| quote! { #p })
        .collect();
    let wher = |extra: &[TokenStream2]| -> TokenStream2 {
        if user_preds.is_empty() && extra.is_empty() {
            quote! {}
        } else {
            quote! { where #(#user_preds,)* #(#extra,)* }
        }
    };

    // --- value field ---------------------------------------------------------
    // Never `.0`: the field is whatever `#[value]` marks, or the struct's sole
    // field, named or positional.
    let member = &value.member;
    let ctor = |expr: TokenStream2| -> TokenStream2 { value.construct(name, expr) };

    // Bounds on the number type, added per impl so a generic `N` works. For a
    // concrete `f64` they are trivially satisfied and cost nothing.
    let v = value_ty;
    // A generic number type may have a destructor; the const impls drop values,
    // so they need it to be `[const] Destruct` (the run-time impls do not).
    let value_is_generic = v_ident.is_some();
    let const_extra: Vec<TokenStream2> = if value_is_generic {
        vec![quote! { #v: [const] ::core::marker::Destruct }]
    } else {
        Vec::new()
    };
    let wher_rt = wher;
    let wher = |extra: &[TokenStream2]| -> TokenStream2 {
        let all: Vec<TokenStream2> = extra.iter().chain(const_extra.iter()).cloned().collect();
        wher_rt(&all)
    };
    let b_mul = quote! { #v: [const] ::core::ops::Mul<Output = #v> };
    let b_div = quote! { #v: [const] ::core::ops::Div<Output = #v> };
    let b_add = quote! { #v: [const] ::core::ops::Add<Output = #v> };
    let b_sub = quote! { #v: [const] ::core::ops::Sub<Output = #v> };
    let b_add_assign = quote! { #v: [const] ::core::ops::AddAssign };
    let b_sub_assign = quote! { #v: [const] ::core::ops::SubAssign };
    let b_mul_assign = quote! { #v: [const] ::core::ops::MulAssign };
    let b_div_assign = quote! { #v: [const] ::core::ops::DivAssign };
    let b_neg = quote! { #v: [const] ::core::ops::Neg<Output = #v> };
    // For the run-time-only impls (`Inv`, `Sum`), which cannot carry `[const]`.
    let b_div_rt = quote! { #v: ::core::ops::Div<Output = #v> };
    let b_add_assign_rt = quote! { #v: ::core::ops::AddAssign };
    let b_copy = quote! { #v: ::core::marker::Copy };
    let b_zero = quote! { #v: #yadc::num_traits::Zero };

    // --- dimension expressions ----------------------------------------------
    // Every result dimension is the dimension's own const operator, read
    // through yadc's total helpers (`product_or_one`, ...: `DIMENSIONLESS`
    // where there is no result, never a panic). Whether the operator exists
    // is decided by the refusal traits of `yadc::gates`, which this derive
    // implements below from the same operators. Nothing here is an inherent
    // method, so hand-written operators on a dimension type drive this derive
    // unchanged.
    let gates = quote! { #yadc::gates };
    let dim_zero = quote! { <#d_ty as #yadc::Dimension>::DIMENSIONLESS };
    let dim_fn = |f: &str, args: TokenStream2| -> TokenStream2 {
        let f = syn::Ident::new(f, Span::call_site());
        quote! { #gates::#f::<#d_ty>(#args) }
    };
    let td = quote! { #d_ident };
    let (ta, tb) = (quote! { #qa }, quote! { #qb });

    // Common impl heads: one dim const, two dim consts, and a borrowed-rhs form.
    let p_d = quote! { const #d_ident: #d_ty };
    let p_ab = [quote! { const #qa: #d_ty }, quote! { const #qb: #d_ty }];
    let p_n = quote! { const __N: i8 };
    let lt_a = [quote! { '__a }];
    let self_d = ty_of(&td);
    let self_a = ty_of(&ta);
    let self_b = ty_of(&tb);
    let zero_ty = ty_of(&quote! { { #dim_zero } });
    let mul_ty = ty_of(&{
        let e = dim_fn("product_or_one", quote! { #qa, #qb });
        quote! { { #e } }
    });
    let div_ty = ty_of(&{
        let e = dim_fn("quotient_or_one", quote! { #qa, #qb });
        quote! { { #e } }
    });
    let sub_ty = ty_of(&{
        let e = dim_fn("difference_or_one", quote! { #qa, #qb });
        quote! { { #e } }
    });

    let h_d = head(&[], ::core::slice::from_ref(&p_d));
    let h_ab = head(&[], &p_ab);
    let h_ab_ref = head(&lt_a, &p_ab);

    // --- refusals -------------------------------------------------------------
    // One impl per refusal trait of `yadc::gates`, for every operand pair
    // (or operand) where that refusal does not apply. The check is a const
    // `bool` read through an associated type (`Bool<{ .. }>: Value<Is =
    // True>`), which the compiler evaluates instead of unifying it with
    // `true`; `do_not_recommend` makes a failing check report the refusal
    // trait - and its message - at the operator.
    let refusals = {
        let mut out = TokenStream2::new();
        for (tr, f, shape) in REFUSALS {
            let tr = syn::Ident::new(tr, Span::call_site());
            let (h, target, args, of) = match shape {
                Shape::Pair => (
                    head_rt(&[], &p_ab),
                    quote! { #gates::#tr<#self_b> },
                    quote! { #qa, #qb },
                    &self_a,
                ),
                Shape::One => (
                    head_rt(&[], ::core::slice::from_ref(&p_d)),
                    quote! { #gates::#tr },
                    quote! { #td },
                    &self_d,
                ),
                Shape::OneN => (
                    head_rt(&[], &[p_d.clone(), p_n.clone()]),
                    quote! { #gates::#tr<__N> },
                    quote! { #td, __N },
                    &self_d,
                ),
            };
            let check = dim_fn(f, args);
            let w =
                wher_rt(&[quote! { #gates::Bool<{ #check }>: #gates::Value<Is = #gates::True> }]);
            out.extend(quote! {
                #[diagnostic::do_not_recommend]
                #h #target for #of #w {}
            });
        }
        out
    };
    // The bounds an operator asks for: `Self: Refusal<Rhs>` for each.
    let pair = |names: &[&str], a: &TokenStream2, b: &TokenStream2| -> Vec<TokenStream2> {
        names
            .iter()
            .map(|n| {
                let n = syn::Ident::new(n, Span::call_site());
                quote! { #a: #gates::#n<#b> }
            })
            .collect()
    };
    let one = |names: &[&str], a: &TokenStream2| -> Vec<TokenStream2> {
        names
            .iter()
            .map(|n| {
                let n = syn::Ident::new(n, Span::call_site());
                quote! { #a: #gates::#n }
            })
            .collect()
    };
    let one_n = |names: &[&str], a: &TokenStream2| -> Vec<TokenStream2> {
        names
            .iter()
            .map(|n| {
                let n = syn::Ident::new(n, Span::call_site());
                quote! { #a: #gates::#n<__N> }
            })
            .collect()
    };
    let add_ab = pair(ADD, &self_a, &self_b);
    let sub_ab = pair(SUB, &self_a, &self_b);
    let sub_assign_ab = pair(&[SUB, &["SubInPlace"]].concat(), &self_a, &self_b);
    let mul_ab = pair(&["MulExists"], &self_a, &self_b);
    let div_ab = pair(&["DivExists"], &self_a, &self_b);
    let mul_assign_ab = pair(&["MulExists", "MulInPlace"], &self_a, &self_b);
    let div_assign_ab = pair(&["DivExists", "DivInPlace"], &self_a, &self_b);
    let scale_d = one(SCALE, &self_d);
    let add_dd = pair(ADD, &self_d, &self_d);

    // The impls a generic number type cannot have (the orphan rule): `N * Qty`
    // and `From<Qty<ONE>> for N`. They are written for the family's own number
    // type when it is concrete, and once per primitive when it is a parameter:
    // their heads carry the family's parameters without the number type.
    let others_prim: Vec<TokenStream2> = generics
        .params
        .iter()
        .filter(|p| !matches!(p, GenericParam::Const(c) if c.ident == *d_ident))
        .filter(|p| !matches!(p, GenericParam::Type(t) if Some(&t.ident) == v_ident))
        .map(param_decl)
        .collect();
    let head_prim = |extra: &[TokenStream2]| -> TokenStream2 {
        let all: Vec<&TokenStream2> = others_prim.iter().chain(extra.iter()).collect();
        if all.is_empty() {
            quote! { const impl }
        } else {
            quote! { const impl<#(#all),*> }
        }
    };
    let h_zero_prim = head_prim(&[]);
    let h_d_prim = head_prim(::core::slice::from_ref(&p_d));
    // Calls `f(number, Qty<D, number>, Qty<ONE, number>)` for each such number
    // type.
    let per_number = |f: &mut dyn FnMut(&TokenStream2, &TokenStream2, &TokenStream2)| {
        if v_ident.is_some() {
            for p in PRIMITIVES {
                let p = syn::Ident::new(p, Span::call_site());
                let p = quote! { #p };
                f(
                    &p,
                    &ty_v(&td, Some(&p)),
                    &ty_v(&quote! { { #dim_zero } }, Some(&p)),
                );
            }
        } else {
            f(&quote! { #v }, &self_d, &zero_ty);
        }
    };
    // `N * Qty`: scaling from the left, with the scaling refusals.
    let left_scalar = {
        let mut out = TokenStream2::new();
        per_number(&mut |p, self_p, _| {
            let mut preds = one(SCALE, self_p);
            preds.push(quote! { #p: [const] ::core::ops::Mul<Output = #p> });
            let w = if v_ident.is_some() {
                quote! { where #(#preds,)* }
            } else {
                wher(&preds)
            };
            let build = ctor(quote! { self * rhs.#member });
            out.extend(quote! {
                #h_d_prim ::core::ops::Mul<#self_p> for #p #w {
                    type Output = #self_p;
                    fn mul(self, rhs: #self_p) -> #self_p { #build }
                }
            });
        });
        out
    };

    // A quantity: every member that can be scaled. The supertraits are the
    // impls below; a point or a label that cannot be added to itself is not a
    // `Quantity`, with the scaling refusal's message.
    let quantity = {
        let mut preds = scale_d.clone();
        preds.push(quote! {
            #self_d: ::core::marker::Copy
                + ::core::cmp::PartialOrd
                + [const] ::core::ops::Add<Output = #self_d>
                + [const] ::core::ops::Mul<#v, Output = #self_d>
                + [const] ::core::ops::Div<#v, Output = #self_d>
                + [const] #yadc::Unit<#self_d, #v>
        });
        let w = wher(&preds);
        quote! {
            #h_d #yadc::Quantity for #self_d #w {
                type Number = #v;
            }
        }
    };

    // Every amount is a unit of its own kind: `KILOPASCAL.value(p)` is how
    // many kilopascals `p` is, `KILOPASCAL.quantity(2.0)` two of them.
    // Building scales, so it has the scaling refusals.
    let unit_of_self = {
        let mut preds = vec![b_copy.clone(), b_mul.clone(), b_div.clone()];
        preds.extend(scale_d.iter().cloned());
        let w = wher(&preds);
        let build = ctor(quote! { self.#member * value });
        quote! {
            #h_d #yadc::Unit<#self_d, #v> for #self_d #w {
                fn quantity(&self, value: #v) -> #self_d { #build }
                fn value(&self, quantity: #self_d) -> #v { quantity.#member / self.#member }
            }
        }
    };

    // A unit is called like the function it is: on a number it builds, on a
    // quantity it reads - `KILOMETRE(3.0)`, `FOOT(d)`. The
    // three std function traits, once per direction, each over `Unit`.
    let callable = |h: &TokenStream2, me: &TokenStream2, q: &TokenStream2| -> TokenStream2 {
        let unit = quote! { #yadc::Unit<#q, #v> };
        let w = wher(&[quote! { #me: [const] #unit }, quote! { #me: [const] ::core::marker::Destruct }]);
        let one = |arg: &TokenStream2, out: &TokenStream2, call: TokenStream2| {
            quote! {
                #h ::core::ops::FnOnce<(#arg,)> for #me #w {
                    type Output = #out;
                    extern "rust-call" fn call_once(self, (x,): (#arg,)) -> #out {
                        let this = &self;
                        #call
                    }
                }
                #h ::core::ops::FnMut<(#arg,)> for #me #w {
                    extern "rust-call" fn call_mut(&mut self, (x,): (#arg,)) -> #out {
                        let this = &*self;
                        #call
                    }
                }
                #h ::core::ops::Fn<(#arg,)> for #me #w {
                    extern "rust-call" fn call(&self, (x,): (#arg,)) -> #out {
                        let this = self;
                        #call
                    }
                }
            }
        };
        let build = one(&quote! { #v }, q, quote! { <#me as #unit>::quantity(this, x) });
        let read = one(q, &quote! { #v }, quote! { <#me as #unit>::value(this, x) });
        quote! { #build #read }
    };
    let calls = if opts.has("Fn") {
        callable(&h_d, &self_d, &self_d)
    } else {
        quote! {}
    };

    let bin_op = |h: &TokenStream2,
                  rhs: &TokenStream2,
                  trait_name: &str,
                  method: &str,
                  out_ty: &TokenStream2,
                  bounds: &[TokenStream2],
                  body: TokenStream2| {
        let tr = syn::Ident::new(trait_name, Span::call_site());
        let m = syn::Ident::new(method, Span::call_site());
        let mut preds = vec![quote! { #out_ty: ::core::marker::Sized }];
        preds.extend(bounds.iter().cloned());
        let w = wher(&preds);
        quote! {
            #h ::core::ops::#tr<#rhs> for #self_a #w {
                type Output = #out_ty;
                fn #m(self, rhs: #rhs) -> Self::Output { #body }
            }
        }
    };
    let with = |a: &[TokenStream2], b: &[&TokenStream2]| -> Vec<TokenStream2> {
        a.iter()
            .cloned()
            .chain(b.iter().map(|t| (*t).clone()))
            .collect()
    };
    let rhs_ref = quote! { &'__a #self_b };

    // The cross-type operators: `+ - * /` between any two members, each
    // asking for its refusals. A sum keeps its left operand's type
    // (`absolute + step` is absolute, a point plus an amount the same
    // point). `*` and `/` also take a borrowed right operand - which keeps a
    // refused scaling (`altitude * 2.0`) reported as one.
    let ops = {
        let mul = bin_op(
            &h_ab,
            &self_b,
            "Mul",
            "mul",
            &mul_ty,
            &with(&mul_ab, &[&b_mul]),
            ctor(quote! { self.#member * rhs.#member }),
        );
        let mul_ref = bin_op(
            &h_ab_ref,
            &rhs_ref,
            "Mul",
            "mul",
            &mul_ty,
            &with(&mul_ab, &[&b_mul, &b_copy]),
            ctor(quote! { self.#member * rhs.#member }),
        );
        let div = bin_op(
            &h_ab,
            &self_b,
            "Div",
            "div",
            &div_ty,
            &with(&div_ab, &[&b_div]),
            ctor(quote! { self.#member / rhs.#member }),
        );
        let div_ref = bin_op(
            &h_ab_ref,
            &rhs_ref,
            "Div",
            "div",
            &div_ty,
            &with(&div_ab, &[&b_div, &b_copy]),
            ctor(quote! { self.#member / rhs.#member }),
        );
        let add = bin_op(
            &h_ab,
            &self_b,
            "Add",
            "add",
            &self_a,
            &with(&add_ab, &[&b_add]),
            ctor(quote! { self.#member + rhs.#member }),
        );
        let sub = bin_op(
            &h_ab,
            &self_b,
            "Sub",
            "sub",
            &sub_ty,
            &with(&sub_ab, &[&b_sub]),
            ctor(quote! { self.#member - rhs.#member }),
        );
        quote! { #mul #mul_ref #div #div_ref #add #sub }
    };

    // Scaling by a number: `Mul<N>` / `Div<N>`, with the scaling refusals
    // (scaling is repeated addition: no point, no label that cannot be added
    // to itself).
    let scalar_ops = {
        let w_mul = wher(&with(&scale_d, &[&b_mul]));
        let w_div = wher(&with(&scale_d, &[&b_div]));
        let m = ctor(quote! { self.#member * rhs });
        let d = ctor(quote! { self.#member / rhs });
        quote! {
            #h_d ::core::ops::Mul<#value_ty> for #self_d #w_mul {
                type Output = Self;
                fn mul(self, rhs: #value_ty) -> Self { #m }
            }
            #h_d ::core::ops::Div<#value_ty> for #self_d #w_div {
                type Output = Self;
                fn div(self, rhs: #value_ty) -> Self { #d }
            }
        }
    };

    // --- extras: on by default, or exactly those `#[quantity(impl(..))]` lists
    let mut extras = TokenStream2::new();
    let h_d_rt_display = head_rt(&[], ::core::slice::from_ref(&p_d));

    if opts.has("Display") {
        // The number with the caller's flags, then a space and the
        // dimension's own `Display` - nothing after a plain number. The bounds
        // are higher-ranked so a number or dimension type without `Display`
        // simply lacks the impl.
        let w = wher_rt(&[
            quote! { for<'__v> #v: ::core::fmt::Display },
            quote! { for<'__v> #d_ty: ::core::fmt::Display },
        ]);
        extras.extend(quote! {
            #h_d_rt_display ::core::fmt::Display for #self_d #w {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    #yadc::display::write_quantity(f, &self.#member, &#d_ident)
                }
            }
        });
    }

    if opts.has("Neg") {
        // Negating is scaling by -1: the scaling refusals.
        let w = wher(&with(&scale_d, &[&b_neg]));
        let build = ctor(quote! { -self.#member });
        extras.extend(quote! {
            #h_d ::core::ops::Neg for #self_d #w {
                type Output = Self;
                fn neg(self) -> Self { #build }
            }
        });
    }

    if opts.has("AddAssign") {
        // In place, so the type cannot change - which `+` guarantees: a sum
        // keeps its left operand's type.
        let w = wher(&with(&add_ab, &[&b_add_assign]));
        let w_ref = wher(&with(&add_ab, &[&b_add_assign, &b_copy]));
        extras.extend(quote! {
            #h_ab ::core::ops::AddAssign<#self_b> for #self_a #w {
                fn add_assign(&mut self, rhs: #self_b) { self.#member += rhs.#member; }
            }
            #h_ab_ref ::core::ops::AddAssign<#rhs_ref> for #self_a #w_ref {
                fn add_assign(&mut self, rhs: #rhs_ref) { self.#member += rhs.#member; }
            }
        });
    }

    if opts.has("SubAssign") {
        let w = wher(&with(&sub_assign_ab, &[&b_sub_assign]));
        let w_ref = wher(&with(&sub_assign_ab, &[&b_sub_assign, &b_copy]));
        extras.extend(quote! {
            #h_ab ::core::ops::SubAssign<#self_b> for #self_a #w {
                fn sub_assign(&mut self, rhs: #self_b) { self.#member -= rhs.#member; }
            }
            #h_ab_ref ::core::ops::SubAssign<#rhs_ref> for #self_a #w_ref {
                fn sub_assign(&mut self, rhs: #rhs_ref) { self.#member -= rhs.#member; }
            }
        });
    }

    // `*=` / `/=`: by a number (the scaling refusals), and by a member
    // whose product or quotient is the left operand's type again - in
    // practice a plain number; owned and borrowed.
    for (tr, m, op, scalar_b, assign_ab) in [
        (
            "MulAssign",
            "mul_assign",
            quote! { *= },
            &b_mul_assign,
            &mul_assign_ab,
        ),
        (
            "DivAssign",
            "div_assign",
            quote! { /= },
            &b_div_assign,
            &div_assign_ab,
        ),
    ] {
        if !opts.has(tr) {
            continue;
        }
        let t = syn::Ident::new(tr, Span::call_site());
        let m = syn::Ident::new(m, Span::call_site());
        let w_n = wher(&with(&scale_d, &[scalar_b]));
        let w_q = wher(&with(assign_ab, &[scalar_b]));
        let w_q_ref = wher(&with(assign_ab, &[scalar_b, &b_copy]));
        extras.extend(quote! {
            #h_d ::core::ops::#t<#value_ty> for #self_d #w_n {
                fn #m(&mut self, rhs: #value_ty) { self.#member #op rhs; }
            }
            #h_ab ::core::ops::#t<#self_b> for #self_a #w_q {
                fn #m(&mut self, rhs: #self_b) { self.#member #op rhs.#member; }
            }
            #h_ab_ref ::core::ops::#t<#rhs_ref> for #self_a #w_q_ref {
                fn #m(&mut self, rhs: #rhs_ref) { self.#member #op rhs.#member; }
            }
        });
    }

    if opts.has("Inv") {
        // `x.inv()` is `1 / x`: the dimension is `DIMENSIONLESS / D`.
        let inv_d = dim_fn("reciprocal_or_one", quote! { #td });
        let inv_ty = ty_of(&quote! { { #inv_d } });
        let mut preds = one(&["InvExists"], &self_d);
        preds.extend([
            quote! { #inv_ty: ::core::marker::Sized },
            b_div_rt.clone(),
            quote! { #v: #yadc::num_traits::One },
        ]);
        let w = wher_rt(&preds);
        let h_d_rt = head_rt(&[], ::core::slice::from_ref(&p_d));
        let build = ctor(quote! { <#v as #yadc::num_traits::One>::one() / self.#member });
        extras.extend(quote! {
            #h_d_rt #yadc::num_traits::Inv for #self_d #w {
                type Output = #inv_ty;
                fn inv(self) -> Self::Output { #build }
            }
        });
    }

    // `x^N` and the `N`-th root, for a compile-time `N`: the result type is
    // the dimension's power (`Dimension::powi`) / root (`PartialRoot`), each
    // with its refusals. Calls read `length.powi::<2>()` through yadc's
    // blanket `PowiAs` / `RootAs` - no inherent methods.
    // The number bounds are higher-ranked (`for<'__v> N: ConstOne`), not
    // plain: with a concrete number type lacking them (`ConstOne`, `Real`) a
    // plain bound naming only that type would be trivially false - an error -
    // where a higher-ranked one just leaves the impl out.
    let qv = quote! { for<'__v> #v };
    if opts.has("Powi") {
        let h_dn = head(&[], &[p_d.clone(), p_n.clone()]);
        let pow_d = dim_fn("power_or_one", quote! { #td, __N });
        let pow_ty = ty_of(&quote! { { #pow_d } });
        let one_v = quote! { <#v as #yadc::num_traits::ConstOne>::ONE };
        let mut preds = one_n(&["PowiExists"], &self_d);
        preds.extend([
            quote! { #pow_ty: ::core::marker::Sized },
            quote! { #qv: ::core::marker::Copy + #yadc::num_traits::ConstOne },
            quote! { #qv: [const] ::core::ops::Mul<Output = #v> + [const] ::core::ops::Div<Output = #v> },
        ]);
        let w = wher(&preds);
        let build = ctor(quote! { if __N < 0 { #one_v / __acc } else { __acc } });
        extras.extend(quote! {
            #h_dn #yadc::Powi<__N> for #self_d #w {
                type Output = #pow_ty;
                // The number type has `*` but maybe not `*=`.
                #[allow(clippy::assign_op_pattern)]
                fn powi_into(self) -> Self::Output {
                    let mut __acc = #one_v;
                    let mut __i = 0;
                    while __i < __N.unsigned_abs() {
                        __acc = __acc * self.#member;
                        __i += 1;
                    }
                    #build
                }
            }
        });
    }
    if opts.has("Root") {
        let h_dn_rt = head_rt(&[], &[p_d.clone(), p_n.clone()]);
        let root_d = dim_fn("root_or_one", quote! { #td, __N });
        let root_ty = ty_of(&quote! { { #root_d } });
        let mut preds = one_n(&["RootHasSquare", "RootOfLabel", "RootExact"], &self_d);
        preds.extend([
            quote! { #root_ty: ::core::marker::Sized },
            quote! { #qv: #yadc::num_traits::real::Real },
        ]);
        let w = wher_rt(&preds);
        let build = ctor(quote! {
            #yadc::num_traits::real::Real::powf(
                self.#member,
                <#v as #yadc::num_traits::One>::one()
                    / <#v as #yadc::num_traits::NumCast>::from(__N).unwrap(),
            )
        });
        extras.extend(quote! {
            #h_dn_rt #yadc::Root<__N> for #self_d #w {
                type Output = #root_ty;
                fn root_into(self) -> Self::Output { #build }
            }
        });
    }

    if opts.has("Sum") {
        // Seeded with `num_traits::Zero`, the numbers summed with their `+=`;
        // the refusals of `+` apply.
        let h_d_rt = head_rt(&[], ::core::slice::from_ref(&p_d));
        let h_d_ref = head_rt(&lt_a, ::core::slice::from_ref(&p_d));
        let w = wher_rt(&with(&add_dd, &[&b_zero, &b_add_assign_rt]));
        let w_ref = wher_rt(&with(&add_dd, &[&b_zero, &b_add_assign_rt, &b_copy]));
        let seed = ctor(quote! { <#value_ty as #yadc::num_traits::Zero>::zero() });
        extras.extend(quote! {
            #h_d_rt ::core::iter::Sum<#self_d> for #self_d #w {
                fn sum<__I>(iter: __I) -> Self
                where
                    __I: ::core::iter::Iterator<Item = #self_d>,
                {
                    iter.fold(#seed, |mut __acc, __q| { __acc.#member += __q.#member; __acc })
                }
            }
            #h_d_ref ::core::iter::Sum<&'__a #self_d> for #self_d #w_ref {
                fn sum<__I>(iter: __I) -> Self
                where
                    __I: ::core::iter::Iterator<Item = &'__a #self_d>,
                {
                    iter.fold(#seed, |mut __acc, __q| { __acc.#member += (*__q).#member; __acc })
                }
            }
        });
    }

    // A change of number type, for a family generic over it:
    // `KILOPASCAL.cast::<f32>()`.
    if opts.explicit("Cast") && v_ident.is_none() {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "`Cast` changes a family's number type, so the family needs one as a parameter: \
             `struct Qty<const D: Dim, N = f64>(N)`",
        ));
    }
    if opts.has("Cast") && v_ident.is_some() {
        let w_ty = quote! { __W };
        let out_ty = ty_v(&td, Some(&w_ty));
        let h = head(&[], &[p_d.clone(), w_ty.clone()]);
        let w = wher(&[
            quote! { #v: [const] #yadc::Cast<__W, Output = __W> },
            quote! { #out_ty: ::core::marker::Sized },
        ]);
        let build = ctor(quote! { <#v as #yadc::Cast<__W>>::cast_into(self.#member) });
        extras.extend(quote! {
            #h #yadc::Cast<__W> for #self_d #w {
                type Output = #out_ty;
                fn cast_into(self) -> #out_ty { #build }
            }
        });
    }

    if opts.has("From") {
        // A dimensionless quantity *is* a number: `f64::from(a / b)`.
        per_number(&mut |p, _, zero_p| {
            let w = if value_is_generic {
                quote! {}
            } else {
                wher(&[])
            };
            extras.extend(quote! {
                #h_zero_prim ::core::convert::From<#zero_p> for #p #w {
                    fn from(quantity: #zero_p) -> Self { quantity.#member }
                }
            });
        });
        // A lossless change of number type: `Qty<D, f32>` into `Qty<D, f64>`.
        // A generic `impl<N, W: From<N>> From<Qty<D, N>> for Qty<D, W>` would
        // overlap core's `impl<T> From<T> for T` (at `N = W`), so it is one
        // impl per pair of primitives that std converts losslessly.
        if v_ident.is_some() {
            for (from, tos) in LOSSLESS {
                let f = syn::Ident::new(from, Span::call_site());
                for to in *tos {
                    let t = syn::Ident::new(to, Span::call_site());
                    let src = ty_v(&td, Some(&quote! { #f }));
                    let dst = ty_v(&td, Some(&quote! { #t }));
                    let build = ctor(quote! { <#t as ::core::convert::From<#f>>::from(q.#member) });
                    extras.extend(quote! {
                        #h_d_prim ::core::convert::From<#src> for #dst {
                            fn from(q: #src) -> Self { #build }
                        }
                    });
                }
            }
        }
    }

    Ok(quote! {
        #refusals
        #quantity
        #unit_of_self
        #calls
        #ops
        #scalar_ops
        #left_scalar
        #extras
    })
}

/// The refusal traits of `yadc::gates` and their checks, with the operands
/// each check reads.
#[derive(Clone, Copy)]
enum Shape {
    /// `Qty<A>: Tr<Qty<B>>`, checked on `(A, B)`.
    Pair,
    /// `Qty<D>: Tr`, checked on `D`.
    One,
    /// `Qty<D>: Tr<N>`, checked on `(D, N)`.
    OneN,
}

const REFUSALS: &[(&str, &str, Shape)] = &[
    ("AddSameDimension", "add_same_dimension", Shape::Pair),
    ("AddNotTwoPoints", "add_not_two_points", Shape::Pair),
    ("AddInOrder", "add_in_order", Shape::Pair),
    ("AddLabels", "add_labels", Shape::Pair),
    ("SubSameDimension", "sub_same_dimension", Shape::Pair),
    ("SubNotFromAmount", "sub_not_from_amount", Shape::Pair),
    ("SubLabels", "sub_labels", Shape::Pair),
    ("SubInPlace", "sub_in_place", Shape::Pair),
    ("MulExists", "mul_exists", Shape::Pair),
    ("DivExists", "div_exists", Shape::Pair),
    ("MulInPlace", "mul_in_place", Shape::Pair),
    ("DivInPlace", "div_in_place", Shape::Pair),
    ("ScaleNotPoint", "scale_not_point", Shape::One),
    ("ScaleLabel", "scale_label", Shape::One),
    ("PowiExists", "powi_exists", Shape::OneN),
    ("InvExists", "inv_exists", Shape::One),
    ("RootHasSquare", "root_has_square", Shape::OneN),
    ("RootOfLabel", "root_of_label", Shape::OneN),
    ("RootExact", "root_exact", Shape::OneN),
];
const ADD: &[&str] = &[
    "AddSameDimension",
    "AddNotTwoPoints",
    "AddInOrder",
    "AddLabels",
];
const SUB: &[&str] = &[
    "SubSameDimension",
    "SubNotFromAmount",
    "SubLabels",
];
const SCALE: &[&str] = &["ScaleNotPoint", "ScaleLabel"];

/// The primitive number types: the impls a generic number type cannot have
/// (`N * Qty`, `From<Qty<ONE>> for N`) are written once for each.
pub(crate) const PRIMITIVES: &[&str] = &[
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32",
    "f64",
];

/// The extras `#[derive(Quantity)]` writes beyond `Quantity`, its
/// supertraits and the cross-type operators: all of them by default, or
/// exactly those `#[quantity(impl(..))]` lists.
const EXTRAS: &[&str] = &[
    "Display",
    "Neg",
    "AddAssign",
    "SubAssign",
    "MulAssign",
    "DivAssign",
    "Powi",
    "Root",
    "Inv",
    "Sum",
    "Cast",
    "From",
    "Fn",
];

/// `#[quantity(impl(..))]`: which extras to write.
struct Options {
    extras: Option<Vec<String>>,
}

impl Options {
    /// Listed in `#[quantity(impl(..))]`, not just on by default.
    fn explicit(&self, name: &str) -> bool {
        self.extras.as_ref().is_some_and(|l| l.iter().any(|e| e == name))
    }

    fn has(&self, name: &str) -> bool {
        match &self.extras {
            None => true,
            Some(list) => list.iter().any(|e| e == name),
        }
    }

    fn parse(input: &DeriveInput) -> syn::Result<Self> {
        let mut extras: Option<Vec<String>> = None;
        for attr in &input.attrs {
            if !attr.path().is_ident("quantity") {
                continue;
            }
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("impl") {
                    let list = extras.get_or_insert_with(Vec::new);
                    meta.parse_nested_meta(|item| {
                        let Some(id) = item.path.get_ident() else {
                            return Err(item.error("expected a trait name"));
                        };
                        let name = id.to_string();
                        if !EXTRAS.contains(&name.as_str()) {
                            return Err(item.error(format!(
                                "`{name}` is not an extra of `#[derive(Quantity)]`; expected one \
                                 of {}",
                                EXTRAS.join(", ")
                            )));
                        }
                        list.push(name);
                        Ok(())
                    })
                } else {
                    Err(meta.error("expected `impl(..)`: `#[quantity(impl(Neg, Powi, ..))]`"))
                }
            })?;
        }
        Ok(Options { extras })
    }
}

/// The single field holding a quantity's value - positional or named, selected
/// by `#[value]` or by being the only field that is not `PhantomData`. The
/// derive never assumes `.0`.
struct ValueField {
    member: syn::Member,
    ty: Type,
    /// The `PhantomData` fields, filled with
    /// `PhantomData` wherever the struct is built.
    phantoms: Vec<syn::Member>,
}

impl ValueField {
    /// Build the struct from one value expression, as a struct expression
    /// that works for named and positional fields alike
    /// (`Qty { 0: v, 1: PhantomData }`).
    fn construct(&self, name: &syn::Ident, expr: TokenStream2) -> TokenStream2 {
        let member = &self.member;
        let phantoms = &self.phantoms;
        quote! { #name { #member: #expr, #(#phantoms: ::core::marker::PhantomData,)* } }
    }
}

fn value_field(data: &Data, span: &impl quote::ToTokens) -> syn::Result<ValueField> {
    let fields = match data {
        Data::Struct(s) => &s.fields,
        _ => {
            return Err(syn::Error::new_spanned(
                span,
                "#[derive(Quantity)] only works on structs",
            ));
        }
    };
    let shape = "a static family holds one value field (and any `PhantomData`), e.g. \
                 `struct Qty<const D: Dim>(f64)`, `struct Qty<const D: Dim, V = f64>(V)` or \
                 `struct Qty<const D: Dim> { #[value] v: f64 }`";
    let all: Vec<(syn::Member, &syn::Field)> = fields
        .iter()
        .enumerate()
        .map(|(i, f)| (member_of(i, f), f))
        .collect();
    let marked: Vec<&(syn::Member, &syn::Field)> = all
        .iter()
        .filter(|(_, f)| has_attr(&f.attrs, "value"))
        .collect();
    let data_fields: Vec<&(syn::Member, &syn::Field)> =
        all.iter().filter(|(_, f)| !is_phantom(&f.ty)).collect();
    let chosen = match (marked.as_slice(), data_fields.as_slice()) {
        ([one], _) => *one,
        ([], [one]) => *one,
        _ => return Err(syn::Error::new_spanned(span, shape)),
    };
    let mut phantoms = Vec::new();
    for (m, f) in &all {
        if *m == chosen.0 {
            continue;
        }
        if !is_phantom(&f.ty) {
            return Err(syn::Error::new_spanned(f, shape));
        }
        phantoms.push(m.clone());
    }
    Ok(ValueField {
        member: chosen.0.clone(),
        ty: chosen.1.ty.clone(),
        phantoms,
    })
}

/// The const generic carrying the dimension: the one marked `#[dim]`, else the
/// struct's only const parameter. Any other generics are passed through.
fn dim_const_param<'a>(
    generics: &'a syn::Generics,
    span: &impl quote::ToTokens,
) -> syn::Result<(&'a syn::Ident, &'a Type)> {
    let consts: Vec<&syn::ConstParam> = generics
        .params
        .iter()
        .filter_map(|p| match p {
            GenericParam::Const(c) => Some(c),
            _ => None,
        })
        .collect();
    let marked: Vec<&&syn::ConstParam> = consts
        .iter()
        .filter(|c| c.attrs.iter().any(|a| a.path().is_ident("dim")))
        .collect();
    match (marked.len(), consts.len()) {
        (1, _) => Ok((&marked[0].ident, &marked[0].ty)),
        (0, 1) => Ok((&consts[0].ident, &consts[0].ty)),
        (0, 0) => Err(syn::Error::new_spanned(
            span,
            "expected a const generic parameter holding the dimension, \
             e.g. `struct Qty<const D: MyDim>(f64)`",
        )),
        (0, _) => Err(syn::Error::new_spanned(
            span,
            "several const generic parameters: mark the dimension one `#[dim]`, \
             e.g. `struct Qty<const N: usize, #[dim] const D: MyDim>(f64)`",
        )),
        _ => Err(syn::Error::new_spanned(
            span,
            "only one const generic parameter may be marked `#[dim]`",
        )),
    }
}

/// The primitive pairs std converts losslessly (`impl From<V> for W`), for
/// the family's `From<Qty<D, V>> for Qty<D, W>`.
const LOSSLESS: &[(&str, &[&str])] = &[
    ("f32", &["f64"]),
    ("i8", &["i16", "i32", "i64", "i128", "isize", "f32", "f64"]),
    ("i16", &["i32", "i64", "i128", "isize", "f32", "f64"]),
    ("i32", &["i64", "i128", "f64"]),
    ("i64", &["i128"]),
    (
        "u8",
        &[
            "u16", "u32", "u64", "u128", "usize", "i16", "i32", "i64", "i128", "isize", "f32",
            "f64",
        ],
    ),
    (
        "u16",
        &[
            "u32", "u64", "u128", "usize", "i32", "i64", "i128", "f32", "f64",
        ],
    ),
    ("u32", &["u64", "u128", "i64", "i128", "f64"]),
    ("u64", &["u128", "i128"]),
];
