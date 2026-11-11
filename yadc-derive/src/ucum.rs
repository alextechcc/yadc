//! `#[derive(UcumSerialize)]` / `#[derive(UcumDeserialize)]`: dimensions as UCUM strings.

use crate::paths::Paths;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type};

// UcumSerialize / UcumDeserialize - emit a UCUM-compliant string for
// a Dimension struct: the axes ("m.kg.s-2"), all-zero plain kind -> "1", and
// any other kind as a prefix (if its `UcumKind` declares one, "pfx[.axes]") or
// else a "{KindName}" annotation ("K{Affine}", "1{Log}"), read back on
// deserialize.

/// One exponent axis of a dimension struct (every field but the `#[kind]`
/// one): its field, the UCUM atom symbol it serializes as, and its type.
struct AxisField {
    ident: syn::Ident,
    symbol: String,
    ty: Type,
}

type DimFields = (Vec<AxisField>, Option<(syn::Ident, Type)>);

// Returns (numeric_fields, kind_field) - same shape as expand_dimension.
// The UCUM symbol of an axis defaults to its field name and is overridden by
// `#[ucum("...")]`, so a field may be called `kilogram` and serialize as `kg`.
fn dim_fields(input: &DeriveInput) -> syn::Result<DimFields> {
    let fields = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(f) => &f.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    &input.ident,
                    "UcumSerialize/Deserialize requires a struct with named fields",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                &input.ident,
                "UcumSerialize/Deserialize only works on structs",
            ));
        }
    };
    let mut numeric = Vec::new();
    let mut kind: Option<(syn::Ident, Type)> = None;
    for f in fields {
        let is_kind = f.attrs.iter().any(|a| a.path().is_ident("kind"));
        let Some(id) = f.ident.clone() else {
            continue;
        };
        if is_kind {
            kind = Some((id, f.ty.clone()));
        } else {
            let symbol = match ucum_symbol(f)? {
                Some(s) => s,
                None => id.to_string(),
            };
            numeric.push(AxisField {
                ident: id,
                symbol,
                ty: f.ty.clone(),
            });
        }
    }
    Ok((numeric, kind))
}

/// The `#[ucum("sym")]` override on a dimension axis field, if present.
fn ucum_symbol(f: &syn::Field) -> syn::Result<Option<String>> {
    for attr in &f.attrs {
        if attr.path().is_ident("ucum") {
            let lit: syn::LitStr = attr.parse_args()?;
            return Ok(Some(lit.value()));
        }
    }
    Ok(None)
}

pub(crate) fn expand_ucum_serialize(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let __paths = Paths::parse(input)?;
    let yadc = &__paths.yadc;
    let serde = &__paths.serde;
    let name = &input.ident;
    let (axes, kind) = dim_fields(input)?;

    // UCUM kind prefix: whatever `#[ucum(prefix = "..")]` declared, else nothing.
    let kind_prefix = match &kind {
        Some((k, kty)) => quote! {
            if let ::core::option::Option::Some(__prefix) =
                <#kty as #yadc::ucum::UcumKind>::ucum_prefix(self.#k)
            {
                __buf.push_str(__prefix);
                __first = false;
            }
        },
        None => quote! {},
    };

    // Axis entries: dot-separated, exponent directly after symbol (UCUM style).
    // The exponent type is touched only through `Default` (the zero test, via
    // `PartialEq`) and `Display` (rendering) - never a literal `0` or `1` - so
    // an axis need not be an integer. UCUM writes a bare symbol for exponent
    // one, which here is exactly "the exponent renders as `1`".
    let axis_entries = axes.iter().map(
        |AxisField {
             ident,
             symbol: sym,
             ty,
         }| {
            quote! {
                {
                    let __v = self.#ident;
                    if __v != <#ty as ::core::default::Default>::default() {
                        if !__first { __buf.push('.'); }
                        __buf.push_str(#sym);
                        let __exp = __yadc_alloc::format!("{}", __v);
                        if __exp != "1" {
                            __buf.push_str(&__exp);
                        }
                        __first = false;
                    }
                }
            }
        },
    );

    // UCUM annotation suffix {KindName} for non-prefix, non-zero kinds (the kinds
    // a prefix or the bare axes can't already distinguish, e.g. the affine kind vs the plain one).
    let kind_annotation = match &kind {
        Some((k, kty)) => quote! {
            if <#kty as #yadc::ucum::UcumKind>::ucum_prefix(self.#k).is_none()
                && self.#k != <#kty as #yadc::Dimension>::DIMENSIONLESS
            {
                __buf.push('{');
                __buf.push_str(<#kty as #yadc::ucum::UcumKind>::name(self.#k));
                __buf.push('}');
            }
        },
        None => quote! {},
    };

    Ok(quote! {
        const _: () = {
            extern crate alloc as __yadc_alloc;
            impl #serde::Serialize for #name {
                fn serialize<__S: #serde::Serializer>(
                    &self,
                    __ser: __S,
                ) -> ::core::result::Result<__S::Ok, __S::Error> {
                    let mut __buf = __yadc_alloc::string::String::new();
                    let mut __first = true;
                    #kind_prefix
                    #(#axis_entries)*
                    // UCUM "1" for a dimensionless/kind-less quantity.
                    if __first { __buf.push('1'); }
                    #kind_annotation
                    __ser.serialize_str(&__buf)
                }
            }
        };
    })
}

