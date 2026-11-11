//! The generator: a `UnitSystem` to a crate's `src/*.rs` (and a starting
//! `Cargo.toml`).
//!
//! The output is laid out by `rustfmt` after it is written, so it is
//! fmt-clean and the same for the same input.

use crate::model::*;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::path::Path;

// --- Type-name helpers --------------------------------------------------------

fn dim_ty(sys: &SystemConfig) -> &str {
    sys.dim_type.as_deref().unwrap_or("Dim")
}
fn qty_ty(sys: &SystemConfig) -> &str {
    sys.qty_type.as_deref().unwrap_or("Qty")
}
fn dyn_ty(sys: &SystemConfig) -> &str {
    sys.dynqty_type.as_deref().unwrap_or("DynQty")
}

// --- Public entry point -------------------------------------------------------

pub fn write_crate(system: &UnitSystem, output_dir: &Path) -> std::io::Result<()> {
    let src = output_dir.join("src");
    std::fs::create_dir_all(&src)?;

    let mut written = vec![src.join("lib.rs"), src.join("dimensionless.rs")];
    std::fs::write(&written[0], gen_lib_rs(system))?;
    std::fs::write(&written[1], gen_dimensionless_rs(system))?;
    std::fs::write(output_dir.join("Cargo.toml"), gen_cargo_toml(system))?;

    for fam_name in &system.system.family_order {
        let Some(fam) = system.family.iter().find(|f| &f.name == fam_name) else {
            continue;
        };
        if fam.quantity.is_empty() {
            continue;
        }
        let path = src.join(format!("{fam_name}.rs"));
        std::fs::write(&path, gen_family_rs(fam, system))?;
        written.push(path);
    }
    format_files(&written);
    Ok(())
}

/// Lay the written files out with `rustfmt` (edition 2024), one at a time
/// through its standard input, so it formats each file alone (a `mod`
/// declaration is not followed). Without it, the files are still valid Rust,
/// just not formatted: say so.
fn format_files(files: &[std::path::PathBuf]) {
    use std::io::Write;
    use std::process::{Command, Stdio};
    for f in files {
        let Ok(src) = std::fs::read(f) else { continue };
        let child = Command::new("rustfmt")
            .args(["--edition", "2024", "--emit", "stdout"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn();
        let mut child = match child {
            Ok(c) => c,
            Err(e) => {
                eprintln!("warning: could not run rustfmt ({e}): the output is not formatted");
                return;
            }
        };
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(&src);
        }
        match child.wait_with_output() {
            Ok(out) if out.status.success() => {
                let _ = std::fs::write(f, out.stdout);
            }
            _ => eprintln!(
                "warning: rustfmt failed on {}: left unformatted",
                f.display()
            ),
        }
    }
}

// --- Base constants -----------------------------------------------------------

/// The base constants by name, and which one is the unit of each axis
/// (`m` -> `METRE`): the one with that axis at exponent 1, nothing else, and
/// the plain label.
struct Base<'a> {
    consts: &'a [BaseConst],
    axis: BTreeMap<&'a str, &'a str>,
    /// The plain dimension-one constant (`ONE`), for a product with no
    /// positive exponent.
    one: Option<&'a str>,
}

impl<'a> Base<'a> {
    fn new(system: &'a UnitSystem) -> Self {
        let mut axis = BTreeMap::new();
        let mut one = None;
        for b in &system.base {
            if b.kind.is_some() {
                continue;
            }
            let nonzero: Vec<(&String, &i64)> = b.dims.iter().filter(|(_, e)| **e != 0).collect();
            match nonzero.as_slice() {
                [] => one = Some(b.name.as_str()),
                [(f, 1)] => {
                    axis.entry(f.as_str()).or_insert(b.name.as_str());
                }
                _ => {}
            }
        }
        Base {
            consts: &system.base,
            axis,
            one,
        }
    }

    /// The base constant with exactly these exponents and label.
    fn exact(&self, dims: &BTreeMap<String, i64>, kind: Option<&str>) -> Option<&'a BaseConst> {
        self.consts
            .iter()
            .find(|b| same_dims(&b.dims, dims) && b.kind.as_deref() == kind)
    }

    /// `METRE * KILOGRAM / (SECOND * SECOND)`: the coherent unit of these
    /// exponents, the product of base constants (axes in `[dim]` order).
    /// A base constant whose name a constant of the module shadows (an arc
    /// `SECOND` of `Angle`) is named by its path, `crate::base::SECOND`.
    fn product(
        &self,
        dims: &BTreeMap<String, i64>,
        fields: &[String],
        q: &str,
        shadowed: &BTreeSet<&str>,
    ) -> String {
        let path = |n: &str| {
            if shadowed.contains(n) {
                format!("crate::base::{n}")
            } else {
                n.to_string()
            }
        };
        let mut num = Vec::new();
        let mut den = Vec::new();
        for f in fields {
            let e = dims.get(f).copied().unwrap_or(0);
            if e == 0 {
                continue;
            }
            let name = self.axis.get(f.as_str()).unwrap_or_else(|| {
                panic!(
                    "quantity `{q}` has the axis `{f}`, but no [[base]] constant is its unit \
                     (dims = {{{f} = 1}}, no kind)"
                )
            });
            for _ in 0..e.unsigned_abs() {
                if e > 0 {
                    num.push(path(name));
                } else {
                    den.push(path(name));
                }
            }
        }
        let one = || {
            self.one.unwrap_or_else(|| {
                panic!(
                    "quantity `{q}` needs a dimension-one [[base]] constant (dims = {{}}, no \
                     kind; `ONE`) for its coherent unit"
                )
            })
        };
        let num = if num.is_empty() {
            path(one())
        } else {
            num.join(" * ")
        };
        match den.len() {
            0 => num,
            1 => format!("{num} / {}", den[0]),
            _ => format!("{num} / ({})", den.join(" * ")),
        }
    }
}

fn same_dims(a: &BTreeMap<String, i64>, b: &BTreeMap<String, i64>) -> bool {
    let nz = |m: &BTreeMap<String, i64>| -> BTreeMap<String, i64> {
        m.iter()
            .filter(|(_, e)| **e != 0)
            .map(|(k, e)| (k.clone(), *e))
            .collect()
    };
    nz(a) == nz(b)
}

/// The base constants' module: the only raw numbers in the crate.
fn gen_base_mod(system: &UnitSystem) -> String {
    let dim = dim_ty(&system.system);
    let qty = qty_ty(&system.system);
    let uses_kind = system.base.iter().any(|b| b.kind.is_some());
    let mut s = String::new();
    s += "/// The base constants: one per base unit, and the zeros - the only raw\n";
    s += "/// numbers in this crate. Every other constant is defined from them with\n";
    s += "/// the quantities' own `const` operators, and re-exported by the module of\n";
    s += "/// its quantity (`mechanical::consts::METRE`).\n";
    s += "pub mod base {\n";
    s += &format!(
        "    use crate::{{{}}};\n",
        if uses_kind {
            format!("{dim}, Kind, {qty}")
        } else {
            format!("{dim}, {qty}")
        }
    );
    s += "    use yadc::Dimension;\n";
    for b in &system.base {
        let ty = format!(
            "{qty}<{}>",
            dim_value(&b.dims, b.kind.as_deref(), &system.dim.fields, dim)
        );
        s += &format!("\n    /// {}\n", b.doc);
        s += &format!("    pub const {}: {ty} = {qty}({});\n", b.name, b.value);
    }
    s += "}\n";
    s
}

