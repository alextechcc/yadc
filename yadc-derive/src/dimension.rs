//! `#[derive(Dimension)]` on a struct: the dimension algebra over exponent
//! axes, plus an optional label (`#[kind]`), whose values may include the
//! affine kind (points) and the logarithmic kind (levels and gains) - which,
//! the label enum says (`yadc::Label`).
//! On an enum - a label - it is `crate::kind`.

use crate::paths::Paths;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type};

pub(crate) fn expand_dimension(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let __paths = Paths::parse(input)?;
    let yadc = &__paths.yadc;
    let name = &input.ident;

    let fields = match &input.data {
        Data::Enum(_) => return crate::kind::expand_label(input),
        Data::Struct(s) => match &s.fields {
            Fields::Named(f) => &f.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    name,
                    "#[derive(Dimension)] requires a struct with named fields",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                name,
                "#[derive(Dimension)] works on a struct (a dimension) or an enum (a label)",
            ));
        }
    };

    // Partition fields into "numeric" (i8/i16/.. exponents) and "kind" (marked
    // with #[kind] - at most one). The kind field's type is reached only through
    // its `Dimension` impl (a kind is a dimension with no exponents), its
    // operators, and `yadc::Label` - which of its values is a point and which
    // a logarithm.
    let mut numeric_fields: Vec<&syn::Field> = Vec::new();
    let mut kind_field: Option<&syn::Field> = None;
    for f in fields.iter() {
        if let Some(attr) = f.attrs.iter().find(|a| a.path().is_ident("kind")) {
            if kind_field.is_some() {
                return Err(syn::Error::new_spanned(f, "only one #[kind] field allowed"));
            }
            if !matches!(attr.meta, syn::Meta::Path(_)) {
                return Err(syn::Error::new_spanned(
                    attr,
                    "the kind field is marked `#[kind]` alone; which of its values is a point \
                     and which a logarithm is said on the label enum's variants: \
                     `#[kind(affine)] Affine`, `#[kind(log)] Log`",
                ));
            }
            kind_field = Some(f);
        } else {
            numeric_fields.push(f);
        }
    }

    // Axis arithmetic is core's own `Add` / `Sub` / `PartialEq` and the zero
    // exponent is `Default::default()` - all const-callable on nightly - so no
    // literal `0` and no bespoke trait is needed. Any axis type with those
    // const impls and `PartialRoot` works: every integer, or `yadc::Exponent`
    // giving `m^½`.
    let axes: Vec<(&syn::Ident, &Type)> = numeric_fields
        .iter()
        .filter_map(|f| f.ident.as_ref().map(|i| (i, &f.ty)))
        .collect();

    // Mul: exponents add.  Div: exponents subtract. A kind survives either
    // only beside a plain number (below).
    // Add/Sub: exponents copied from the left operand, kinds add/subtract -
    // and `None` when the operands' exponents differ, so a mismatch (Mass +
    // Force) is rejected just like an illegal kind combination.
    // An exponent is combined with its type's own `+` / `-`, whose `Output`
    // only has to convert `Into<Option<_>>`: an integer's sum is itself
    // (`Some`), `yadc::Exponent`'s is already an `Option`. An exponent that
    // does not fit its type has no value to be, so the product panics with a
    // message saying so (at compile time, where the product is a type).
    let checked = |f: &syn::Ident, ty: &Type, add: bool| {
        let op = if add {
            quote! { + }
        } else {
            quote! { - }
        };
        let msg = format!(
            "the exponent `{f}` of this {} does not fit in its type",
            if add { "product" } else { "quotient" }
        );
        quote! {
            #f: match ::core::convert::Into::<::core::option::Option<#ty>>::into(a.#f #op b.#f) {
                ::core::option::Option::Some(e) => e,
                ::core::option::Option::None => ::core::panic!(#msg),
            }
        }
    };
    // What the exponent types must provide, stated once per type on the
    // impls that use it.
    let mut axis_types: Vec<&Type> = Vec::new();
    for (_, ty) in &axes {
        let key = quote! { #ty }.to_string();
        if !axis_types.iter().any(|t| quote! { #t }.to_string() == key) {
            axis_types.push(ty);
        }
    }
    let axis_bounds = |tr: &str| -> TokenStream2 {
        let t = syn::Ident::new(tr, Span::call_site());
        let preds = axis_types.iter().map(|ty| {
            quote! {
                #ty: [const] ::core::ops::#t,
                <#ty as ::core::ops::#t>::Output: [const] ::core::convert::Into<::core::option::Option<#ty>>,
            }
        });
        quote! { where #(#preds)* }
    };
    let mul_bounds = axis_bounds("Add");
    let div_bounds = axis_bounds("Sub");
    let mul_inits: Vec<_> = axes.iter().map(|(f, ty)| checked(f, ty, true)).collect();
    let div_inits: Vec<_> = axes.iter().map(|(f, ty)| checked(f, ty, false)).collect();
    let copy_inits: Vec<_> = axes.iter().map(|(f, _)| quote! { #f: a.#f }).collect();
    let zero_inits: Vec<_> = axes
        .iter()
        .map(|(f, ty)| quote! { #f: <#ty as ::core::default::Default>::default() })
        .collect();
    let root_inits: Vec<_> = axes
        .iter()
        .map(|(f, ty)| quote! { #f: <#ty as #yadc::PartialRoot>::partial_root(self.#f, n)? })
        .collect();
    let axes_same: Vec<_> = axes
        .iter()
        .map(|(f, _)| quote! { && a.#f == b.#f })
        .collect();
    let axes_zero = |x: TokenStream2| -> TokenStream2 {
        let each = axes
            .iter()
            .map(|(f, ty)| quote! { && #x.#f == <#ty as ::core::default::Default>::default() });
        quote! { (true #(#each)*) }
    };

    // The `#[kind]` field is optional: a label telling apart quantities of the
    // same exponents (an absolute temperature from a temperature difference).
    // It decides `+` / `-`, and makes `*` / `/` partial (a point or a level
    // has no products). With none, no kind code is emitted, products are
    // total and every combination of matching exponents is allowed.
    // The kind is reached only through `Dimension` and its operators, which
    // return `Option` - never a variant, never an inherent method. Every step
    // is `?`: a rejected kind makes the whole result `None`.
    let kind_init = |op: Option<(&str, &str)>| -> TokenStream2 {
        let Some(kf) = kind_field else {
            return quote! {};
        };
        let k = kf.ident.as_ref().unwrap();
        let kty = &kf.ty;
        match op {
            Some((tr, m)) => {
                let t = syn::Ident::new(tr, Span::call_site());
                let m = syn::Ident::new(m, Span::call_site());
                quote! { #k: <#kty as ::core::ops::#t>::#m(a.#k, b.#k)?, }
            }
            None => quote! { #k: <#kty as #yadc::Dimension>::DIMENSIONLESS, },
        }
    };
    // A kind is a label. Under `*` and `/` it survives only beside a plain
    // number - every exponent zero and the plain kind - and is dropped by
    // anything with units: 2 × T is still an absolute temperature, R × T is
    // just J/mol. The kind alone can't tell `2` from `2 m`; the dimension can,
    // so the rule lives here.
    let (kind_mul, kind_div) = match kind_field {
        Some(kf) => {
            let k = kf.ident.as_ref().unwrap();
            let kty = &kf.ty;
            let plain = quote! { <#kty as #yadc::Dimension>::DIMENSIONLESS };
            let one = quote! { <Self as #yadc::Dimension>::DIMENSIONLESS };
            (
                quote! {
                    #k: if a == #one { b.#k } else if b == #one { a.#k } else { #plain },
                },
                quote! {
                    #k: if b == #one { a.#k } else { #plain },
                },
            )
        }
        None => (quote! {}, quote! {}),
    };
    let kind_add = kind_init(Some(("Add", "add")));
    let kind_sub = kind_init(Some(("Sub", "sub")));
    let kind_identity = kind_init(None);
    let kind_root = match kind_field {
        Some(kf) => {
            let k = kf.ident.as_ref().unwrap();
            let kty = &kf.ty;
            quote! { #k: <#kty as #yadc::PartialRoot>::partial_root(self.#k, n)?, }
        }
        None => quote! {},
    };
    let some = quote! { ::core::option::Option::Some };
    let none = quote! { ::core::option::Option::None };
    let opt = quote! { ::core::option::Option<Self> };

    // `plain` forgets the label: written only for a dimension with one (the
    // trait's provided `plain` is the identity).
    let plain_fn = match kind_field {
        Some(kf) => {
            let k = kf.ident.as_ref().unwrap();
            let kty = &kf.ty;
            quote! {
                fn plain(self) -> Self {
                    let a = self;
                    #name { #(#copy_inits,)* #k: <#kty as #yadc::Dimension>::plain(a.#k) }
                }
            }
        }
        None => quote! {},
    };

    // The affine and the logarithmic kind. A point (affine) has no products,
    // no sum with another point and no root; two points of one dimension
    // differ by a plain amount, and a plain amount moves one. A logarithm
    // adds where the things it is the logarithm of multiply, so `+` and `-`
    // add and subtract its exponents - a power level less a power level is
    // the logarithm of a plain ratio, a gain - and nothing but another
    // logarithm adds to it; a gain (no exponents) scales and leaves any
    // other product as a label does, and a level (with exponents) has no
    // products. Beside a plain number a product is a scaling (below).
    let special = kind_field.is_some();
    // Asks the kind of `x` (`a`, `b`, `self`) whether it is a point or a
    // logarithm, through `yadc::Label`.
    let ask = |method: &str, x: TokenStream2| -> TokenStream2 {
        match kind_field {
            Some(kf) => {
                let (k, kty) = (kf.ident.as_ref().unwrap(), &kf.ty);
                let m = syn::Ident::new(method, Span::call_site());
                quote! { <#kty as #yadc::Label>::#m(#x.#k) }
            }
            None => quote! { false },
        }
    };
    let is_aff = |x: TokenStream2| ask("is_affine", x);
    let is_log = |x: TokenStream2| ask("is_log", x);
    let (kname, kplain) = match kind_field {
        Some(kf) => {
            let (k, kty) = (kf.ident.as_ref().unwrap(), &kf.ty);
            (quote! { #k }, quote! { <#kty as #yadc::Dimension>::DIMENSIONLESS })
        }
        None => (quote! {}, quote! {}),
    };
    let (a_, b_, s_) = (quote! { a }, quote! { b }, quote! { self });
    let (aff_a, aff_b, log_a, log_b) = (
        is_aff(a_.clone()),
        is_aff(b_.clone()),
        is_log(a_.clone()),
        is_log(b_.clone()),
    );
    let (zero_a, zero_b) = (axes_zero(a_.clone()), axes_zero(b_.clone()));
    let one = quote! { <Self as #yadc::Dimension>::DIMENSIONLESS };
    let (prod_out, wrap) = if special {
        (quote! { #opt }, quote! { #some })
    } else {
        (quote! { Self }, quote! {})
    };
    let (mul_head, div_head, add_head, sub_head, root_head) = if special {
        let log_sum = |inits: &[TokenStream2]| {
            quote! {
                if #log_a || #log_b {
                    if !(#log_a && #log_b) {
                        return #none;
                    }
                    return #some(#name { #(#inits,)* #kname: a.#kname });
                }
            }
        };
        (
            quote! {
                // Beside a plain number a product is a scaling: it keeps the
                // other only where that scales (adds to itself as itself).
                if a == #one || b == #one {
                    let x = if a == #one { b } else { a };
                    return match <Self as ::core::ops::Add>::add(x, x) {
                        #some(sum) if sum == x => #some(x),
                        _ => #none,
                    };
                }
                if #aff_a || #aff_b {
                    return #none;
                }
                if #log_a || #log_b {
                    if (#log_a && #log_b) || (#log_a && !#zero_a) || (#log_b && !#zero_b) {
                        return #none;
                    }
                }
            },
            quote! {
                if b == #one {
                    return match <Self as ::core::ops::Add>::add(a, a) {
                        #some(sum) if sum == a => #some(a),
                        _ => #none,
                    };
                }
                if #aff_a || #aff_b {
                    return #none;
                }
                if #log_a || #log_b {
                    if (#log_a && #log_b) || (#log_a && !#zero_a) || (#log_b && !#zero_b) {
                        return #none;
                    }
                }
            },
            {
                let logs = log_sum(&mul_inits);
                quote! {
                    #logs
                    if #aff_b {
                        return #none;
                    }
                    if #aff_a {
                        return if (true #(#axes_same)*) && b.#kname == #kplain {
                            #some(a)
                        } else {
                            #none
                        };
                    }
                }
            },
            {
                let logs = log_sum(&div_inits);
                quote! {
                    #logs
                    if #aff_b {
                        return if #aff_a && (true #(#axes_same)*) {
                            #some(#name { #(#copy_inits,)* #kname: #kplain })
                        } else {
                            #none
                        };
                    }
                    if #aff_a {
                        return if (true #(#axes_same)*) && b.#kname == #kplain {
                            #some(a)
                        } else {
                            #none
                        };
                    }
                }
            },
            {
                let (aff_s, log_s) = (is_aff(s_.clone()), is_log(s_.clone()));
                quote! {
                    if #aff_s || #log_s {
                        return #none;
                    }
                }
            },
        )
    } else {
        Default::default()
    };
    // A logarithm's sum adds exponents, so `+` and `-` then ask the exponent
    // types for what `*` and `/` ask.
    let (add_bounds, sub_bounds) = if special {
        (mul_bounds.clone(), div_bounds.clone())
    } else {
        (quote! {}, quote! {})
    };

    let mul_body = quote! { #name { #(#mul_inits,)* #kind_mul } };
    let div_body = quote! { #name { #(#div_inits,)* #kind_div } };
    let (wrap_mul, wrap_div) = if special {
        (quote! { #wrap(#mul_body) }, quote! { #wrap(#div_body) })
    } else {
        (mul_body, div_body)
    };

    // `Display`: the axes with superscript exponents, then a label that is
    // not plain, in braces, by its `Display`.
    let display = if crate::util::dimension_impls(&input.attrs)?
        .is_none_or(|l| l.iter().any(|x| x == "Display"))
    {
        let mut axis_items = Vec::new();
        for f in &numeric_fields {
            let id = f.ident.as_ref().unwrap();
            let ty = &f.ty;
            let sym = crate::util::axis_symbol(f)?.unwrap_or_else(|| id.to_string());
            axis_items.push(quote! {
                (#sym, &self.#id as &dyn ::core::fmt::Display, self.#id != <#ty as ::core::default::Default>::default())
            });
        }
        let mut tags = Vec::new();
        if let Some(kf) = kind_field {
            let k = kf.ident.as_ref().unwrap();
            let kty = &kf.ty;
            tags.push(quote! {
                (&self.#k as &dyn ::core::fmt::Display, self.#k != <#kty as #yadc::Dimension>::DIMENSIONLESS)
            });
        }
        quote! {
            impl ::core::fmt::Display for #name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    #yadc::display::write_dimension(f, &[#(#axis_items),*], &[#(#tags),*])
                }
            }
        }
    } else {
        quote! {}
    };

    let powi = crate::util::partial_powi_impl(yadc, name);
    let numbers: Vec<syn::Ident> = crate::quantity::PRIMITIVES
        .iter()
        .map(|p| syn::Ident::new(p, proc_macro2::Span::call_site()))
        .collect();
    Ok(quote! {
        #display
        #powi
        // Exponents add under `*`, subtract under `/`, and must be equal under
        // `+` / `-`, where the label decides the rest and a rejected
        // combination is `None`. `PartialEq` (as a const impl), `Eq`, `Debug`,
        // `Copy`, `Clone` and `ConstParamTy` are the user's own derives.
        // Dimension one: every exponent zero, the label's own dimension one.
        const impl #yadc::Dimension for #name {
            const DIMENSIONLESS: Self = #name { #(#zero_inits,)* #kind_identity };
            #plain_fn
        }
        #[allow(clippy::suspicious_arithmetic_impl)] // exponents add under `*`
        const impl ::core::ops::Mul for #name #mul_bounds {
            type Output = #prod_out;
            fn mul(self, rhs: Self) -> #prod_out {
                let (a, b) = (self, rhs);
                #mul_head
                #wrap_mul
            }
        }
        #[allow(clippy::suspicious_arithmetic_impl)] // exponents subtract under `/`
        const impl ::core::ops::Div for #name #div_bounds {
            type Output = #prod_out;
            fn div(self, rhs: Self) -> #prod_out {
                let (a, b) = (self, rhs);
                #div_head
                #wrap_div
            }
        }
        const impl ::core::ops::Add for #name #add_bounds {
            type Output = #opt;
            fn add(self, rhs: Self) -> #opt {
                let (a, b) = (self, rhs);
                #add_head
                if !(true #(#axes_same)*) {
                    return #none;
                }
                #some(#name { #(#copy_inits,)* #kind_add })
            }
        }
        const impl ::core::ops::Sub for #name #sub_bounds {
            type Output = #opt;
            fn sub(self, rhs: Self) -> #opt {
                let (a, b) = (self, rhs);
                #sub_head
                if !(true #(#axes_same)*) {
                    return #none;
                }
                #some(#name { #(#copy_inits,)* #kind_sub })
            }
        }
        // Scaled by a number it is the same dimension, where it can be scaled
        // at all: scaling is repeated addition, so where it adds to itself as
        // itself - not where it has no sum with itself, nor where that sum is
        // another dimension. Once per primitive number type.
        #(
            const impl ::core::ops::Mul<#numbers> for #name #add_bounds {
                type Output = #opt;
                fn mul(self, _: #numbers) -> #opt {
                    match <Self as ::core::ops::Add>::add(self, self) {
                        #some(sum) if sum == self => #some(self),
                        _ => #none,
                    }
                }
            }
            const impl ::core::ops::Div<#numbers> for #name #add_bounds {
                type Output = #opt;
                fn div(self, _: #numbers) -> #opt {
                    match <Self as ::core::ops::Add>::add(self, self) {
                        #some(sum) if sum == self => #some(self),
                        _ => #none,
                    }
                }
            }
        )*
        // The n-th root roots every exponent - `PartialRoot` on the axis type:
        // an integer needs an even split, `yadc::Exponent` always has one -
        // and roots the kind.
        const impl #yadc::PartialRoot for #name {
            fn partial_root(self, n: i8) -> #opt {
                #root_head
                #some(#name { #(#root_inits,)* #kind_root })
            }
        }
    })
}