pub(crate) fn expand_ucum_deserialize(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let __paths = Paths::parse(input)?;
    let yadc = &__paths.yadc;
    let serde = &__paths.serde;
    let name = &input.ident;
    let (axes, kind) = dim_fields(input)?;

    // Sort by symbol length descending so longer symbols are tried first
    // (avoids "m" matching the start of "mol" or "mg").
    let mut sorted_axes: Vec<&AxisField> = axes.iter().collect();
    sorted_axes.sort_by_key(|a| ::core::cmp::Reverse(a.symbol.len()));

    // Generate an if-else chain: for each token, `strip_prefix` the known
    // symbol and parse the remainder as an i8 exponent (empty -> 1).
    let parse_chain = sorted_axes.iter().map(
        |AxisField {
             ident,
             symbol: sym,
             ty,
         }| {
            quote! {
                else if let ::core::option::Option::Some(__exp_str) = __tok.strip_prefix(#sym) {
                    // The exponent is parsed at the field's own type through
                    // `FromStr` - nothing here assumes `i8`, or any integer at
                    // all. A bare symbol is exponent one, spelled `"1"`: the
                    // exact inverse of what `UcumSerialize` omits.
                    let __src = if __exp_str.is_empty() { "1" } else { __exp_str };
                    let __exp: #ty = <#ty as ::core::str::FromStr>::from_str(__src)
                        .map_err(|_| {
                            <__D::Error as #serde::de::Error>::custom(
                                __yadc_alloc::format!("bad exponent in `{}`", __tok),
                            )
                        })?;
                    __out.#ident = __exp;
                }
            }
        },
    );

    // Strip a trailing {KindName} annotation and read it back with
    // `UcumKind::from_name` to recover non-prefix kinds (e.g. "K{Affine}").
    // The remaining string is then processed for UCUM prefix / axes as normal.
    let annotation_extract = match &kind {
        Some((k, kty)) => quote! {
            let __s: &str = if __s.ends_with('}') {
                if let ::core::option::Option::Some(__open) = __s.rfind('{') {
                    let __ann = &__s[__open + 1..__s.len() - 1];
                    __out.#k = <#kty as #yadc::ucum::UcumKind>::from_name(__ann)
                        .ok_or_else(|| {
                            <__D::Error as #serde::de::Error>::custom(
                                __yadc_alloc::format!("unknown kind `{}`", __ann),
                            )
                        })?;
                    __s[..__open].trim_end()
                } else {
                    &__s
                }
            } else {
                &__s
            };
        },
        None => quote! { let __s: &str = &__s; },
    };

    // Kind prefix detection. "1" is the dimensionless unit. Otherwise the first
    // dot-separated token is offered to `Kind::from_ucum_prefix`; if it names a
    // prefix-bearing kind (one with a declared `#[ucum(prefix = "..")]`) we set the
    // kind and continue with the remaining axes. This is the exact inverse of
    // the `ucum_prefix` used during serialization, so the mapping lives in one
    // place (the Kind derive) rather than being duplicated here.
    let kind_prefix_extract = match &kind {
        Some((k, kty)) => quote! {
            let __axes: &str = if __s == "1" {
                return ::core::result::Result::Ok(__out);
            } else {
                let (__head, __rest) = match __s.find('.') {
                    ::core::option::Option::Some(__i) => (&__s[..__i], &__s[__i + 1..]),
                    ::core::option::Option::None => (__s, ""),
                };
                if let ::core::option::Option::Some(__k) =
                    <#kty as #yadc::ucum::UcumKind>::from_ucum_prefix(__head)
                {
                    __out.#k = __k;
                    if __rest.is_empty() {
                        return ::core::result::Result::Ok(__out);
                    }
                    __rest
                } else {
                    __s
                }
            };
        },
        None => quote! {
            let __axes: &str =
                if __s == "1" { return ::core::result::Result::Ok(__out); }
                else { __s };
        },
    };

    Ok(quote! {
        const _: () = {
            extern crate alloc as __yadc_alloc;
            impl<'de> #serde::Deserialize<'de> for #name {
                fn deserialize<__D: #serde::Deserializer<'de>>(
                    __de: __D,
                ) -> ::core::result::Result<Self, __D::Error> {
                    let __s: __yadc_alloc::string::String =
                        <__yadc_alloc::string::String as #serde::Deserialize>::deserialize(__de)?;
                    let mut __out = <#name as #yadc::Dimension>::DIMENSIONLESS;
                    #annotation_extract
                    #kind_prefix_extract
                    for __tok in __axes.split('.') {
                        if false { unreachable!() }
                        #(#parse_chain)*
                        else {
                            return ::core::result::Result::Err(
                                <__D::Error as #serde::de::Error>::custom(
                                    __yadc_alloc::format!("unknown unit `{}`", __tok),
                                ),
                            );
                        }
                    }
                    ::core::result::Result::Ok(__out)
                }
            }
        };
    })
}