/// `{ Dim { m: 1, ..Dim::DIMENSIONLESS } }`, or `{ Dim::DIMENSIONLESS }`.
fn dim_value(
    dims: &BTreeMap<String, i64>,
    kind: Option<&str>,
    fields: &[String],
    dim: &str,
) -> String {
    let parts = dim_field_exprs(dims, kind, fields);
    if parts.is_empty() {
        format!("{{ {dim}::DIMENSIONLESS }}")
    } else {
        format!(
            "{{ {dim} {{ {}, ..{dim}::DIMENSIONLESS }} }}",
            parts.join(", ")
        )
    }
}

// --- lib.rs -------------------------------------------------------------------

/// The impls of the scale type: a unit of its zero's type, called like any
/// unit. `@DIM@` and `@QTY@` are the system's dimension and quantity.
const SCALE_IMPLS: &str = r#"const impl<P, S, N> Unit<P, N> for Scale<P, S>
where
    P: Copy + [const] core::ops::Sub + [const] core::ops::Add<<P as core::ops::Sub>::Output, Output = P>,
    S: [const] Unit<<P as core::ops::Sub>::Output, N>,
{
    fn quantity(&self, value: N) -> P {
        self.0 + self.1.quantity(value)
    }
    fn value(&self, quantity: P) -> N {
        self.1.value(quantity - self.0)
    }
}

// A scale is called like any unit: on a number it builds, on a value it
// reads. Std's three function traits, once for each direction.
macro_rules! scale_is_called {
    ($arg:ty => $out:ty, $way:ident) => {
        const impl<const D: @DIM@, N, S> FnOnce<($arg,)> for Scale<@QTY@<D, N>, S>
        where
            Self: [const] Unit<@QTY@<D, N>, N> + [const] core::marker::Destruct,
            N: [const] core::marker::Destruct,
        {
            type Output = $out;
            extern "rust-call" fn call_once(self, (x,): ($arg,)) -> $out {
                Unit::$way(&self, x)
            }
        }
        const impl<const D: @DIM@, N, S> FnMut<($arg,)> for Scale<@QTY@<D, N>, S>
        where
            Self: [const] Unit<@QTY@<D, N>, N> + [const] core::marker::Destruct,
            N: [const] core::marker::Destruct,
        {
            extern "rust-call" fn call_mut(&mut self, (x,): ($arg,)) -> $out {
                Unit::$way(&*self, x)
            }
        }
        const impl<const D: @DIM@, N, S> Fn<($arg,)> for Scale<@QTY@<D, N>, S>
        where
            Self: [const] Unit<@QTY@<D, N>, N> + [const] core::marker::Destruct,
            N: [const] core::marker::Destruct,
        {
            extern "rust-call" fn call(&self, (x,): ($arg,)) -> $out {
                Unit::$way(self, x)
            }
        }
    };
}
scale_is_called!(N => @QTY@<D, N>, quantity);
scale_is_called!(@QTY@<D, N> => N, value);

"#;

pub fn gen_lib_rs(system: &UnitSystem) -> String {
    let mut s = String::new();
    let dim = dim_ty(&system.system);
    let qty = qty_ty(&system.system);
    let dyn_ = dyn_ty(&system.system);

    s += "#![cfg_attr(not(feature = \"std\"), no_std)]\n";
    s += "#![feature(generic_const_exprs)]\n";
    s += "#![feature(min_adt_const_params)]\n";
    s += "#![feature(const_trait_impl)]\n";
    s += "#![feature(const_cmp)]\n";
    s += "#![feature(const_default)]\n";
    s += "#![feature(derive_const)]\n";
    s += "#![feature(const_try)]\n";
    s += "#![feature(const_ops)]\n";
    s += "#![feature(const_convert)]\n";
    // The dynamic type's dynamickit impls drop their operands in `const` code.
    s += "#![feature(const_destruct)]\n";
    s += "#![feature(fn_traits)]\n";
    s += "#![feature(unboxed_closures)]\n";
    s += "#![allow(incomplete_features)]\n";
    s += "#![allow(clippy::excessive_precision)]\n";
    s += "#![allow(clippy::approx_constant)]\n";
    s += "\n";
    // `Kind` and the dimension are const generic parameter types: they derive
    // `ConstParamTy` (and the const `PartialEq` it builds on) here; yadc's
    // derive writes the algebra.
    s += "use core::marker::ConstParamTy;\n";
    s += "\n";
    s += "pub use yadc::{Dimension, Quantity, Unit};\n";
    s += "\n";

    validate_kind(&system.kind);
    s += "/// The kinds: how a quantity behaves under `+` and `-`, beside its\n";
    s += "/// exponents. The `#[default]` one is an ordinary amount.\n";
    if let Some(affine) = &system.kind.affine {
        s += &format!("/// `{affine}` is the affine kind, `#[kind(affine)]`: a\n");
        s += "/// point, measured from a zero somebody chose - two differ by an amount,\n";
        s += "/// and an amount moves one.\n";
    }
    if let Some(log) = &system.kind.log {
        s += &format!("/// `{log}` is the logarithmic kind, `#[kind(log)]`: a\n");
        s += "/// level or a gain, where adding is multiplying what it is the logarithm\n";
        s += "/// of.\n";
    }
    s += "/// Any other is a label of this system's own, telling apart quantities of\n";
    s += "/// the same exponents, with its sums as `#[kind(..)]` rules. Angles are not\n";
    s += "/// a kind but an exponent axis of the dimension (`rad`).\n";
    s += "///\n";
    s += "/// `#[derive(Dimension)]` on the enum writes the label algebra: `*` and `/`\n";
    s += "/// keep a label only beside a plain number; `+` and `-` work within one\n";
    s += "/// label, plus each variant's rules. A sum with no result is `None`.\n";
    s += "#[derive_const(PartialEq, Default)]\n";
    s += "#[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]\n";
    s += "#[cfg_attr(\n";
    s += "    feature = \"serde\",\n";
    s += "    derive(serde::Serialize, serde::Deserialize, yadc::UcumKind)\n";
    s += ")]\n";
    s += "pub enum Kind {\n";
    for v in &system.kind.variants {
        if *v == system.kind.default {
            s += "    #[default]\n";
        }
        let mut items: Vec<String> = Vec::new();
        if system.kind.affine.as_ref() == Some(v) {
            items.push("affine".into());
        }
        if system.kind.log.as_ref() == Some(v) {
            items.push("log".into());
        }
        if let Some(rules) = system.kind.rules.get(v) {
            items.extend(rules.iter().cloned());
        }
        if !items.is_empty() {
            s += &format!("    #[kind({})]\n", items.join(", "));
        }
        s += &format!("    {v},\n");
    }
    s += "}\n\n";

    s += "/// The dimension: an exponent per base unit, and a label. It prints as the\n";
    s += "/// product of its axes (`m·kg·s⁻²`), a label that is not plain in braces.\n";
    s += "#[derive_const(PartialEq)]\n";
    s += "#[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]\n";
    s += "#[allow(non_snake_case)]\n";
    s += "#[cfg_attr(feature = \"serde\", derive(yadc::UcumSerialize, yadc::UcumDeserialize))]\n";
    s += &format!("pub struct {dim} {{\n");
    for field in &system.dim.fields {
        s += &format!("    pub {field}: i8,\n");
    }
    s += "    #[kind]\n";
    s += "    pub kind: Kind,\n";
    s += "}\n\n";

    s += "/// A quantity whose dimension is in its type, counted in `N` (`f64` unless\n";
    s += "/// you say otherwise: `KILOPASCAL.cast::<f32>()`). Its number is private to\n";
    s += "/// this crate: the base constants (`base::METRE`) are the only raw numbers,\n";
    s += "/// and everyone else builds and reads through units - every constant is one\n";
    s += "/// (`KILOMETRE(3.0)` builds, `MILE(d)` reads), and so is every scale, a zero\n";
    s += "/// and a step, and every unit enum's value (`LengthUnit::Mile.value(d)`).\n";
    s += "///\n";
    s += "/// It serializes transparently as just its number: the dimension `D` is the\n";
    s += "/// deserialization target's - schema-trusted, nothing on the wire.\n";
    s += "#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, yadc::Quantity)]\n";
    s += "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]\n";
    s += &format!("pub struct {qty}<const D: {dim}, N = f64>(N);\n\n");
    s += "/// A scale: a zero and a step, `Scale(zero, step)`. A unit of its zero's type:\n";
    s += "/// `n` is `zero + n · step`, and a value reads `(value - zero) / step`. °C is\n";
    s += "/// `Scale(ICE_POINT, KELVIN)`; a scale made at run time is the same expression.\n";
    s += "/// Called like any unit: `CELSIUS(25.0)` builds, `CELSIUS(t)` reads.\n";
    s += "#[derive(Copy, Clone, PartialEq, Debug)]\n";
    s += "pub struct Scale<P, S>(pub P, pub S);\n\n";
    s += &SCALE_IMPLS.replace("@DIM@", dim).replace("@QTY@", qty);

    // A dimension with a kind field has partial products - a point or a
    // level has none - and the dynamic type's with them.
    let products = "Option<Self>";
    let products_doc = match (&system.kind.affine, &system.kind.log) {
        (None, None) => "A product or quotient always exists (`Some`); a sum or a\n/// scaling may not (`None` where the dimensions don't combine), so a calculation\n/// reads `(((a * b)? + c)? * 2.0)?`.".to_string(),
        (affine, log) => {
            let without = match (affine, log) {
                (Some(_), Some(_)) => "a point or a level",
                (Some(_), None) => "a point",
                _ => "a level",
            };
            format!(
                "Any of them may have no result - `None` where the dimensions don't combine:\n/// a sum of two dimensions, a product with {without} - so a calculation reads\n/// `(((a * b)? + c)? * 2.0)?`."
            )
        }
    };
    s += &DYNAMIC
        .replace("__PRODUCTS_DOC__", &products_doc)
        .replace("__PRODUCTS__", products)
        .replace("__DYN__", dyn_)
        .replace("__QTY__", qty)
        .replace("__DIM__", dim);
    s += "\n";
    s += &gen_base_mod(system);
    s += "\n";
    s += "pub mod dimensionless;\n";
    s += "\n";

    // Hand-written, non-generated modules (e.g. uom_bridge). Declared here but
    // their files are never written by the codegen, so regeneration leaves them
    // intact.
    if !system.system.extra_modules.is_empty() {
        for m in &system.system.extra_modules {
            if let Some(feat) = &m.feature {
                s += &format!("#[cfg(feature = \"{feat}\")]\n");
            }
            s += &format!("pub mod {};\n", m.name);
        }
        s += "\n";
    }

    let mut mod_names: Vec<&str> = system
        .system
        .family_order
        .iter()
        .filter(|name| {
            system
                .family
                .iter()
                .find(|f| &f.name == *name)
                .is_some_and(|f| !f.quantity.is_empty())
        })
        .map(|s| s.as_str())
        .collect();
    mod_names.sort_unstable();
    for fam_name in mod_names {
        s += &format!("#[cfg(feature = \"{fam_name}\")]\n");
        s += &format!("pub mod {fam_name};\n");
    }
    s
}

