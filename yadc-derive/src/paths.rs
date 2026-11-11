//! `#[yadc(crate = "..", serde = "..")]`: where generated code finds `yadc` and `serde`.

use syn::{DeriveInput, Token, parse::ParseStream};

/// Where the generated code should look for `yadc` and, for the UCUM derives,
/// `serde`.
///
/// Nothing emitted by this crate hardcodes a crate name: every path below is
/// spliced from here, so a crate that renames the dependency
/// (`use yadc as ydc;`, a vendored copy, a macro-generating macro) says so once
/// with `#[yadc(crate = "ydc")]` on the derived item.
pub(crate) struct Paths {
    pub(crate) yadc: syn::Path,
    pub(crate) serde: syn::Path,
}

impl Paths {
    /// Read `#[yadc(crate = "..", serde = "..")]` off the derived item.
    pub(crate) fn parse(input: &DeriveInput) -> syn::Result<Self> {
        let mut paths = Paths {
            yadc: syn::parse_quote!(::yadc),
            serde: syn::parse_quote!(::serde),
        };
        for attr in &input.attrs {
            if !attr.path().is_ident("yadc") {
                continue;
            }
            attr.parse_args_with(|stream: ParseStream<'_>| {
                while !stream.is_empty() {
                    let key = <syn::Ident as syn::ext::IdentExt>::parse_any(stream)?;
                    stream.parse::<Token![=]>()?;
                    match key.to_string().as_str() {
                        "crate" => paths.yadc = stream.parse::<syn::LitStr>()?.parse()?,
                        "serde" => paths.serde = stream.parse::<syn::LitStr>()?.parse()?,
                        other => {
                            return Err(syn::Error::new_spanned(
                                &key,
                                format!(
                                    "unknown `#[yadc(..)]` argument `{other}`; \
                                     expected `crate` or `serde`"
                                ),
                            ));
                        }
                    }
                    if stream.is_empty() {
                        break;
                    }
                    stream.parse::<Token![,]>()?;
                }
                Ok(())
            })?;
        }
        Ok(paths)
    }
}
