//! `#[derive(ConstOps)]`: the std operators of a struct that carries a value
//! and const parameters, lifted over them - the value by its own operators,
//! each lifted parameter by its type's.

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::parse::Parse;
use syn::{Data, DeriveInput, Expr, GenericParam, Token, Type};

pub(crate) fn expand(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let opts = Options::parse(input)?;
    let name = &input.ident;
    let generics = &input.generics;

    let value = value_field(&input.data, name)?;
    let value_ty = &value.ty;
    let lifted = lifted_params(generics, name)?;
    let is_lifted = |id: &syn::Ident| lifted.iter().any(|l| l.ident == *id);

    // The checks and the parameter's results, per derive, beside the struct.
    let m = format_ident!("__const_ops_{}", name);

    // --- generics passthrough ------------------------------------------------
    // Only the lifted const parameters are ours; every impl repeats the other
    // parameters verbatim and substitutes the lifted slots with what that
    // impl needs.
    let others: Vec<TokenStream2> = generics
        .params
        .iter()
        .filter(|p| !matches!(p, GenericParam::Const(c) if is_lifted(&c.ident)))
        .map(param_decl)
        .collect();
    // Lifetimes go before type and const parameters, so an impl's own borrow
    // lifetime is spliced ahead of the struct's parameters. Every impl is a
    // `const impl`, except `Sum` and the comparisons across number types,
    // which use `head_rt`. Value-type bounds are `[const]`: const only when
    // used in a const context.
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
    // The number type, when it is one of the struct's own parameters
    // (`S<const P: T, N>(N)`).
    let v_ident: Option<&syn::Ident> = generics.params.iter().find_map(|p| match p {
        GenericParam::Type(t)
            if matches!(value_ty, Type::Path(tp)
            if tp.qself.is_none() && tp.path.is_ident(&t.ident)) =>
        {
            Some(&t.ident)
        }
        _ => None,
    });
    // The struct with these lifted parameters, in order, and this number
    // type (or its own).
    let ty_v = |params: &[TokenStream2], val: Option<&TokenStream2>| -> TokenStream2 {
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
                GenericParam::Const(c) => match lifted.iter().position(|l| l.ident == c.ident) {
                    Some(k) => params[k].clone(),
                    None => {
                        let i = &c.ident;
                        quote! { #i }
                    }
                },
            })
            .collect();
        quote! { #name<#(#args),*> }
    };
    let ty_of = |params: &[TokenStream2]| -> TokenStream2 { ty_v(params, None) };
    let user_preds: Vec<TokenStream2> = generics
        .where_clause
        .iter()
        .flat_map(|w| w.predicates.iter())
        .map(|p| quote! { #p })
        .collect();
    let wher_rt = |extra: &[TokenStream2]| -> TokenStream2 {
        if user_preds.is_empty() && extra.is_empty() {
            quote! {}
        } else {
            quote! { where #(#user_preds,)* #(#extra,)* }
        }
    };

    // --- value field ---------------------------------------------------------
    let member = &value.member;
    let ctor = |expr: TokenStream2| -> TokenStream2 { value.construct(name, expr) };

    let v = value_ty;
    // A generic number type may have a destructor; the const impls drop
    // values, so they need it to be `[const] Destruct`.
    let value_is_generic = v_ident.is_some();
    let const_extra: Vec<TokenStream2> = if value_is_generic {
        vec![quote! { #v: [const] ::core::marker::Destruct }]
    } else {
        Vec::new()
    };
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
    let b_add_assign_rt = quote! { #v: ::core::ops::AddAssign };
    let b_default = quote! { #v: ::core::default::Default };
    let b_copy = quote! { #v: ::core::marker::Copy };

    // --- the parameters' results --------------------------------------------
    // Each lifted parameter `P` has three names: its own, `__A_P` in a left
    // operand and `__B_P` in a right one.
    let named = |prefix: &str| -> Vec<syn::Ident> {
        lifted
            .iter()
            .map(|l| format_ident!("{}{}", prefix, l.ident))
            .collect()
    };
    let (qa, qb) = (named("__A_"), named("__B_"));
    let toks =
        |ids: &[syn::Ident]| -> Vec<TokenStream2> { ids.iter().map(|i| quote! { #i }).collect() };
    let tp = toks(&named(""));
    let (ta, tb) = (toks(&qa), toks(&qb));
    // Every lifted parameter's result of `f` (`__product`, ..) of `a` and
    // `b`, a const argument each.
    let results = |f: &str, a: &[TokenStream2], b: &[TokenStream2]| -> Vec<TokenStream2> {
        lifted
            .iter()
            .zip(a.iter().zip(b))
            .map(|(l, (a, b))| {
                let f = format_ident!("{}_{}", f, l.ident);
                quote! { { #m::#f(#a, #b) } }
            })
            .collect()
    };
    // A check of every lifted parameter at once, `__can_mul(A.., B..)`.
    let check_call = |f: &str, a: &[TokenStream2], b: &[TokenStream2]| -> TokenStream2 {
        let f = syn::Ident::new(f, Span::call_site());
        quote! { #m::#f(#(#a,)* #(#b),*) }
    };
    let ones: Vec<TokenStream2> = lifted
        .iter()
        .map(|l| {
            let c = format_ident!("__ONE_{}", l.ident);
            quote! { #m::#c }
        })
        .collect();

    let decls = |ids: &[syn::Ident]| -> Vec<TokenStream2> {
        lifted
            .iter()
            .zip(ids)
            .map(|(l, i)| {
                let t = &l.ty;
                quote! { const #i: #t }
            })
            .collect()
    };
    let p_p = decls(&named(""));
    let p_ab: Vec<TokenStream2> = decls(&qa).into_iter().chain(decls(&qb)).collect();
    let lt_a = [quote! { '__a }];
    let self_p = ty_of(&tp);
    let self_a = ty_of(&ta);
    let self_b = ty_of(&tb);
    let result_ty = |f: &str| ty_of(&results(f, &ta, &tb));
    let (add_ty, sub_ty, mul_ty, div_ty) = (
        result_ty("__sum"),
        result_ty("__difference"),
        result_ty("__product"),
        result_ty("__quotient"),
    );

    let h_p = head(&[], &p_p);
    let h_ab = head(&[], &p_ab);
    let h_ab_ref = head(&lt_a, &p_ab);

    // --- checks ----------------------------------------------------------------
    // One impl per check trait, for every operand pair where the check holds.
    // The check is a const `bool` read through an associated type
    // (`__Bool<{ .. }>: __Value<Is = __True>`), which the compiler evaluates
    // instead of unifying it with `true`; `do_not_recommend` makes a failing
    // check report the check trait - and its message - at the operator.
    let checks = {
        let mut out = TokenStream2::new();
        for (tr, f, ..) in CHECKS {
            let tr = syn::Ident::new(tr, Span::call_site());
            let check = check_call(f, &ta, &tb);
            let w = wher_rt(&[quote! { #m::__Bool<{ #check }>: #m::__Value<Is = #m::__True> }]);
            let h = head_rt(&[], &p_ab);
            out.extend(quote! {
                #[diagnostic::do_not_recommend]
                #h #m::#tr<#self_b> for #self_a #w {}
            });
        }
        out
    };
    let check = |tr: &str, a: &TokenStream2, b: &TokenStream2| -> TokenStream2 {
        let tr = syn::Ident::new(tr, Span::call_site());
        quote! { #a: #m::#tr<#b> }
    };
    let module = {
        let args = |ids: &[syn::Ident]| -> Vec<TokenStream2> {
            lifted
                .iter()
                .zip(ids)
                .map(|(l, i)| {
                    let t = &l.ty;
                    quote! { #i: #t }
                })
                .collect()
        };
        let traits = CHECKS.iter().map(|(tr, f, op, msg, label)| {
            let tr = syn::Ident::new(tr, Span::call_site());
            let f = syn::Ident::new(f, Span::call_site());
            let body = |a: &syn::Ident, b: &syn::Ident| match *op {
                "+" => quote! { (#a + #b).is_some() },
                "-" => quote! { (#a - #b).is_some() },
                "*" => quote! { (#a * #b).is_some() },
                "/" => quote! { (#a / #b).is_some() },
                "+=" => quote! { matches!(#a + #b, ::core::option::Option::Some(c) if c == #a) },
                "-=" => quote! { matches!(#a - #b, ::core::option::Option::Some(c) if c == #a) },
                "*=" => quote! { matches!(#a * #b, ::core::option::Option::Some(c) if c == #a) },
                _ => quote! { matches!(#a / #b, ::core::option::Option::Some(c) if c == #a) },
            };
            let bodies = qa.iter().zip(&qb).map(|(a, b)| body(a, b));
            let (da, db) = (args(&qa), args(&qb));
            quote! {
                /// Whether the operator exists for these two operands'
                /// parameters: whether it does for each.
                pub const fn #f(#(#da,)* #(#db),*) -> bool { true #(&& #bodies)* }
                #[diagnostic::on_unimplemented(message = #msg, label = #label)]
                pub trait #tr<R> {}
            }
        });
        let per_param = lifted.iter().map(|l| {
            let (i, t, one) = (&l.ident, &l.ty, &l.one);
            let c = format_ident!("__ONE_{}", i);
            let or_one = format_ident!("__or_one_{}", i);
            let [sum, difference, product, quotient] =
                ["__sum", "__difference", "__product", "__quotient"]
                    .map(|f| format_ident!("{}_{}", f, i));
            let doc = format!("`{i}` of a plain number.");
            quote! {
                #[doc = #doc]
                pub const #c: #t = #one;
                /// The parameter inside `Some`, or that of a plain number: a
                /// result type must exist even where its operator does not.
                pub const fn #or_one(p: ::core::option::Option<#t>) -> #t {
                    match p {
                        ::core::option::Option::Some(p) => p,
                        ::core::option::Option::None => #c,
                    }
                }
                pub const fn #sum(a: #t, b: #t) -> #t { #or_one(a + b) }
                pub const fn #difference(a: #t, b: #t) -> #t { #or_one(a - b) }
                pub const fn #product(a: #t, b: #t) -> #t { #or_one(a * b) }
                pub const fn #quotient(a: #t, b: #t) -> #t { #or_one(a / b) }
            }
        });
        let vis = &input.vis;
        let doc = format!("The parameters' results and operator checks of `{name}`'s `ConstOps`.");
        quote! {
            #[doc = #doc]
            #[doc(hidden)]
            #[allow(non_snake_case, dead_code, clippy::eq_op)]
            #vis mod #m {
                #[allow(unused_imports)]
                use super::*;

                #(#per_param)*

                /// A `bool` known at compile time, as a type.
                pub struct __Bool<const B: bool>;
                /// What a `__Bool` is: `__True` or `__False`.
                pub trait __Value {
                    type Is;
                }
                pub struct __True;
                pub struct __False;
                impl __Value for __Bool<true> {
                    type Is = __True;
                }
                impl __Value for __Bool<false> {
                    type Is = __False;
                }

                #(#traits)*
            }
        }
    };

    // The impls a generic number type cannot have (the orphan rule): `n * S`
    // and `n / S`. They are written for the struct's own number type when it
    // is concrete, and once per primitive when it is a parameter: their heads
    // carry the struct's parameters without it.
    let others_prim: Vec<TokenStream2> = generics
        .params
        .iter()
        .filter(|p| !matches!(p, GenericParam::Const(c) if is_lifted(&c.ident)))
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
    let h_p_prim = head_prim(&p_p);
    // Calls `f(number, Some(number))` for each such number, or `f(number,
    // None)` for the struct's own.
    let per_number = |f: &mut dyn FnMut(&TokenStream2, Option<&TokenStream2>)| {
        if v_ident.is_some() {
            for p in PRIMITIVES {
                let p = syn::Ident::new(p, Span::call_site());
                let p = quote! { #p };
                f(&p, Some(&p));
            }
        } else {
            f(&quote! { #v }, None);
        }
    };
    // `n * S`, scaling from the left, and `n / S`, a number divided by a
    // member: a member of the plain number's parameters divided by it.
    let left_scalar = {
        let mut out = TokenStream2::new();
        let quotient = results("__quotient", &ones, &tp);
        let one_args: Vec<TokenStream2> = ones.iter().map(|o| quote! { { #o } }).collect();
        per_number(&mut |p, val| {
            let self_n = ty_v(&tp, val);
            let one_n = ty_v(&one_args, val);
            let out_n = ty_v(&quotient, val);
            let w = |pred: TokenStream2| {
                if v_ident.is_some() {
                    quote! { where #pred }
                } else {
                    wher(&[pred])
                }
            };
            let w_mul = w(quote! { #p: [const] ::core::ops::Mul<Output = #p> });
            let c = check("DivResult", &one_n, &self_n);
            let w_div = w(quote! {
                #p: [const] ::core::ops::Div<Output = #p>, #out_n: ::core::marker::Sized, #c
            });
            let (mm, dd) = (
                ctor(quote! { self * rhs.#member }),
                ctor(quote! { self / rhs.#member }),
            );
            out.extend(quote! {
                #h_p_prim ::core::ops::Mul<#self_n> for #p #w_mul {
                    type Output = #self_n;
                    fn mul(self, rhs: #self_n) -> #self_n { #mm }
                }
                #h_p_prim ::core::ops::Div<#self_n> for #p #w_div {
                    type Output = #out_n;
                    fn div(self, rhs: #self_n) -> #out_n { #dd }
                }
            });
        });
        out
    };

    let bin_op = |h: &TokenStream2,
                  rhs: &TokenStream2,
                  trait_name: &str,
                  method: &str,
                  out_ty: &TokenStream2,
                  bounds: &[TokenStream2],
                  body: TokenStream2| {
        let tr = syn::Ident::new(trait_name, Span::call_site());
        let mth = syn::Ident::new(method, Span::call_site());
        let mut preds = vec![quote! { #out_ty: ::core::marker::Sized }];
        preds.extend(bounds.iter().cloned());
        let w = wher(&preds);
        quote! {
            #h ::core::ops::#tr<#rhs> for #self_a #w {
                type Output = #out_ty;
                fn #mth(self, rhs: #rhs) -> Self::Output { #body }
            }
        }
    };
    let rhs_ref = quote! { &'__a #self_b };

    // `+ - * /` between any two members, owned and borrowed right operand.
    let ops = {
        let mut out = TokenStream2::new();
        for (tr, mth, out_ty, chk, b, sym) in [
            ("Add", "add", &add_ty, "AddResult", &b_add, quote! { + }),
            ("Sub", "sub", &sub_ty, "SubResult", &b_sub, quote! { - }),
            ("Mul", "mul", &mul_ty, "MulResult", &b_mul, quote! { * }),
            ("Div", "div", &div_ty, "DivResult", &b_div, quote! { / }),
        ] {
            let c = check(chk, &self_a, &self_b);
            let body = ctor(quote! { self.#member #sym rhs.#member });
            out.extend(bin_op(
                &h_ab,
                &self_b,
                tr,
                mth,
                out_ty,
                &[c.clone(), b.clone()],
                body.clone(),
            ));
            out.extend(bin_op(
                &h_ab_ref,
                &rhs_ref,
                tr,
                mth,
                out_ty,
                &[c, b.clone(), b_copy.clone()],
                body,
            ));
        }
        out
    };

    // Scaling by a number, `S * n` and `S / n`, and negation.
    let scalar_ops = {
        let w_mul = wher(::core::slice::from_ref(&b_mul));
        let w_div = wher(::core::slice::from_ref(&b_div));
        let w_neg = wher(::core::slice::from_ref(&b_neg));
        let mm = ctor(quote! { self.#member * rhs });
        let dd = ctor(quote! { self.#member / rhs });
        let nn = ctor(quote! { -self.#member });
        quote! {
            #h_p ::core::ops::Mul<#value_ty> for #self_p #w_mul {
                type Output = Self;
                fn mul(self, rhs: #value_ty) -> Self { #mm }
            }
            #h_p ::core::ops::Div<#value_ty> for #self_p #w_div {
                type Output = Self;
                fn div(self, rhs: #value_ty) -> Self { #dd }
            }
            #h_p ::core::ops::Neg for #self_p #w_neg {
                type Output = Self;
                fn neg(self) -> Self { #nn }
            }
        }
    };

    // `+= -= *= /=` by a member whose result is the left operand's type
    // again, owned and borrowed; `*=` and `/=` also by a number.
    let mut assign = TokenStream2::new();
    for (tr, mth, op, chk, b) in [
        (
            "AddAssign",
            "add_assign",
            quote! { += },
            "AddAssignResult",
            &b_add_assign,
        ),
        (
            "SubAssign",
            "sub_assign",
            quote! { -= },
            "SubAssignResult",
            &b_sub_assign,
        ),
        (
            "MulAssign",
            "mul_assign",
            quote! { *= },
            "MulAssignResult",
            &b_mul_assign,
        ),
        (
            "DivAssign",
            "div_assign",
            quote! { /= },
            "DivAssignResult",
            &b_div_assign,
        ),
    ] {
        let t = syn::Ident::new(tr, Span::call_site());
        let mth = syn::Ident::new(mth, Span::call_site());
        let c = check(chk, &self_a, &self_b);
        let w_q = wher(&[c.clone(), b.clone()]);
        let w_q_ref = wher(&[c, b.clone(), b_copy.clone()]);
        assign.extend(quote! {
            #h_ab ::core::ops::#t<#self_b> for #self_a #w_q {
                fn #mth(&mut self, rhs: #self_b) { self.#member #op rhs.#member; }
            }
            #h_ab_ref ::core::ops::#t<#rhs_ref> for #self_a #w_q_ref {
                fn #mth(&mut self, rhs: #rhs_ref) { self.#member #op rhs.#member; }
            }
        });
        if tr == "MulAssign" || tr == "DivAssign" {
            let w_n = wher(::core::slice::from_ref(b));
            assign.extend(quote! {
                #h_p ::core::ops::#t<#value_ty> for #self_p #w_n {
                    fn #mth(&mut self, rhs: #value_ty) { self.#member #op rhs; }
                }
            });
        }
    }

    // `Sum`, seeded with the number's `Default`, the numbers summed with
    // their `+=`; it exists where `+=` of the member and itself does.
    let sum = {
        let h_p_rt = head_rt(&[], &p_p);
        let h_p_ref = head_rt(&lt_a, &p_p);
        let c = check("AddAssignResult", &self_p, &self_p);
        let w = wher_rt(&[c.clone(), b_default.clone(), b_add_assign_rt.clone()]);
        let w_ref = wher_rt(&[
            c,
            b_default.clone(),
            b_add_assign_rt.clone(),
            b_copy.clone(),
        ]);
        let seed = ctor(quote! { <#value_ty as ::core::default::Default>::default() });
        quote! {
            #h_p_rt ::core::iter::Sum<#self_p> for #self_p #w {
                fn sum<__I>(iter: __I) -> Self
                where
                    __I: ::core::iter::Iterator<Item = #self_p>,
                {
                    iter.fold(#seed, |mut __acc, __q| { __acc.#member += __q.#member; __acc })
                }
            }
            #h_p_ref ::core::iter::Sum<&'__a #self_p> for #self_p #w_ref {
                fn sum<__I>(iter: __I) -> Self
                where
                    __I: ::core::iter::Iterator<Item = &'__a #self_p>,
                {
                    iter.fold(#seed, |mut __acc, __q| { __acc.#member += (*__q).#member; __acc })
                }
            }
        }
    };

    // A lossless change of number type, one impl per pair of primitives std
    // converts losslessly (a generic one would overlap core's `impl<T>
    // From<T> for T`).
    let mut from = TokenStream2::new();
    if v_ident.is_some() {
        for (f, tos) in LOSSLESS {
            let f = syn::Ident::new(f, Span::call_site());
            for to in *tos {
                let t = syn::Ident::new(to, Span::call_site());
                let src = ty_v(&tp, Some(&quote! { #f }));
                let dst = ty_v(&tp, Some(&quote! { #t }));
                let build = ctor(quote! { <#t as ::core::convert::From<#f>>::from(q.#member) });
                from.extend(quote! {
                    #h_p_prim ::core::convert::From<#src> for #dst {
                        fn from(q: #src) -> Self { #build }
                    }
                });
            }
        }
    }

    // `#[const_ops(from(M => N, ..))]`: a member counted in `M` meets one
    // counted in `N`, and the result is counted in `N`, the `M` side
    // converted with std `From<M> for N`.
    let mut mixed = TokenStream2::new();
    if !opts.from.is_empty() && v_ident.is_none() {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "`from(M => N)` mixes number types, so the struct needs one as a parameter: \
             `struct S<const P: T, N>(N)`",
        ));
    }
    let h_ab_prim = head_prim(&p_ab);
    // `k * S` for a listed number type that is not a primitive.
    let mut listed: Vec<String> = Vec::new();
    for (mm, nn) in &opts.from {
        for t in [mm, nn] {
            let key = quote! { #t }.to_string();
            if PRIMITIVES.contains(&key.as_str()) || listed.contains(&key) {
                continue;
            }
            listed.push(key);
            let p_t = ty_v(&tp, Some(&quote! { #t }));
            let build = ctor(quote! { self * rhs.#member });
            mixed.extend(quote! {
                #h_p_prim ::core::ops::Mul<#p_t> for #t where #t: [const] ::core::ops::Mul<Output = #t> {
                    type Output = #p_t;
                    fn mul(self, rhs: #p_t) -> #p_t { #build }
                }
            });
        }
    }
    for (mm, nn) in &opts.from {
        let (mm, nn) = (quote! { #mm }, quote! { #nn });
        let at = |params: &[TokenStream2], num: &TokenStream2| ty_v(params, Some(num));
        let (a_m, a_n, b_m, b_n) = (at(&ta, &mm), at(&ta, &nn), at(&tb, &mm), at(&tb, &nn));
        let (p_m, p_n) = (at(&tp, &mm), at(&tp, &nn));
        let result_n = |f: &str| at(&results(f, &ta, &tb), &nn);
        let conv = quote! { #nn: [const] ::core::convert::From<#mm> };
        let up = |x: TokenStream2| quote! { <#nn as ::core::convert::From<#mm>>::from(#x) };
        let primitive = |t: &TokenStream2| PRIMITIVES.iter().any(|p| t.to_string() == *p);
        let up_rhs = up(quote! { rhs.#member });
        let up_self = up(quote! { self.#member });
        let up_q = up(quote! { q.#member });
        let num = |tr: &str| {
            let t = syn::Ident::new(tr, Span::call_site());
            quote! { #nn: [const] ::core::ops::#t<Output = #nn> }
        };
        for (tr, f, chk, sym) in [
            ("Add", "__sum", "AddResult", quote! { + }),
            ("Sub", "__difference", "SubResult", quote! { - }),
            ("Mul", "__product", "MulResult", quote! { * }),
            ("Div", "__quotient", "DivResult", quote! { / }),
        ] {
            let out = result_n(f);
            let t = syn::Ident::new(tr, Span::call_site());
            let mth = syn::Ident::new(&tr.to_lowercase(), Span::call_site());
            let c = check(chk, &a_n, &b_n);
            for (left, right, left_up) in [(&a_m, &b_n, true), (&a_n, &b_m, false)] {
                let (l, r) = if left_up {
                    (up(quote! { self.#member }), quote! { rhs.#member })
                } else {
                    (quote! { self.#member }, up(quote! { rhs.#member }))
                };
                let build = ctor(quote! { #l #sym #r });
                let preds = [
                    quote! { #out: ::core::marker::Sized },
                    conv.clone(),
                    num(tr),
                    c.clone(),
                ];
                mixed.extend(quote! {
                    #h_ab_prim ::core::ops::#t<#right> for #left where #(#preds,)* {
                        type Output = #out;
                        fn #mth(self, rhs: #right) -> #out { #build }
                    }
                });
            }
        }
        // In place, into the `N` side.
        for (tr, mth, op, chk) in [
            ("AddAssign", "add_assign", quote! { += }, "AddAssignResult"),
            ("SubAssign", "sub_assign", quote! { -= }, "SubAssignResult"),
        ] {
            let t = syn::Ident::new(tr, Span::call_site());
            let mth = syn::Ident::new(mth, Span::call_site());
            let preds = [
                conv.clone(),
                quote! { #nn: [const] ::core::ops::#t },
                check(chk, &a_n, &b_n),
            ];
            mixed.extend(quote! {
                #h_ab_prim ::core::ops::#t<#b_m> for #a_n where #(#preds,)* {
                    fn #mth(&mut self, rhs: #b_m) { self.#member #op #up_rhs; }
                }
            });
        }
        // Scaling an `M` member by an `N` number, from either side.
        let w_scale = |extra: TokenStream2| quote! { where #conv, #extra };
        let (w_mul, w_div) = (w_scale(num("Mul")), w_scale(num("Div")));
        let (sm, sd) = (
            ctor(quote! { #up_self * rhs }),
            ctor(quote! { #up_self / rhs }),
        );
        let sl = ctor(quote! { self * #up_rhs });
        mixed.extend(quote! {
            #h_p_prim ::core::ops::Mul<#nn> for #p_m #w_mul {
                type Output = #p_n;
                fn mul(self, rhs: #nn) -> #p_n { #sm }
            }
            #h_p_prim ::core::ops::Div<#nn> for #p_m #w_div {
                type Output = #p_n;
                fn div(self, rhs: #nn) -> #p_n { #sd }
            }
            #h_p_prim ::core::ops::Mul<#p_m> for #nn #w_mul {
                type Output = #p_n;
                fn mul(self, rhs: #p_m) -> #p_n { #sl }
            }
        });
        // A number divided by an `M` member: a member of the plain
        // number's parameters divided by it, counted in `N`.
        {
            let quotient = results("__quotient", &ones, &tp);
            let one_args: Vec<TokenStream2> = ones.iter().map(|o| quote! { { #o } }).collect();
            let (one_n, out_n) = (at(&one_args, &nn), at(&quotient, &nn));
            let c = check("DivResult", &one_n, &p_n);
            let dl = ctor(quote! { self / #up_rhs });
            mixed.extend(quote! {
                #h_p_prim ::core::ops::Div<#p_m> for #nn
                where #conv, #nn: [const] ::core::ops::Div<Output = #nn>, #out_n: ::core::marker::Sized, #c
                {
                    type Output = #out_n;
                    fn div(self, rhs: #p_m) -> #out_n { #dl }
                }
            });
        }
        // `N`'s member from an `M` one, where the lossless `From`s above
        // don't already convert the two.
        if !(primitive(&mm) && primitive(&nn)) {
            let build = ctor(quote! { #up_q });
            mixed.extend(quote! {
                #h_p_prim ::core::convert::From<#p_m> for #p_n where #conv {
                    fn from(q: #p_m) -> #p_n { #build }
                }
            });
        }
        // Compared at `N`, either way round.
        let h_p_rt = {
            let ps: Vec<&TokenStream2> = others_prim.iter().chain(&p_p).collect();
            quote! { impl<#(#ps),*> }
        };
        let w_cmp = quote! { where #nn: ::core::cmp::PartialOrd + ::core::convert::From<#mm>, #mm: ::core::marker::Copy };
        let (us, them) = (up(quote! { other.#member }), up(quote! { self.#member }));
        mixed.extend(quote! {
            #h_p_rt ::core::cmp::PartialEq<#p_m> for #p_n #w_cmp {
                fn eq(&self, other: &#p_m) -> bool { self.#member == #us }
            }
            #h_p_rt ::core::cmp::PartialEq<#p_n> for #p_m #w_cmp {
                fn eq(&self, other: &#p_n) -> bool { #them == other.#member }
            }
            #h_p_rt ::core::cmp::PartialOrd<#p_m> for #p_n #w_cmp {
                fn partial_cmp(&self, other: &#p_m) -> ::core::option::Option<::core::cmp::Ordering> {
                    self.#member.partial_cmp(&#us)
                }
            }
            #h_p_rt ::core::cmp::PartialOrd<#p_n> for #p_m #w_cmp {
                fn partial_cmp(&self, other: &#p_n) -> ::core::option::Option<::core::cmp::Ordering> {
                    #them.partial_cmp(&other.#member)
                }
            }
        });
    }

    Ok(quote! {
        #module
        #checks
        #ops
        #scalar_ops
        #left_scalar
        #assign
        #sum
        #from
        #mixed
    })
}

/// The check traits: name, check fn, operator, message, label.
const CHECKS: &[(&str, &str, &str, &str, &str)] = &[
    (
        "AddResult",
        "__can_add",
        "+",
        "cannot add: `+` of these parameters has no result",
        "`+` of these parameters has no result",
    ),
    (
        "SubResult",
        "__can_sub",
        "-",
        "cannot subtract: `-` of these parameters has no result",
        "`-` of these parameters has no result",
    ),
    (
        "MulResult",
        "__can_mul",
        "*",
        "cannot multiply: `*` of these parameters has no result",
        "`*` of these parameters has no result",
    ),
    (
        "DivResult",
        "__can_div",
        "/",
        "cannot divide: `/` of these parameters has no result",
        "`/` of these parameters has no result",
    ),
    (
        "AddAssignResult",
        "__can_add_assign",
        "+=",
        "cannot add in place: `+` of these parameters is not the left one",
        "`+` of these parameters is not the left one",
    ),
    (
        "SubAssignResult",
        "__can_sub_assign",
        "-=",
        "cannot subtract in place: `-` of these parameters is not the left one",
        "`-` of these parameters is not the left one",
    ),
    (
        "MulAssignResult",
        "__can_mul_assign",
        "*=",
        "cannot multiply in place: `*` of these parameters is not the left one",
        "`*` of these parameters is not the left one",
    ),
    (
        "DivAssignResult",
        "__can_div_assign",
        "/=",
        "cannot divide in place: `/` of these parameters is not the left one",
        "`/` of these parameters is not the left one",
    ),
];

/// The primitive number types: the impls a generic number type cannot have
/// (`n * S`, `n / S`) are written once for each.
const PRIMITIVES: &[&str] = &[
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32",
    "f64",
];

/// The primitive pairs std converts losslessly (`impl From<V> for W`).
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

/// `#[const_ops(from(M => N, ..))]`.
struct Options {
    /// Number types `M` that meet `N`, the pair converting with std
    /// `From<M> for N`.
    from: Vec<(Type, Type)>,
}

impl Options {
    fn parse(input: &DeriveInput) -> syn::Result<Self> {
        let mut from: Vec<(Type, Type)> = Vec::new();
        for attr in &input.attrs {
            if !attr.path().is_ident("const_ops") {
                continue;
            }
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("from") {
                    let content;
                    syn::parenthesized!(content in meta.input);
                    let source: Type = content.parse()?;
                    content.parse::<Token![=>]>()?;
                    let targets = content.parse_terminated(Type::parse, Token![,])?;
                    let key = |t: &Type| quote! { #t }.to_string();
                    for target in targets {
                        if key(&source) == key(&target) {
                            return Err(syn::Error::new_spanned(
                                &target,
                                "a number type meets itself already: list other types",
                            ));
                        }
                        if from
                            .iter()
                            .any(|(a, b)| key(a) == key(&target) && key(b) == key(&source))
                        {
                            return Err(syn::Error::new_spanned(
                                &target,
                                "`from` runs one way: a pair listed both ways would give two \
                                 results for one sum",
                            ));
                        }
                        if from
                            .iter()
                            .any(|(a, b)| key(a) == key(&source) && key(b) == key(&target))
                        {
                            return Err(syn::Error::new_spanned(&target, "listed twice"));
                        }
                        from.push((source.clone(), target));
                    }
                    Ok(())
                } else {
                    Err(meta.error(
                        "expected `from(M => N, ..)` (number types that meet, converting with \
                         `From<M> for N`); a lifted parameter says its `one` itself: \
                         `#[ops(one = EXPR)] const P: T`",
                    ))
                }
            })?;
        }
        Ok(Options { from })
    }
}

/// The single field holding the value - positional or named, marked
/// `#[value]` or the only field that is not `PhantomData`.
struct ValueField {
    member: syn::Member,
    ty: Type,
    /// The `PhantomData` fields, filled with `PhantomData` wherever the
    /// struct is built.
    phantoms: Vec<syn::Member>,
}

impl ValueField {
    /// The struct from one value expression, as a struct expression that
    /// works for named and positional fields alike (`S { 0: v }`).
    fn construct(&self, name: &syn::Ident, expr: TokenStream2) -> TokenStream2 {
        let member = &self.member;
        let phantoms = &self.phantoms;
        quote! { #name { #member: #expr, #(#phantoms: ::core::marker::PhantomData,)* } }
    }
}

fn value_field(data: &Data, span: &impl quote::ToTokens) -> syn::Result<ValueField> {
    let Data::Struct(s) = data else {
        return Err(syn::Error::new_spanned(span, "`ConstOps` is for structs"));
    };
    let shape = "`ConstOps` needs one value field (and any `PhantomData`): \
                 `struct S<const P: T, N>(N)` or `struct S<const P: T> { #[value] v: f64 }`";
    let all: Vec<(syn::Member, &syn::Field)> = s
        .fields
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let m = match &f.ident {
                Some(id) => syn::Member::Named(id.clone()),
                None => syn::Member::Unnamed(syn::Index::from(i)),
            };
            (m, f)
        })
        .collect();
    let marked: Vec<&(syn::Member, &syn::Field)> = all
        .iter()
        .filter(|(_, f)| f.attrs.iter().any(|a| a.path().is_ident("value")))
        .collect();
    let data_fields: Vec<&(syn::Member, &syn::Field)> =
        all.iter().filter(|(_, f)| !is_phantom(&f.ty)).collect();
    let chosen = match (marked.as_slice(), data_fields.as_slice()) {
        ([one], _) => *one,
        ([], [one]) => *one,
        _ => return Err(syn::Error::new_spanned(span, shape)),
    };
    let mut phantoms = Vec::new();
    for (mb, f) in &all {
        if *mb == chosen.0 {
            continue;
        }
        if !is_phantom(&f.ty) {
            return Err(syn::Error::new_spanned(f, shape));
        }
        phantoms.push(mb.clone());
    }
    Ok(ValueField {
        member: chosen.0.clone(),
        ty: chosen.1.ty.clone(),
        phantoms,
    })
}

/// `PhantomData<..>`, by its last path segment.
fn is_phantom(ty: &Type) -> bool {
    matches!(ty, Type::Path(tp)
        if tp.qself.is_none()
            && tp.path.segments.last().is_some_and(|s| s.ident == "PhantomData"))
}

/// A lifted const parameter, `#[ops(one = EXPR)] const P: T`.
struct Lifted {
    ident: syn::Ident,
    ty: Type,
    /// Its value in a plain number: the result where an operator of `T` has
    /// none.
    one: Expr,
}

/// The lifted const parameters: those marked `#[ops(one = EXPR)]`, in order.
fn lifted_params(
    generics: &syn::Generics,
    span: &impl quote::ToTokens,
) -> syn::Result<Vec<Lifted>> {
    let mut out = Vec::new();
    for p in &generics.params {
        let GenericParam::Const(c) = p else { continue };
        for attr in c.attrs.iter().filter(|a| a.path().is_ident("ops")) {
            let mut one: Option<Expr> = None;
            if let syn::Meta::List(_) = attr.meta {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("one") {
                        one = Some(meta.value()?.parse()?);
                        Ok(())
                    } else {
                        Err(meta.error("expected `one = EXPR`, the parameter of a plain number"))
                    }
                })?;
            }
            let Some(one) = one else {
                return Err(syn::Error::new_spanned(
                    attr,
                    "a lifted parameter needs its value in a plain number: \
                     `#[ops(one = EXPR)]`, the result where an operator of its type has none",
                ));
            };
            if out.iter().any(|l: &Lifted| l.ident == c.ident) {
                return Err(syn::Error::new_spanned(attr, "`#[ops]` given twice: a parameter is lifted once"));
            }
            out.push(Lifted {
                ident: c.ident.clone(),
                ty: c.ty.clone(),
                one,
            });
        }
    }
    if out.is_empty() {
        return Err(syn::Error::new_spanned(
            span,
            "`ConstOps` lifts the const parameters marked `#[ops(one = EXPR)]`: \
             `struct S<#[ops(one = T::ONE)] const P: T, N>(N)`",
        ));
    }
    Ok(out)
}

/// One generic parameter as an impl declares it: without this derive's
/// helper attributes and without a default.
fn param_decl(p: &GenericParam) -> TokenStream2 {
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