/// The dynamic type, as a user would write it with dynamickit: the dynamic
/// version of the static quantity (and of a bare `f64`, the same value as a
/// dimensionless quantity), every operator and method lifted from the
/// fields' own. `__DYN__`, `__QTY__` and `__DIM__` are the dynamic, static and
/// dimension type names.
const DYNAMIC: &str = r#"/// A quantity of any dimension, decided at run time: the dynamic version of
/// `__QTY__<D>`, built with dynamickit. Converting (`From`) moves the number into
/// `value` and records `D` in `dim`; `TryFrom<__DYN__> for __QTY__<D>` is the
/// checked way back (the error hands the original back). A bare `f64` is the
/// same value as a dimensionless quantity (`shared`). Numbers still go through
/// units, by way of the static quantity: `__DYN__::from(KILOMETRE.quantity(3.0))`,
/// then `Mass::try_from(d).map(|m| KILOGRAM.value(m))`.
///
/// Its arithmetic is every field's own: the dimension's operator on `dim`, the
/// number's on `value`. __PRODUCTS_DOC__ Values of one dimension are ordered by number,
/// `None` otherwise. A root or power with a run-time exponent is the
/// dimension's and the number's (`None` where the dimension has none).
/// Scaling by a number is the dimension's own too: its `* f64` and `/ f64`,
/// which `#[derive(Dimension)]` writes.
#[derive(Copy, Clone, PartialEq, Debug, dynamickit::Dynamic)]
#[dynamic(__QTY__<const D: __DIM__> => { dim: D, value: .0 })]
#[dynamic(shared f64 => { dim: __DIM__::DIMENSIONLESS, value: self })]
#[dynamic(impl(
    TryFrom,
    Mul -> __PRODUCTS__,
    Div -> __PRODUCTS__,
    Add -> Option<Self>,
    Sub -> Option<Self>,
    Mul<f64> -> Option<Self>,
    Div<f64> -> Option<Self>,
    PartialOrd,
    yadc::PartialRoot::partial_root(self, n: i8) -> Option<Self>,
    yadc::PartialPowi::partial_powi(self, n: i8) -> Option<Self>,
))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct __DYN__ {
    dim: __DIM__,
    value: f64,
}
"#;

/// Check `[kind]` against what `#[derive(Dimension)]` accepts, so a bad
/// config fails here with the reason rather than as a derive error in the
/// generated crate.
fn validate_kind(kind: &KindConfig) {
    let variants = &kind.variants;
    assert!(
        variants.contains(&kind.default),
        "[kind] default `{}` is not one of the variants {variants:?}",
        kind.default
    );
    assert!(
        !variants.iter().any(|v| v == "Invalid"),
        "[kind] variants include `Invalid`, but a rejected combination is `None`, not a kind: \
         remove it (write `op(..) = invalid` in [kind] rules to reject a combination)"
    );
    for (key, named) in [("affine", &kind.affine), ("log", &kind.log)] {
        if let Some(v) = named {
            assert!(
                variants.contains(v),
                "[kind] {key} `{v}` is not one of the variants {variants:?}"
            );
            assert!(
                *v != kind.default,
                "[kind] {key} `{v}` is the default: the default is the kind of an ordinary \
                 amount"
            );
        }
    }
    assert!(
        kind.affine.is_none() || kind.affine != kind.log,
        "[kind] affine and log name the same variant"
    );
    for (name, rules) in &kind.rules {
        assert!(
            variants.contains(name),
            "[kind] rules name `{name}`, which is not one of the [kind] variants {variants:?}"
        );
        for rule in rules {
            let Some((lhs, result)) = rule.split_once('=') else {
                panic!(
                    "[kind] rule `{rule}` is not `op(Operand) = Result`: a label declares its \
                     sums; the affine and the logarithmic kind are said with `affine = ..` and \
                     `log = ..` in [kind]"
                );
            };
            let (op, operand) = lhs
                .trim()
                .strip_suffix(')')
                .and_then(|l| l.split_once('('))
                .unwrap_or_else(|| panic!("[kind] rule `{rule}` is not `op(Operand) = Result`"));
            assert!(
                matches!(op.trim(), "add" | "sub"),
                "[kind] rule `{rule}`: `{}` is not `add` or `sub` - kinds are labels, and `*` / `/` \
                 are total (a label survives only beside a plain number)",
                op.trim()
            );
            for (n, result) in [(operand.trim(), false), (result.trim(), true)] {
                assert!(
                    variants.iter().any(|v| v == n) || (result && n == "invalid"),
                    "[kind] rule `{rule}`: `{n}` is not a [kind] variant {variants:?}{}",
                    if result { " or `invalid`" } else { "" }
                );
            }
        }
    }
}

// --- Unit constants -------------------------------------------------------------
//
// A module's `consts` holds a constant per multiple (`KILOMETRE = 1000.0 *
// METRE`), the quantities' extra constants (`ICE_POINT`, and a scale,
// `CELSIUS = Scale(ICE_POINT, KELVIN)`), and re-exports the base constants of
// its quantities. The unit enums below name them.

/// `KilowattHour` -> `KILOWATT_HOUR`, `Lithium7GyromagneticRatio` ->
/// `LITHIUM7_GYROMAGNETIC_RATIO`: an underscore before every uppercase letter
/// that follows a lowercase letter or a digit, or that starts a new word after
/// an uppercase run (`SIUnit` -> `SI_UNIT`).
fn screaming_snake(ident: &str) -> String {
    let chars: Vec<char> = ident.chars().collect();
    let mut out = String::new();
    for (i, &c) in chars.iter().enumerate() {
        if i > 0 && c.is_uppercase() {
            let prev = chars[i - 1];
            let next_lower = chars.get(i + 1).is_some_and(|n| n.is_lowercase());
            if prev.is_lowercase() || prev.is_ascii_digit() || (prev.is_uppercase() && next_lower) {
                out.push('_');
            }
        }
        out.extend(c.to_uppercase());
    }
    out
}

/// One quantity of a module, as the constants and units see it.
pub(crate) struct QuantityRef<'a> {
    name: &'a str,
    kind: Option<&'a str>,
    dims: &'a BTreeMap<String, i64>,
    consts: &'a [ExtraConst],
    /// The units, each variant once.
    units: Vec<&'a UnitDef>,
}

impl<'a> QuantityRef<'a> {
    fn new(
        name: &'a str,
        kind: Option<&'a str>,
        dims: &'a BTreeMap<String, i64>,
        consts: &'a [ExtraConst],
        units: &'a [UnitDef],
    ) -> Self {
        let mut seen = HashSet::new();
        let units = units
            .iter()
            .filter(|u| seen.insert(u.variant.as_str()))
            .collect();
        QuantityRef {
            name,
            kind,
            dims,
            consts,
            units,
        }
    }
}

/// What a constant is.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Role {
    /// A multiple's constant (`KILOMETRE`).
    Multiple,
    /// A quantity's extra constant, verbatim (`ICE_POINT`).
    Extra,
}

/// One constant of a module, before collision renaming.
struct UnitConst<'a> {
    /// The quantity it belongs to (the name prefix on a clash).
    owner: &'a str,
    /// Its type: the owner.
    ty: String,
    /// Its name before collision renaming.
    base_name: String,
    /// A fixed name: never prefixed (an extra constant's).
    fixed: bool,
    /// The unit it was made for, if any.
    unit: Option<&'a UnitDef>,
    role: Role,
    /// Its size in coherent units (multiples).
    multiplier: Option<&'a str>,
    /// The verbatim value (extra constants).
    value: Option<&'a str>,
    doc: String,
}

/// What a unit's enum variant names: `by = KILOMETRE`, `by = CELSIUS`.
enum VariantSpec {
    /// The constant at this index.
    Const(usize),
    /// A unit value named in the TOML.
    By(String),
}

/// A module's constants, named; and per unit what its variant names.
pub(crate) struct ModuleUnits<'a> {
    consts: Vec<UnitConst<'a>>,
    names: Vec<String>,
    variants: BTreeMap<(&'a str, &'a str), VariantSpec>,
    /// Per quantity, the index of its coherent constant (the first multiple
    /// of size 1).
    coherent: BTreeMap<&'a str, usize>,
}

fn is_one(m: &str) -> bool {
    m.trim().parse::<f64>() == Ok(1.0)
}

/// `m * C`: a factor that is not a plain literal is parenthesised.
fn scaled(m: &str, c: &str) -> String {
    let m = m.trim();
    if is_one(m) {
        c.to_string()
    } else if m.parse::<f64>().is_ok() {
        format!("{m} * {c}")
    } else {
        format!("({m}) * {c}")
    }
}

/// Plan a module's constants and unit variants.
pub(crate) fn plan_module<'a>(
    quantities: &[QuantityRef<'a>],
    affine: Option<&str>,
) -> ModuleUnits<'a> {
    let mut consts: Vec<UnitConst<'a>> = Vec::new();
    let mut coherent = BTreeMap::new();
    // Pass 1: the multiples (each quantity's coherent unit first), so the
    // other units can name them.
    for q in quantities {
        let mut first = None;
        for &unit in &q.units {
            if !unit.is_multiple() {
                continue;
            }
            let m = unit.multiplier.as_deref().unwrap_or_else(|| {
                panic!(
                    "unit `{}` of `{}` has no size: give it `m` (a multiple) or `by` (the unit \
                     value it is: another constant, a scale)",
                    unit.variant, q.name
                )
            });
            let base_name = unit
                .constant
                .clone()
                .unwrap_or_else(|| screaming_snake(&unit.variant));
            consts.push(UnitConst {
                owner: q.name,
                ty: q.name.to_string(),
                base_name,
                fixed: unit.constant.is_some(),
                unit: Some(unit),
                role: Role::Multiple,
                multiplier: Some(m),
                value: None,
                doc: format!(
                    "One {}, as {} `{}`.",
                    unit_label(unit),
                    article(q.name),
                    q.name
                ),
            });
            if first.is_none() && is_one(m) {
                first = Some(consts.len() - 1);
            }
        }
        if let Some(i) = first {
            coherent.insert(q.name, i);
        }
        for c in q.consts {
            consts.push(UnitConst {
                owner: q.name,
                ty: c.ty.clone().unwrap_or_else(|| q.name.to_string()),
                base_name: c.name.clone(),
                fixed: true,
                unit: None,
                role: Role::Extra,
                multiplier: None,
                value: Some(&c.value),
                doc: c.doc.clone(),
            });
        }
    }
    // Pass 2: what each unit's variant names.
    let mut variants = BTreeMap::new();
    for q in quantities {
        for &unit in &q.units {
            let spec = if let Some(by) = unit.by.as_deref() {
                VariantSpec::By(by.to_string())
            } else {
                let i = consts
                    .iter()
                    .position(|c| {
                        c.role == Role::Multiple
                            && c.owner == q.name
                            && c.unit.is_some_and(|u| u.variant == unit.variant)
                    })
                    .expect("planned in pass 1");
                VariantSpec::Const(i)
            };
            if q.kind.is_some() && q.kind == affine && matches!(spec, VariantSpec::Const(_)) {
                panic!(
                    "unit `{}` of the affine quantity `{}` is not a scale: a point has no \
                     \"one of it\" - give it `by`, a scale constant (a zero and a step)",
                    unit.variant, q.name
                );
            }
            variants.insert((q.name, unit.variant.as_str()), spec);
        }
    }
    let names = unit_const_names(&consts);
    ModuleUnits {
        consts,
        names,
        variants,
        coherent,
    }
}

/// Final names of one module's constants, by index. A name two of them share
/// is prefixed with the owning quantity for *every* entry that has it
/// (`TIME_...`, `ANGLE_...`), so the result does not depend on declaration
/// order. A name given in the TOML is never prefixed.
fn unit_const_names(consts: &[UnitConst<'_>]) -> Vec<String> {
    let mut count: BTreeMap<&str, usize> = BTreeMap::new();
    for c in consts {
        *count.entry(c.base_name.as_str()).or_default() += 1;
    }
    let names: Vec<String> = consts
        .iter()
        .map(|c| {
            if count[c.base_name.as_str()] > 1 && !c.fixed {
                format!("{}_{}", screaming_snake(c.owner), c.base_name)
            } else {
                c.base_name.clone()
            }
        })
        .collect();
    let mut seen = HashSet::new();
    for n in &names {
        assert!(
            seen.insert(n),
            "generated constant name `{n}` is still ambiguous after prefixing"
        );
    }
    names
}

/// `` `Kilometre` (km) ``, or just `` `Ratio` `` for a unit with no printable
/// symbol.
fn unit_label(unit: &UnitDef) -> String {
    let symbol = unit.symbol.replace('\n', " ");
    if symbol.is_empty() || symbol.contains('`') {
        format!("`{}`", unit.variant)
    } else {
        format!("`{}` ({symbol})", unit.variant)
    }
}

fn article(word: &str) -> &'static str {
    if word.starts_with(['A', 'E', 'I', 'O', 'U']) {
        "an"
    } else {
        "a"
    }
}

/// The identifiers in an expression.
fn idents(expr: &str) -> BTreeSet<String> {
    expr.split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .filter(|w| {
            w.chars()
                .next()
                .is_some_and(|c| c.is_alphabetic() || c == '_')
        })
        .map(str::to_string)
        .collect()
}

/// A module's constants and imports, rendered.
struct Rendered {
    consts: String,
    /// The base constants re-exported by `consts` (so in scope through
    /// `use consts::*`).
    reexported: BTreeSet<String>,
}

/// `pub mod consts { .. }`: the base constants of the module's quantities
/// (re-exported), then each constant of the module, defined from them.
fn gen_consts_mod(
    plan: &ModuleUnits<'_>,
    quantities: &[QuantityRef<'_>],
    base: &Base<'_>,
    fields: &[String],
) -> Rendered {
    // Base constants of this module's quantities.
    let mut reexport: Vec<&BaseConst> = Vec::new();
    for b in base.consts {
        if quantities
            .iter()
            .any(|q| same_dims(&b.dims, q.dims) && b.kind.as_deref() == q.kind)
        {
            reexport.push(b);
        }
    }
    // The module's coherent constants that *are* a base constant of the same
    // name (`METRE` of `Length`): re-exported, not defined.
    let itself: BTreeSet<&str> = quantities
        .iter()
        .filter_map(|q| {
            let i = *plan.coherent.get(q.name)?;
            let b = base.exact(q.dims, q.kind)?;
            (b.name == plan.names[i]).then_some(b.name.as_str())
        })
        .collect();
    // A base constant another module constant shadows is not re-exported.
    reexport.retain(|b| itself.contains(b.name.as_str()) || !plan.names.contains(&b.name));
    let reexported: BTreeSet<String> = reexport.iter().map(|b| b.name.clone()).collect();
    let mut items = String::new();
    let mut used: BTreeSet<String> = BTreeSet::new();
    let shadowed: BTreeSet<&str> = plan
        .names
        .iter()
        .map(String::as_str)
        .filter(|n| !itself.contains(n))
        .collect();
    for (i, c) in plan.consts.iter().enumerate() {
        let name = &plan.names[i];
        let q = quantities.iter().find(|q| q.name == c.owner).unwrap();
        let expr = match c.role {
            Role::Extra => c.value.unwrap().to_string(),
            Role::Multiple => {
                let m = c.multiplier.unwrap();
                let coh = plan.coherent.get(q.name).copied();
                if coh == Some(i) {
                    // The coherent unit: a base constant, or their product.
                    match base.exact(q.dims, q.kind) {
                        Some(b) => {
                            if b.name == *name {
                                continue; // re-exported below
                            }
                            qualified(&b.name, &shadowed)
                        }
                        None => coherent_expr(q, base, fields, &shadowed),
                    }
                } else if let Some(k) = coh {
                    scaled(m, &plan.names[k])
                } else {
                    let e = match base.exact(q.dims, q.kind) {
                        Some(b) => qualified(&b.name, &shadowed),
                        None => coherent_expr(q, base, fields, &shadowed),
                    };
                    if is_one(m) {
                        e
                    } else if e.contains(' ') || e.contains('.') {
                        scaled(m, &format!("({e})"))
                    } else {
                        scaled(m, &e)
                    }
                }
            }
        };
        used.extend(idents(&expr));
        items += &format!("\n    /// {}\n", c.doc);
        items += &format!("    pub const {name}: {} = {expr};\n", c.ty);
    }
    if items.is_empty() && reexport.is_empty() {
        return Rendered {
            consts: String::new(),
            reexported,
        };
    }
    let mut s = String::new();
    s += "/// The constants of this module: the base constants of its quantities\n";
    s += "/// (re-exported), then one per multiple, each defined from them - `KILOMETRE`\n";
    s += "/// is `1000.0 * METRE`, a `Length` and a unit of its own (`KILOMETRE.value(d)`).\n";
    s += "/// Quantity arithmetic is `const`, so they compose in `const` items:\n";
    s += "/// `const LIMIT: Velocity = 50.0 * KILOMETRE / HOUR;`.\n";
    s += "pub mod consts {\n";
    s += "    use super::*;\n";
    let local: BTreeSet<&str> = plan.names.iter().map(String::as_str).collect();
    let from_base: Vec<&str> = base
        .consts
        .iter()
        .map(|b| b.name.as_str())
        .filter(|n| used.contains(*n) && !reexported.contains(*n) && !local.contains(n))
        .collect();
    if !from_base.is_empty() {
        s += &format!("    use crate::base::{{{}}};\n", from_base.join(", "));
    }
    for b in &reexport {
        s += &format!("\n    /// {}\n", b.doc);
        s += &format!("    pub use crate::base::{};\n", b.name);
    }
    s += &items;
    s += "}\n";
    Rendered {
        consts: s,
        reexported,
    }
}

/// `crate::base::NAME` where a module constant shadows `NAME`.
fn qualified(name: &str, shadowed: &BTreeSet<&str>) -> String {
    if shadowed.contains(name) {
        format!("crate::base::{name}")
    } else {
        name.to_string()
    }
}

/// The coherent unit of a quantity with no base constant of its own: the
/// product of base constants. A product has the plain label, so a quantity
/// with a label takes its coherent unit from a `[[base]]` constant.
fn coherent_expr(
    q: &QuantityRef<'_>,
    base: &Base<'_>,
    fields: &[String],
    shadowed: &BTreeSet<&str>,
) -> String {
    if let Some(kind) = q.kind {
        panic!(
            "quantity `{}` has the label `{kind}`: no product of base constants has a label, so \
             its coherent unit must be a `[[base]]` constant with the same `dims` and `kind`",
            q.name
        );
    }
    base.product(q.dims, fields, q.name, shadowed)
}

// --- dimensionless.rs --------------------------------------------------------

pub fn gen_dimensionless_rs(system: &UnitSystem) -> String {
    let dim = dim_ty(&system.system);
    let qty = qty_ty(&system.system);
    let no_dims = BTreeMap::new();
    let quantities = [QuantityRef::new(
        "Dimensionless",
        None,
        &no_dims,
        &[],
        &system.dimensionless.units,
    )];
    gen_module(
        &quantities,
        system,
        &format!("pub type Dimensionless = {qty}<{{ {dim}::DIMENSIONLESS }}>;\n"),
        "//! Plain numbers: ratios, percentages, and levels in decibels. Variants\n\
         //! mirror uom's `ratio.rs` quantity.\n",
        false,
    )
}

// --- <family>.rs -------------------------------------------------------------

pub fn gen_family_rs(family: &FamilyDef, system: &UnitSystem) -> String {
    let dim = dim_ty(&system.system);
    let qty = qty_ty(&system.system);
    let quantities: Vec<QuantityRef<'_>> = family
        .quantity
        .iter()
        .map(|q| QuantityRef::new(&q.name, q.kind.as_deref(), &q.dims, &q.consts, &q.units))
        .collect();
    let mut aliases = String::new();
    for q in &family.quantity {
        aliases += "\n";
        aliases += &fmt_type_alias(
            &q.name,
            &q.dims,
            q.kind.as_deref(),
            &system.dim.fields,
            dim,
            qty,
        );
    }
    let uses_kind = family.quantity.iter().any(|q| q.kind.is_some());
    gen_module(&quantities, system, &aliases, "", uses_kind)
}

/// A module: imports, the quantities' aliases, the constants and the unit
/// enums.
fn gen_module(
    quantities: &[QuantityRef<'_>],
    system: &UnitSystem,
    aliases: &str,
    header: &str,
    uses_kind: bool,
) -> String {
    let dim = dim_ty(&system.system);
    let qty = qty_ty(&system.system);
    let base = Base::new(system);
    let plan = plan_module(quantities, system.kind.affine.as_deref());
    let rendered = gen_consts_mod(&plan, quantities, &base, &system.dim.fields);
    let mut enums = String::new();
    for q in quantities {
        let e = gen_unit_enum(q, &plan, system);
        if !e.is_empty() {
            enums += "\n";
            enums += &e;
        }
    }
    // Base constants the enums name that `consts` does not re-export.
    let used = idents(&enums);
    let local: BTreeSet<&str> = plan.names.iter().map(String::as_str).collect();
    let from_base: Vec<&str> = base
        .consts
        .iter()
        .map(|b| b.name.as_str())
        .filter(|n| used.contains(*n) && !rendered.reexported.contains(*n) && !local.contains(n))
        .collect();

    let mut s = String::new();
    s += header;
    if !header.is_empty() {
        s += "\n";
    }
    let mut names = vec![dim, qty];
    if uses_kind {
        names.push("Kind");
    }
    // The scale type, where a constant or an enum of the module names it.
    if idents(&rendered.consts).contains("Scale") || used.contains("Scale") {
        names.push("Scale");
    }
    names.sort_unstable();
    s += &format!("use crate::{{{}}};\n", names.join(", "));
    if !from_base.is_empty() {
        s += &format!("use crate::base::{{{}}};\n", from_base.join(", "));
    }
    let in_consts = plan
        .names
        .iter()
        .chain(rendered.reexported.iter())
        .any(|n| used.contains(n));
    if !rendered.consts.is_empty() && in_consts {
        s += "use consts::*;\n";
    }
    s += "use yadc::Dimension;\n";
    s += "\n";
    s += aliases.trim_start_matches('\n');
    if !rendered.consts.is_empty() {
        s += "\n";
        s += &rendered.consts;
    }
    s += &enums;
    s
}

/// `use`-free list of what `#[derive(..)]` a unit enum gets.
fn fmt_derive_attr(extra: &[String]) -> String {
    let mut all = vec!["yadc::Unit".to_string(), "dynamickit::Variants".to_string()];
    all.extend(extra.iter().cloned());
    format!("#[derive({})]\n", all.join(", "))
}

// --- Unit enums ----------------------------------------------------------------

/// `<Q>Unit`, the unit enum of quantity `q`: every unit of `q`, a variant
/// each naming the unit value it is - a multiple its constant (`by =
/// KILOMETRE`), a scale its constant (`by = CELSIUS`), one that measures
/// another type the type it converts through (`by = DBM, via = PowerLevel`).
/// Nothing for a quantity without units.
fn gen_unit_enum(q: &QuantityRef<'_>, plan: &ModuleUnits<'_>, system: &UnitSystem) -> String {
    if q.units.is_empty() {
        return String::new();
    }
    // The coherent unit is the `#[default]`: what a menu starts on. Failing
    // that, the first unit.
    let default = q
        .units
        .iter()
        .position(|u| u.is_multiple() && u.multiplier.as_deref().is_some_and(is_one))
        .unwrap_or(0);
    let non_const = q.units.iter().any(|u| u.non_const);
    let mut s = format!(
        "/// The units of `{}`, for a menu, a config file, a parser.\n",
        q.name
    );
    s += "#[derive_const(Default)]\n";
    s += &fmt_derive_attr(&system.system.unit_derives);
    // A level's conversion is not `const`, so neither is the enum's.
    if non_const {
        s += &format!("#[unit(of = {}, non_const)]\n", q.name);
    } else {
        s += &format!("#[unit(of = {})]\n", q.name);
    }
    s += &format!("pub enum {}Unit {{\n", q.name);
    for (i, unit) in q.units.iter().enumerate() {
        if i == default {
            s += "    #[default]\n";
        }
        let spec = match &plan.variants[&(q.name, unit.variant.as_str())] {
            VariantSpec::Const(k) => format!("by = {}", plan.names[*k]),
            VariantSpec::By(t) => match &unit.via {
                Some(via) => format!("by = {t}, via = {via}"),
                None => format!("by = {t}"),
            },
        };
        s += &format!(
            "    #[unit(\"{}\", \"{}\", {spec})]\n",
            rust_str(&unit.symbol),
            rust_str(&unit.long)
        );
        s += &format!("    {},\n", unit.variant);
    }
    s += "}\n";
    s
}

// --- Cargo.toml --------------------------------------------------------------

pub fn gen_cargo_toml(system: &UnitSystem) -> String {
    let nonempty: Vec<&str> = system
        .system
        .family_order
        .iter()
        .filter_map(|name| {
            system
                .family
                .iter()
                .find(|f| &f.name == name)
                .filter(|f| !f.quantity.is_empty())
                .map(|_| name.as_str())
        })
        .collect();

    let mut s = String::new();
    s += "[package]\n";
    s += &format!("name = \"{}\"\n", system.system.crate_name);
    s += "version = \"0.1.0\"\n";
    s += "edition = \"2024\"\n";
    s += "license = \"MIT OR BSD-3-Clause\"\n";
    // Both deps are pulled in without their default features so that
    // `--no-default-features` on this crate really is `no_std`; `std` and `libm`
    // forward to `yadc`, which in turn feeds `num-traits` (the `Real` impls the
    // nonlinear unit conversions need).
    s += "\n[dependencies]\n";
    s += "yadc = { path = \"../yadc\", default-features = false }\n";
    s += "dynamickit = { path = \"../dynamickit\" }\n";
    s += "serde = { version = \"1\", default-features = false, features = [\"derive\"], optional = true }\n";
    s += "\n[features]\n";
    s += "default = [\n";
    s += "    \"std\",\n";
    for f in &nonempty {
        s += &format!("    \"{f}\",\n");
    }
    s += "]\n";
    s += "std = [\"yadc/std\", \"serde?/std\"]\n";
    s += "# Software float math for a `no_std` build: `Real` (and so the nonlinear\n";
    s += "# unit conversions and the dynamic type's roots) needs either this or `std`.\n";
    s += "libm = [\"yadc/libm\"]\n";
    for f in &nonempty {
        s += &format!("{f} = []\n");
    }
    s += "# `alloc`: the UCUM (de)serializers build owned `String`s.\n";
    s += "serde = [\"dep:serde\", \"serde/alloc\", \"yadc/ucum\"]\n";
    s
}

// --- Dim type alias formatting ------------------------------------------------

/// Build the ordered list of non-zero field expressions.
fn dim_field_exprs(
    dims: &BTreeMap<String, i64>,
    kind: Option<&str>,
    fields: &[String],
) -> Vec<String> {
    let mut parts = Vec::new();
    for f in fields {
        if let Some(&v) = dims.get(f)
            && v != 0
        {
            parts.push(format!("{f}: {v}"));
        }
    }
    if let Some(k) = kind {
        parts.push(format!("kind: Kind::{k}"));
    }
    parts
}

fn fmt_type_alias(
    qty_name: &str,
    dims: &BTreeMap<String, i64>,
    kind: Option<&str>,
    fields: &[String],
    dim_ty: &str,
    qty_ty: &str,
) -> String {
    format!(
        "pub type {qty_name} = {qty_ty}<{}>;\n",
        dim_value(dims, kind, fields, dim_ty)
    )
}

/// Escape a string for a single-line Rust string literal.
///
/// Line breaks and tabs are escaped rather than passed through: some UCUM
/// `printSymbol`s carry literal newlines.
fn rust_str(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE: &str = r#"
[system]
crate_name = "t"
family_order = ["thermal"]
unit_derives = ["Copy", "Clone"]

[kind]
variants = ["Linear", "Affine", "Log", "Rank"]
default = "Linear"
affine = "Affine"
log = "Log"
rules = { Rank = ["add(Rank) = invalid"] }

[dim]
fields = ["m", "s", "K"]

[[base]]
name = "ONE"
doc = "One."

[[base]]
name = "METRE"
dims = {m = 1}
doc = "The metre."

[[base]]
name = "SECOND"
dims = {s = 1}
doc = "The second."

[[base]]
name = "KELVIN"
dims = {K = 1}
doc = "The kelvin, a step."

[[base]]
name = "ABSOLUTE_ZERO"
dims = {K = 1}
kind = "Affine"
value = "0.0"
doc = "Absolute zero."

[[base]]
name = "FIRST"
kind = "Rank"
doc = "The first rank."

[dimensionless]
units = [
    {v = "Ratio", m = "1.0", s = "", l = ""},
    {v = "Percent", m = "1.0e-2", s = "%", l = "percent"},
]
"#;

    fn system(families: &str) -> UnitSystem {
        toml::from_str(&format!("{BASE}{families}")).unwrap()
    }

    const THERMAL: &str = r#"
[[family]]
name = "thermal"

[[family.quantity]]
name = "ThermodynamicTemperature"
dims = {K = 1}
kind = "Affine"
consts = [
    {name = "ICE_POINT", value = "ABSOLUTE_ZERO + 273.15 * KELVIN", doc = "0 °C."},
    {name = "CELSIUS", ty = "Scale<ThermodynamicTemperature, TemperatureInterval>", value = "Scale(ICE_POINT, KELVIN)", doc = "The Celsius scale."},
]
units = [
    {v = "Kelvin", s = "K", l = "kelvin", by = "Scale(ABSOLUTE_ZERO, KELVIN)"},
    {v = "Celsius", s = "°C", l = "degrees Celsius", by = "CELSIUS"},
]

[[family.quantity]]
name = "TemperatureInterval"
dims = {K = 1}
units = [
    {v = "Kelvin", m = "1.0", s = "K", l = "kelvin"},
    {v = "Celsius", s = "°C", l = "degrees Celsius", by = "KELVIN"},
    {v = "Fahrenheit", m = "5.0 / 9.0", c = "DEGREE_FAHRENHEIT", s = "°F", l = "degrees Fahrenheit"},
]

[[family.quantity]]
name = "Speed"
dims = {m = 1, s = -1}
units = [
    {v = "MetrePerSecond", m = "1.0", s = "m/s", l = "metres per second"},
    {v = "KilometrePerHour", m = "0.27777777777777779", s = "km/h", l = "kilometres per hour"},
]

[[family.quantity]]
name = "Frequency"
dims = {s = -1}
units = [{v = "Kilohertz", m = "1000.0", s = "kHz", l = "kilohertz"}]

[[family.quantity]]
name = "Loudness"
dims = {m = 1}
units = [
    {v = "Metre", m = "1.0", s = "m", l = "metres"},
    {v = "Phon", s = "phon", l = "phons", by = "crate::level::PHON", via = "crate::level::LoudnessLevel", non_const = true},
]

[[family.quantity]]
name = "Rank"
kind = "Rank"
units = [
    {v = "First", m = "1.0", s = "", l = "firsts"},
    {v = "Third", m = "3.0", s = "rd", l = "thirds"},
]
"#;

    fn family() -> String {
        let sys = system(THERMAL);
        gen_family_rs(&sys.family[0], &sys)
    }

    /// The base constants are the only raw numbers; everything else is a
    /// product of them, a multiple of the coherent unit, or verbatim.
    #[test]
    fn constants_are_defined_from_the_base() {
        let lib = gen_lib_rs(&system(THERMAL));
        assert!(lib.contains(
            "    pub const METRE: Qty<{ Dim { m: 1, ..Dim::DIMENSIONLESS } }> = Qty(1.0);\n"
        ));
        assert!(lib.contains("pub const ABSOLUTE_ZERO: Qty<{ Dim { K: 1, kind: Kind::Affine, ..Dim::DIMENSIONLESS } }> = Qty(0.0);"));
        let out = family();
        assert!(out.contains("    pub const METRE_PER_SECOND: Speed = METRE / SECOND;\n"));
        assert!(out.contains(
            "    pub const KILOMETRE_PER_HOUR: Speed = 0.27777777777777779 * METRE_PER_SECOND;\n"
        ));
        // No coherent unit of its own: the product, scaled.
        assert!(out.contains("    pub const KILOHERTZ: Frequency = 1000.0 * (ONE / SECOND);\n"));
        assert!(out.contains(
            "    pub const DEGREE_FAHRENHEIT: TemperatureInterval = (5.0 / 9.0) * KELVIN;\n"
        ));
        assert!(out.contains(
            "    pub const ICE_POINT: ThermodynamicTemperature = ABSOLUTE_ZERO + 273.15 * KELVIN;\n"
        ));
        // A base constant is re-exported, not redefined.
        assert!(out.contains("    pub use crate::base::KELVIN;\n"));
        assert!(out.contains("    pub use crate::base::ABSOLUTE_ZERO;\n"));
        assert!(!out.contains("pub const KELVIN"));
        assert!(!out.contains("Qty("));
        // A label: its coherent unit is a base constant, re-exported.
        assert!(out.contains("    pub use crate::base::FIRST;\n"));
        assert!(!out.contains("pub const FIRST"));
    }

    /// A temperature scale is a constant of its quantity's `consts`, a
    /// `Scale` (the system's scale type, imported where a module names it),
    /// or a scale written in place; the enum names either. A unit of another
    /// constant's size has no constant of its own.
    #[test]
    fn scales_are_constants() {
        let lib = gen_lib_rs(&system(THERMAL));
        assert!(lib.contains("const impl<P, S, N> Unit<P, N> for Scale<P, S>\n"));
        assert!(lib.contains("pub struct Scale<P, S>(pub P, pub S);\n"));
        assert!(lib.contains("#![feature(fn_traits)]\n#![feature(unboxed_closures)]\n"));
        let out = family();
        assert!(out.contains("use crate::{Dim, Kind, Qty, Scale};\n"));
        assert!(out.contains(
            "    pub const CELSIUS: Scale<ThermodynamicTemperature, TemperatureInterval> =\n        Scale(ICE_POINT, KELVIN);\n"
        ) || out.contains(
            "    pub const CELSIUS: Scale<ThermodynamicTemperature, TemperatureInterval> = Scale(ICE_POINT, KELVIN);\n"
        ));
        assert!(
            out.contains("    #[unit(\"°C\", \"degrees Celsius\", by = CELSIUS)]\n    Celsius,\n")
        );
        assert!(out.contains(
            "    #[unit(\"K\", \"kelvin\", by = Scale(ABSOLUTE_ZERO, KELVIN))]\n    Kelvin,\n"
        ));
        assert!(
            out.contains("    #[unit(\"°C\", \"degrees Celsius\", by = KELVIN)]\n    Celsius,\n")
        );
        assert!(out.contains("    #[unit(\"°F\", \"degrees Fahrenheit\", by = DEGREE_FAHRENHEIT)]\n    Fahrenheit,\n"));
        assert!(!out.contains("pub struct"));
        assert!(!out.contains("RANKINE"));
    }

    /// A unit value that measures another type - a level, defined in a
    /// hand-written module - is named verbatim with the type it converts
    /// through; its conversion is not `const`, which makes its enum an
    /// ordinary impl.
    #[test]
    fn non_const_units_make_the_enum_non_const() {
        let out = family();
        assert!(out.contains("#[unit(of = Loudness, non_const)]\npub enum LoudnessUnit {"));
        assert!(
            out.contains("    #[unit(\"phon\", \"phons\", by = crate::level::PHON, via = crate::level::LoudnessLevel)]\n    Phon,\n")
        );
        assert!(!out.contains("PHON_REFERENCE"));
        assert!(out.contains("#[unit(of = Speed)]\npub enum SpeedUnit {"));
        let d = gen_dimensionless_rs(&system(THERMAL));
        assert!(d.contains("    pub const RATIO: Dimensionless = ONE;\n"));
        assert!(d.contains("    pub const PERCENT: Dimensionless = 1.0e-2 * RATIO;\n"));
        assert!(d.contains("    pub use crate::base::ONE;\n"));
        assert!(!d.contains("impl ::core::convert::From"));
    }

    #[test]
    fn unit_enums_name_their_units() {
        let out = family();
        assert!(out.contains(
            "#[derive_const(Default)]\n#[derive(yadc::Unit, dynamickit::Variants, Copy, Clone)]\n#[unit(of = Speed)]\npub enum SpeedUnit {\n    #[default]\n    #[unit(\"m/s\", \"metres per second\", by = METRE_PER_SECOND)]\n    MetrePerSecond,\n"
        ));
    }

    #[test]
    fn generated_lib() {
        let out = gen_lib_rs(&system(THERMAL));
        assert!(out.contains("pub enum Kind {\n    #[default]\n    Linear,\n    #[kind(affine)]\n    Affine,\n    #[kind(log)]\n    Log,\n    #[kind(add(Rank) = invalid)]\n    Rank,\n}\n"));
        // The kind's variants say which is affine and which logarithmic;
        // the field is marked alone, its products partial, and the dynamic
        // type's too.
        assert!(out.contains("    #[kind]\n    pub kind: Kind,\n"));
        assert!(out.contains("    Mul -> Option<Self>,\n    Div -> Option<Self>,\n    Add -> Option<Self>,\n"));
        assert!(out.contains("#[dynamic(Qty<const D: Dim> => { dim: D, value: .0 })]\n"));
        assert!(
            out.contains("#[dynamic(shared f64 => { dim: Dim::DIMENSIONLESS, value: self })]\n")
        );
        assert!(out.contains("pub struct Qty<const D: Dim, N = f64>(N);\n"));
        assert!(!out.contains("impl PartialRoot"));
        for gone in [
            "Label",
            "powi_by_mul",
            "yadc::math",
            "Offset",
            "Log<",
        ] {
            assert!(!out.contains(gone), "{gone}");
        }
    }

    #[test]
    fn screaming_snake_names() {
        assert_eq!(screaming_snake("KilowattHour"), "KILOWATT_HOUR");
        assert_eq!(
            screaming_snake("Lithium7GyromagneticRatio"),
            "LITHIUM7_GYROMAGNETIC_RATIO"
        );
        assert_eq!(screaming_snake("SIUnit"), "SI_UNIT");
        assert_eq!(screaming_snake("Percent"), "PERCENT");
    }

    /// A name two constants of one module share is prefixed with the quantity
    /// for every entry that has it; unique names stay bare.
    #[test]
    fn unit_constant_collisions_are_prefixed_within_a_module() {
        let sys = system(
            r#"
[[family]]
name = "thermal"
[[family.quantity]]
name = "Time"
dims = {s = 1}
units = [
    {v = "Second", m = "1.0", s = "s", l = "seconds"},
    {v = "Minute", m = "60.0", s = "min", l = "minutes"},
]
[[family.quantity]]
name = "Angle"
dims = {m = 1}
units = [
    {v = "Metre", m = "1.0", s = "m", l = "metres"},
    {v = "Minute", m = "2.9e-4", s = "'", l = "minutes of arc"},
]
"#,
        );
        let out = gen_family_rs(&sys.family[0], &sys);
        assert!(out.contains("    pub const TIME_MINUTE: Time = 60.0 * SECOND;"));
        assert!(out.contains("    pub const ANGLE_MINUTE: Angle = 2.9e-4 * METRE;"));
        assert!(
            out.contains("    #[unit(\"min\", \"minutes\", by = TIME_MINUTE)]\n    Minute,\n")
        );
    }

    /// Without an affine or a logarithmic kind no variant is marked; the
    /// products stay partial, as for any dimension with a kind field.
    #[test]
    fn labels_alone_mark_no_variant() {
        let sys: UnitSystem = toml::from_str(
            &BASE
                .replace("affine = \"Affine\"\nlog = \"Log\"\n", "")
                .replace("kind = \"Affine\"", "kind = \"Rank\""),
        )
        .unwrap();
        let out = gen_lib_rs(&sys);
        assert!(out.contains("    #[kind]\n    pub kind: Kind,\n"));
        assert!(!out.contains("#[kind(affine)]"));
        assert!(out.contains("    Mul -> Option<Self>,\n    Div -> Option<Self>,\n"));
    }

    #[test]
    #[should_panic(expected = "is not a scale")]
    fn affine_units_are_scales() {
        let sys = system(
            r#"
[[family]]
name = "thermal"
[[family.quantity]]
name = "ThermodynamicTemperature"
dims = {K = 1}
kind = "Affine"
units = [{v = "Kelvin", m = "1.0", s = "K", l = "kelvin"}]
"#,
        );
        gen_family_rs(&sys.family[0], &sys);
    }

    #[test]
    #[should_panic(expected = "`div` is not `add` or `sub` - kinds are labels")]
    fn mul_and_div_rules_are_rejected() {
        let sys: UnitSystem =
            toml::from_str(&BASE.replace("add(Rank) = invalid", "div(Rank) = Rank")).unwrap();
        gen_lib_rs(&sys);
    }

    /// A rule is a sum; which variant is affine or logarithmic is its own key.
    #[test]
    #[should_panic(expected = "is not `op(Operand) = Result`")]
    fn bare_rules_are_rejected() {
        let sys: UnitSystem =
            toml::from_str(&BASE.replace("add(Rank) = invalid", "Affine")).unwrap();
        gen_lib_rs(&sys);
    }

    #[test]
    #[should_panic(expected = "[kind] affine `Point` is not one of the variants")]
    fn named_kinds_are_variants() {
        let sys: UnitSystem =
            toml::from_str(&BASE.replace("affine = \"Affine\"", "affine = \"Point\"")).unwrap();
        gen_lib_rs(&sys);
    }

    /// A unit is a multiple (`m`) or a unit value (`by`): any other
    /// description is an error, not a field quietly left out.
    #[test]
    fn unknown_unit_fields_are_rejected() {
        let parsed: Result<UnitSystem, _> = toml::from_str(&format!(
            "{BASE}{}",
            r#"
[[family]]
name = "thermal"
[[family.quantity]]
name = "Loudness"
dims = {m = 1}
units = [{v = "Phon", m = "1.0", s = "phon", l = "phons", scale = "pow10"}]
"#
        ));
        let message = parsed.err().expect("an unknown field").to_string();
        assert!(message.contains("unknown field `scale`"), "{message}");
    }
}
