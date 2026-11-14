//! The generator: a `UnitSystem` to a crate's `src/*.rs` (and a starting
//! `Cargo.toml`).
//!
//! The output is laid out by `rustfmt` after it is written, so it is
//! fmt-clean and the same for the same input.

use crate::model::*;
use std::collections::{BTreeMap, HashSet};
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
fn unit_ty(sys: &SystemConfig) -> &str {
    sys.unit_type.as_deref().unwrap_or("QtyUnit")
}

/// The names a generated file refers to: the system's own types, by role.
struct Names<'a> {
    dim: &'a str,
    qty: &'a str,
    dyn_: &'a str,
    unit: &'a str,
    /// `SIUnitConsts`: the unit's const parameter.
    consts: String,
    /// `DynamicSIUnit`: a unit of any dimension, chosen at run time.
    dyn_unit: String,
}

impl<'a> Names<'a> {
    fn of(sys: &'a SystemConfig) -> Self {
        let unit = unit_ty(sys);
        Names {
            dim: dim_ty(sys),
            qty: qty_ty(sys),
            dyn_: dyn_ty(sys),
            unit,
            consts: format!("{unit}Consts"),
            dyn_unit: format!("Dynamic{unit}"),
        }
    }
}

// --- Public entry point -------------------------------------------------------

pub fn write_crate(system: &UnitSystem, output_dir: &Path) -> std::io::Result<()> {
    let src = output_dir.join("src");
    let dynamic = src.join("dynamic");
    std::fs::create_dir_all(&dynamic)?;
    let mut written = Vec::new();
    let mut write = |path: std::path::PathBuf, text: &str| -> std::io::Result<()> {
        std::fs::write(&path, text)?;
        written.push(path);
        Ok(())
    };
    write(src.join("lib.rs"), &gen_lib_rs(system))?;
    write(src.join("exact.rs"), EXACT)?;
    write(src.join("prefix.rs"), PREFIX)?;
    std::fs::write(output_dir.join("Cargo.toml"), gen_cargo_toml(system))?;

    for fam in &system.family {
        assert!(
            system.system.family_order.contains(&fam.name),
            "family `{}` is not listed in [system].family_order: add it",
            fam.name,
        );
    }
    // Each module's quantities and unit types at the root, its unit enums in
    // `dynamic`.
    let mut unit_mods: Vec<(String, Option<String>)> = Vec::new();
    let mut module = |name: &str, feature: Option<&str>, m: Module| -> std::io::Result<()> {
        write(src.join(format!("{name}.rs")), &m.quantities)?;
        if !m.units.is_empty() {
            write(dynamic.join(format!("{name}.rs")), &m.units)?;
            unit_mods.push((name.to_string(), feature.map(str::to_string)));
        }
        Ok(())
    };
    module("dimensionless", None, gen_dimensionless_rs(system))?;
    for fam_name in &system.system.family_order {
        let Some(fam) = system.family.iter().find(|f| &f.name == fam_name) else {
            continue;
        };
        if fam.quantity.is_empty() {
            continue;
        }
        module(fam_name, Some(fam_name), gen_family_rs(fam, system))?;
    }
    write(dynamic.join("mod.rs"), &gen_dynamic_rs(system, &unit_mods))?;
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

/// The exact number a unit's size and zero are written in, as `src/exact.rs`.
const EXACT: &str = include_str!("../templates/exact.rs");

/// The SI prefixes and `Prefixed`, written as `src/prefix.rs`.
const PREFIX: &str = include_str!("../templates/prefix.rs");

/// The unit type, its const parameter, and its one `Unit` impl: the bridge
/// from numbers to quantities. `__QTY__`, `__DIM__`, `__UNIT__` and
/// `__CONSTS__` are the system's type names.
const UNIT: &str = r#"/// What a unit is: its size and its zero in the coherent unit of its
/// quantity, exact, and its symbol and name. A number `n` in the unit is `n ·
/// size + zero` coherent units: the inch is `size` 0.0254 (metres), the
/// degree Celsius `size` 1 and `zero` 273.15 (kelvins). The const parameter
/// of `__UNIT__`.
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy)]
pub struct __CONSTS__ {
    pub size: Exact,
    pub zero: Exact,
    pub symbol: &'static str,
    pub name: &'static str,
}

/// A unit: a type of its own, its quantity's dimension `D` and its
/// definition `C` in its const parameters (`units::Mile`, in a module's
/// `units`), built
/// as `Mile {}`. A unit of `__QTY__<D, N>` in any number type its size and zero
/// convert to: `Mile {}.quantity(3.0)` builds, `Mile {}.value(d)` reads,
/// rounding the size and zero once each, at compile time. Prints its symbol
/// (`{}`) or name (`{:#}`).
///
/// An integer quantity counts in units whose size is a whole number of
/// coherent units: its size and zero are rounded to whole numbers, so a unit
/// smaller than one coherent unit has none in integers.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct __UNIT__<const D: __DIM__, const C: __CONSTS__>;

/// The bridge from numbers to quantities, and back: the only place a
/// number becomes a quantity.
const impl<const D: __DIM__, const C: __CONSTS__, N> Unit<__QTY__<D, N>, N> for __UNIT__<D, C>
where
    N: const From<Exact>
        + [const] core::ops::Add<Output = N>
        + [const] core::ops::Sub<Output = N>
        + [const] core::ops::Mul<Output = N>
        + [const] core::ops::Div<Output = N>
        + [const] core::marker::Destruct,
{
    fn quantity(&self, value: N) -> __QTY__<D, N> {
        __QTY__(value * const { size::<N, C>() } + const { zero::<N, C>() })
    }
    fn value(&self, quantity: __QTY__<D, N>) -> N {
        (quantity.0 - const { zero::<N, C>() }) / const { size::<N, C>() }
    }
}

/// A unit's size in `N`: a function, as a generic constant reads no field.
const fn size<N: const From<Exact>, const C: __CONSTS__>() -> N {
    N::from(C.size)
}

/// A unit's zero in `N`.
const fn zero<N: const From<Exact>, const C: __CONSTS__>() -> N {
    N::from(C.zero)
}

impl<const D: __DIM__, const C: __CONSTS__> core::fmt::Display for __UNIT__<D, C> {
    /// `{}` the symbol, `{:#}` the name; width, fill and alignment apply.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.pad(if f.alternate() { C.name } else { C.symbol })
    }
}
"#;

pub fn gen_lib_rs(system: &UnitSystem) -> String {
    let mut s = String::new();
    let n = Names::of(&system.system);
    let (dim, qty, dyn_, unit) = (n.dim, n.qty, n.dyn_, n.unit);

    s += "//! A unit system built on yadc, generated by yadc-codegen.\n";
    s += "//!\n";
    s += &format!("//! - `Kind` and `{dim}`: the kinds and the dimension (an exponent per base unit, and a kind).\n");
    s += &format!("//! - `{qty}<D, N>`: a quantity whose dimension is in its type, counted in `N`.\n");
    s += &format!(
        "//! - `{unit}<D, C>`: a unit, a type of its own (`<module>::units::Mile`), its\n//!   definition `C` a `{}` - an exact size and zero, a symbol and a name.\n//!   `Mile {{}}.quantity(3.0)` builds, `Mile {{}}.value(d)` reads, in any number type.\n",
        n.consts
    );
    s += &format!(
        "//! - `dynamic`: `{dyn_}` and `{}`, a quantity and a unit of a dimension decided\n//!   at run time; and each module's unit enums, a choice among its units.\n",
        n.dyn_unit
    );
    s += &format!(
        "//! - One module per family, each a Cargo feature: {}; and `dimensionless`,\n//!   always present.\n",
        system
            .system
            .family_order
            .iter()
            .map(|f| format!("`{f}`"))
            .collect::<Vec<_>>()
            .join(", ")
    );
    s += "#![cfg_attr(not(feature = \"std\"), no_std)]\n";
    s += "#![feature(generic_const_exprs)]\n";
    // A unit's definition is a struct with a `&'static str`, as a const
    // parameter.
    s += "#![feature(adt_const_params)]\n";
    s += "#![feature(unsized_const_params)]\n";
    s += "#![feature(const_trait_impl)]\n";
    s += "#![feature(const_cmp)]\n";
    s += "#![feature(const_default)]\n";
    s += "#![feature(derive_const)]\n";
    s += "#![feature(const_try)]\n";
    s += "#![feature(const_ops)]\n";
    s += "#![feature(const_convert)]\n";
    // The dynamic type's dynamickit impls drop their operands in `const` code.
    s += "#![feature(const_destruct)]\n";
    s += "#![allow(incomplete_features)]\n";
    s += "#![allow(clippy::excessive_precision)]\n";
    s += "#![allow(clippy::approx_constant)]\n";
    s += "\n";
    s += "use core::marker::ConstParamTy;\n";
    s += "\n";
    s += "pub use yadc::{Dimension, Quantity, Unit};\n";
    s += "\n";
    s += "pub mod exact;\n";
    s += "pub use exact::Exact;\n";
    s += "pub mod prefix;\n";
    s += "pub use prefix::*;\n";
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
    s += "/// the same exponents, with its sums as `#[kind(..)]` rules.";
    if system.dim.fields.iter().any(|f| f == "rad") {
        s += " Angles are not\n/// a kind but an exponent axis of the dimension (`rad`).";
    }
    s += "\n";
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
        if system.kind.absolute.contains(v) {
            items.push("absolute".into());
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


    s += "/// A quantity whose dimension is in its type, counted in `N`: every use\n";
    s += "/// says which (`Pressure<f64>`, `Pressure<f32>`). Its number is private to\n";
    s += &format!("/// this crate: a number becomes a quantity only through a unit (`{unit}`'s\n");
    s += "/// `Unit` impl is the one place), and a quantity value is a unit of its own\n";
    s += "/// kind (`q.quantity(n)` is `q * n`, `q.value(x)` is `x / q`).\n";
    s += "///\n";
    s += "/// It serializes transparently as just its number: the dimension `D` is the\n";
    s += "/// deserialization target's - schema-trusted, nothing on the wire.\n";
    s += "#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, yadc::Quantity)]\n";
    s += "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]\n";
    s += &format!("pub struct {qty}<const D: {dim}, N>(N);\n\n");
    s += &UNIT
        .replace("__QTY__", qty)
        .replace("__DIM__", dim)
        .replace("__UNIT__", unit)
        .replace("__CONSTS__", &n.consts);
    s += "\n";
    s += "pub mod dimensionless;\n";
    s += "pub mod dynamic;\n";
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

/// `dynamic/mod.rs`: the dynamic quantity and unit, and the modules of unit
/// enums (`dynamic::mechanical::LengthUnit`), each behind its family's
/// feature.
pub fn gen_dynamic_rs(system: &UnitSystem, unit_mods: &[(String, Option<String>)]) -> String {
    let n = Names::of(&system.system);
    // Which of the dynamic type's results may be missing: with an affine or
    // a log kind, a product too (a point or a level has none).
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
    let mut s = String::new();
    s += "//! The dynamic half: what is chosen at run time - a quantity's dimension\n";
    s += &format!("//! (`{}`), a unit of any dimension (`{}`), and a choice among a\n", n.dyn_, n.dyn_unit);
    s += "//! quantity's units (each module's unit enums, `dynamic::mechanical::LengthUnit`).\n\n";
    s += &format!(
        "use crate::{{{}, {}, {}, {}}};\nuse yadc::{{Dimension, Unit}};\n\n",
        n.dim, n.qty, n.unit, n.consts
    );
    for (m, feature) in unit_mods {
        if let Some(f) = feature {
            s += &format!("#[cfg(feature = \"{f}\")]\n");
        }
        s += &format!("pub mod {m};\n");
    }
    if !unit_mods.is_empty() {
        s += "\n";
    }
    s += &DYNAMIC
        .replace("__PRODUCTS_DOC__", &products_doc)
        .replace("__DYNUNIT__", &n.dyn_unit)
        .replace("__DYN__", n.dyn_)
        .replace("__QTY__", n.qty)
        .replace("__DIM__", n.dim)
        .replace("__UNIT__", n.unit)
        .replace("__CONSTS__", &n.consts);
    s
}

/// The dynamic types, as a user would write them with dynamickit: the
/// dynamic version of the static quantity (and of a bare `f64`, the same
/// value as a dimensionless quantity), every operator and method lifted from
/// the fields' own; and of the static unit.
const DYNAMIC: &str = r#"/// A quantity of any dimension, decided at run time: the dynamic version of
/// `__QTY__<D, f64>`, built with dynamickit. Converting (`From`) moves the number into
/// `value` and records `D` in `dim`; `TryFrom<__DYN__> for __QTY__<D, f64>` is the
/// checked way back (the error hands the original back). A bare `f64` is the
/// same value as a dimensionless quantity (`shared`). Numbers go through
/// units: a `__DYNUNIT__` reads and builds it directly.
///
/// Its arithmetic is every field's own: the dimension's operator on `dim`, the
/// number's on `value`. __PRODUCTS_DOC__ Values of one dimension are ordered by number,
/// `None` otherwise. A root or power with a run-time exponent is the
/// dimension's and the number's (`None` where the dimension has none).
/// Scaling by a number is the dimension's own too: its `* f64` and `/ f64`,
/// which `#[derive(Dimension)]` writes.
#[derive(Copy, Clone, PartialEq, Debug, dynamickit::Dynamic)]
#[dynamic(__QTY__<const D: __DIM__, f64> => { dim: D, value: .0 })]
#[dynamic(shared f64 => { dim: __DIM__::DIMENSIONLESS, value: self })]
#[dynamic(impl(
    Mul -> Option<Self>,
    Div -> Option<Self>,
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
    pub(crate) dim: __DIM__,
    pub(crate) value: f64,
}

/// A unit of any dimension, decided at run time: the dynamic version of
/// `__UNIT__<D, C>`, built with dynamickit - `From` records `D` and `C`,
/// `TryFrom<__DYNUNIT__> for __UNIT__<D, C>` is the checked way back. A
/// module's unit enum converts to one (`From<LengthUnit>`).
///
/// A unit of `Option<__DYN__>` counted in `Option<f64>`, the way dynamic
/// arithmetic results are: `quantity` builds a quantity of its dimension,
/// and `value` reads one of its dimension - `None` for another, or for
/// `None`. So a reading takes a calculation's result as it comes:
/// `unit.value(a * b)`.
#[derive(Copy, Clone, PartialEq, Debug, dynamickit::Dynamic)]
#[dynamic(__UNIT__<const D: __DIM__, const C: __CONSTS__> => { dim: D, consts: C })]
pub struct __DYNUNIT__ {
    pub dim: __DIM__,
    pub consts: __CONSTS__,
}

impl Unit<Option<__DYN__>, Option<f64>> for __DYNUNIT__ {
    fn quantity(&self, value: Option<f64>) -> Option<__DYN__> {
        let (size, zero) = (f64::from(self.consts.size), f64::from(self.consts.zero));
        Some(__DYN__ { dim: self.dim, value: value? * size + zero })
    }
    fn value(&self, quantity: Option<__DYN__>) -> Option<f64> {
        let q = quantity?;
        let (size, zero) = (f64::from(self.consts.size), f64::from(self.consts.zero));
        (q.dim == self.dim).then(|| (q.value - zero) / size)
    }
}

impl core::fmt::Display for __DYNUNIT__ {
    /// `{}` the symbol, `{:#}` the name; width, fill and alignment apply.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.pad(if f.alternate() { self.consts.name } else { self.consts.symbol })
    }
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
    for v in &kind.absolute {
        assert!(
            variants.contains(v),
            "[kind] absolute `{v}` is not one of the variants {variants:?}"
        );
        assert!(
            *v != kind.default && Some(v) != kind.affine.as_ref() && Some(v) != kind.log.as_ref(),
            "[kind] absolute `{v}` is the default, the affine or the log variant: those have \
             their own sums"
        );
    }
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

/// An expression with its number literals written exactly, as `Exact`s:
/// `273.15` is `Exact::dec(27315, -2)`, `1000.0` is `Exact::int(1000)`, and
/// `core::f64::consts::PI` is `Exact::PI`. Everything else is kept as written.
fn exact_literals(expr: &str) -> String {
    let expr = expr.replace("core::f64::consts::PI", "Exact::PI");
    let chars: Vec<char> = expr.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        let prev_ident = i > 0 && (chars[i - 1].is_alphanumeric() || chars[i - 1] == '_');
        let starts = c.is_ascii_digit() || (c == '.' && chars.get(i + 1).is_some_and(|d| d.is_ascii_digit()));
        if !starts || prev_ident {
            out.push(c);
            i += 1;
            continue;
        }
        let begin = i;
        while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '_' || chars[i] == '.') {
            i += 1;
        }
        if i < chars.len() && (chars[i] == 'e' || chars[i] == 'E') {
            i += 1;
            if i < chars.len() && (chars[i] == '-' || chars[i] == '+') {
                i += 1;
            }
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
        }
        let lit: String = chars[begin..i].iter().filter(|c| **c != '_').collect();
        out += &exact_literal(&lit);
    }
    out
}

/// One number literal as an `Exact`.
fn exact_literal(lit: &str) -> String {
    let (mantissa, exp) = match lit.find(['e', 'E']) {
        Some(k) => (&lit[..k], lit[k + 1..].parse::<i32>().expect("a number literal's exponent")),
        None => (lit, 0),
    };
    let (int, frac) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let mut digits = format!("{int}{frac}");
    let mut exp = exp - frac.len() as i32;
    let trimmed = digits.trim_start_matches('0');
    digits = if trimmed.is_empty() { "0".into() } else { trimmed.to_string() };
    while digits.len() > 1 && digits.ends_with('0') {
        digits.pop();
        exp += 1;
    }
    if digits == "0" {
        "Exact::ZERO".into()
    } else if exp >= 0 && digits.len() + exp as usize <= 30 {
        format!("Exact::int({digits}{})", "0".repeat(exp as usize))
    } else {
        format!("Exact::dec({digits}, {exp})")
    }
}

// --- Unit types -----------------------------------------------------------------

/// One quantity of a module, as its units see it.
struct QuantityRef<'a> {
    name: &'a str,
    kind: Option<&'a str>,
    dims: &'a BTreeMap<String, i64>,
    /// The units, each variant once.
    units: Vec<&'a UnitDef>,
}

impl<'a> QuantityRef<'a> {
    fn new(name: &'a str, kind: Option<&'a str>, dims: &'a BTreeMap<String, i64>, units: &'a [UnitDef]) -> Self {
        let mut seen = HashSet::new();
        let units = units.iter().filter(|u| seen.insert(u.variant.as_str())).collect();
        QuantityRef { name, kind, dims, units }
    }
}

/// Each unit's type name in a module, by quantity and variant: its `t`, or
/// its variant - prefixed with its quantity where two quantities' units
/// share one (`TemperatureIntervalKelvin`), so the result does not depend on
/// declaration order. A `t` is never prefixed.
fn unit_type_names<'a>(quantities: &[QuantityRef<'a>]) -> BTreeMap<(&'a str, &'a str), String> {
    let base = |u: &UnitDef| u.type_name.clone().unwrap_or_else(|| u.variant.clone());
    let mut count: BTreeMap<String, usize> = BTreeMap::new();
    for q in quantities {
        for u in &q.units {
            *count.entry(base(u)).or_default() += 1;
        }
    }
    let mut names = BTreeMap::new();
    let mut seen = HashSet::new();
    for q in quantities {
        for u in &q.units {
            let b = base(u);
            let name = if count[&b] > 1 && u.type_name.is_none() { format!("{}{b}", q.name) } else { b };
            assert!(seen.insert(name.clone()), "unit type name `{name}` is still ambiguous after prefixing: give one a `t`");
            names.insert((q.name, u.variant.as_str()), name);
        }
    }
    names
}

/// Whether a quantity is of dimension one, with the plain label.
fn is_plain(q: &QuantityRef<'_>) -> bool {
    q.kind.is_none() && q.dims.values().all(|e| *e == 0)
}

/// A quantity's dimension as a const argument: its constant (`LENGTH`), or
/// `{ Dim::DIMENSIONLESS }`.
fn dim_arg(q: &QuantityRef<'_>, dim: &str) -> String {
    if is_plain(q) { format!("{{ {dim}::DIMENSIONLESS }}") } else { screaming_snake(q.name) }
}

/// `KilowattHour` -> `KILOWATT_HOUR`: an underscore before every uppercase
/// letter that follows a lowercase letter or a digit, or that starts a new
/// word after an uppercase run.
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

/// `kilometres` -> `Kilometres`.
fn capitalized(s: &str) -> String {
    let mut c = s.chars();
    c.next().map_or_else(String::new, |f| f.to_uppercase().chain(c).collect())
}

fn is_one(m: &str) -> bool {
    m.trim().parse::<f64>() == Ok(1.0)
}

/// `pub mod units { .. }`: a type per unit of the module's quantities, its
/// size and zero written exactly.
fn gen_units_mod(quantities: &[QuantityRef<'_>], names: &BTreeMap<(&str, &str), String>, system: &UnitSystem) -> String {
    let n = Names::of(&system.system);
    let mut items = String::new();
    for q in quantities {
        let dim = dim_arg(q, n.dim);
        for u in &q.units {
            let size = exact_literals(&u.multiplier);
            let zero = u.zero.as_deref().map_or_else(|| "Exact::ZERO".to_string(), exact_literals);
            let in_coherent = match &u.zero {
                Some(z) => format!("`n · {} + {}`", u.multiplier.trim(), z.trim()),
                None => format!("`n · {}`", u.multiplier.trim()),
            };
            let label = if u.long.is_empty() { u.variant.clone() } else { capitalized(&u.long) };
            let symbol = u.symbol.replace('\n', " ");
            let symbol = if symbol.is_empty() || symbol.contains('`') { String::new() } else { format!(" (`{symbol}`)") };
            items += &format!(
                "\n    /// {label}{symbol}, a unit of `{}`: `n` of them is {in_coherent} coherent units.\n",
                q.name
            );
            items += &format!(
                "    pub type {} = {}<{dim}, {{ {} {{ size: {size}, zero: {zero}, symbol: \"{}\", name: \"{}\" }} }}>;\n",
                names[&(q.name, u.variant.as_str())],
                n.unit,
                n.consts,
                rust_str(&u.symbol),
                rust_str(&u.long)
            );
        }
    }
    if items.is_empty() {
        return String::new();
    }
    let mut s = String::new();
    s += "/// The units of this module's quantities: a type each, its size and zero\n";
    s += "/// in its quantity's coherent unit, exact (`Mile {}.quantity(3.0_f64)`).\n";
    s += "pub mod units {\n";
    let mut uses = vec!["Exact", n.unit, n.consts.as_str()];
    if items.contains("::DIMENSIONLESS") {
        uses.push(n.dim);
    }
    uses.sort_unstable();
    if quantities.iter().any(|q| !is_plain(q)) {
        s += "    use super::*;\n";
    }
    s += &format!("    use crate::{{{}}};\n", uses.join(", "));
    if items.contains("::DIMENSIONLESS") {
        s += "    use yadc::Dimension;\n";
    }
    s += &items;
    s += "}\n";
    s
}

// --- dimensionless.rs --------------------------------------------------------

pub fn gen_dimensionless_rs(system: &UnitSystem) -> Module {
    let n = Names::of(&system.system);
    let no_dims = BTreeMap::new();
    let quantities = [QuantityRef::new("Dimensionless", None, &no_dims, &system.dimensionless.units)];
    gen_module(
        &quantities,
        system,
        &format!("pub type Dimensionless<N> = {}<{{ {}::DIMENSIONLESS }}, N>;\n", n.qty, n.dim),
        "//! Plain numbers: ratios and their units.\n",
        "dimensionless",
    )
}

// --- <family>.rs -------------------------------------------------------------

pub fn gen_family_rs(family: &FamilyDef, system: &UnitSystem) -> Module {
    let n = Names::of(&system.system);
    let quantities: Vec<QuantityRef<'_>> = family
        .quantity
        .iter()
        .map(|q| QuantityRef::new(&q.name, q.kind.as_deref(), &q.dims, &q.units))
        .collect();
    let mut aliases = String::new();
    for q in &quantities {
        let value = dim_value(q.dims, q.kind, &system.dim.fields, n.dim);
        let arg = dim_arg(q, n.dim);
        if !is_plain(q) {
            let value = value.strip_prefix("{ ").and_then(|v| v.strip_suffix(" }")).unwrap_or(&value);
            aliases += &format!("\n/// The dimension of `{}`.\npub const {arg}: {} = {value};\n", q.name, n.dim);
        }
        aliases += &fmt_type_alias(q.name, &arg, n.qty);
    }
    gen_module(&quantities, system, &aliases, "", &family.name)
}

/// A family's two halves: its quantities and unit types (`<family>.rs`), and
/// its unit enums (`dynamic/<family>.rs`), each with its imports.
pub struct Module {
    pub quantities: String,
    pub units: String,
}

/// A module: its quantities' aliases and unit types, and - in the dynamic
/// half - its unit enums. `name` is the module's (`mechanical`).
fn gen_module(quantities: &[QuantityRef<'_>], system: &UnitSystem, aliases: &str, header: &str, name: &str) -> Module {
    let n = Names::of(&system.system);
    let names = unit_type_names(quantities);
    let units_mod = gen_units_mod(quantities, &names, system);

    // The quantities and unit types.
    let mut s = String::new();
    s += header;
    if !header.is_empty() {
        s += "\n";
    }
    let mut uses = vec![n.dim, n.qty];
    if aliases.contains("Kind::") {
        uses.push("Kind");
    }
    uses.sort_unstable();
    s += &format!("use crate::{{{}}};\n", uses.join(", "));
    if aliases.contains("::DIMENSIONLESS") {
        s += "use yadc::Dimension;\n";
    }
    s += "\n";
    s += aliases.trim_start_matches('\n');
    if !units_mod.is_empty() {
        s += "\n";
        s += &units_mod;
    }

    // The unit enums.
    let mut enums = String::new();
    for q in quantities {
        enums += &gen_unit_enum(q, &names, system);
    }
    let mut u = String::new();
    if !enums.is_empty() {
        u += &format!(
            "//! The units of `{name}`'s quantities, chosen at run time: for a menu, a\n//! config file, a parser.\n\n"
        );
        u += &format!("use crate::{name}::*;\n");
        u += &format!("use crate::{name}::units::*;\n");
        u += &format!("use crate::dynamic::{};\n", n.dyn_unit);
        u += "use crate::Exact;\n";
        u += "use core::marker::Destruct;\n";
        u += "use core::ops::{Add, Div, Mul, Sub};\n";
        u += &enums;
    }
    Module { quantities: s, units: u }
}

/// What a unit counts in: what the unit type's `Unit` impl asks of `N`.
const NUMBER: &str = "const From<Exact> + [const] Add<Output = N> + [const] Sub<Output = N> + [const] Mul<Output = N> + [const] Div<Output = N> + [const] Destruct";

/// `use`-free list of what `#[derive(..)]` a unit enum gets.
fn fmt_derive_attr(extra: &[String]) -> String {
    let mut all = vec!["yadc::Unit".to_string(), "dynamickit::Variants".to_string()];
    all.extend(extra.iter().cloned());
    format!("#[derive({})]\n", all.join(", "))
}

// --- Unit enums ----------------------------------------------------------------

/// `<Q>Unit`, the unit enum of quantity `q`: a variant per unit, naming its
/// type; and its conversion to the dynamic unit. Nothing for a quantity
/// without units.
fn gen_unit_enum(q: &QuantityRef<'_>, names: &BTreeMap<(&str, &str), String>, system: &UnitSystem) -> String {
    if q.units.is_empty() {
        return String::new();
    }
    let n = Names::of(&system.system);
    // The coherent unit is the `#[default]`: what a menu starts on. Failing
    // that, the first unit.
    let default = q.units.iter().position(|u| is_one(&u.multiplier) && u.zero.is_none()).unwrap_or(0);
    let enum_name = format!("{}Unit", q.name);
    let mut variants = String::new();
    let mut arms = String::new();
    for (i, unit) in q.units.iter().enumerate() {
        let ty = &names[&(q.name, unit.variant.as_str())];
        if i == default {
            variants += "    #[default]\n";
        }
        variants += &format!("    #[unit({ty})]\n    {},\n", unit.variant);
        arms += &format!("            {enum_name}::{} => {ty} {{}}.into(),\n", unit.variant);
    }
    let mut s = String::new();
    s += &format!("\n/// The units of `{}`, in any number type they count in.\n", q.name);
    s += "#[derive_const(Default)]\n";
    s += &fmt_derive_attr(&system.system.unit_derives);
    s += &format!("#[unit(of = {}<N>, number = N, for<N: {NUMBER}>)]\n", q.name);
    s += &format!("pub enum {enum_name} {{\n{variants}}}\n\n");
    s += &format!(
        "impl From<{enum_name}> for {} {{\n    fn from(unit: {enum_name}) -> Self {{\n        match unit {{\n{arms}        }}\n    }}\n}}\n",
        n.dyn_unit
    );
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

fn fmt_type_alias(qty_name: &str, dim_value: &str, qty_ty: &str) -> String {
    format!("pub type {qty_name}<N> = {qty_ty}<{dim_value}, N>;\n")
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
unit_type = "SIUnit"

[kind]
variants = ["Linear", "Affine", "Log", "Rank"]
default = "Linear"
affine = "Affine"
log = "Log"
rules = { Rank = ["add(Rank) = invalid"] }

[dim]
fields = ["m", "s", "K"]

[dimensionless]
units = [{v = "Ratio", m = "1.0", s = "", l = ""}]
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
units = [
    {v = "Kelvin", m = "1.0", s = "K", l = "kelvin", t = "KelvinScale"},
    {v = "Celsius", m = "1.0", z = "273.15", s = "°C", l = "degrees Celsius"},
]

[[family.quantity]]
name = "TemperatureInterval"
dims = {K = 1}
units = [
    {v = "Kelvin", m = "1.0", s = "K", l = "kelvin"},
    {v = "Fahrenheit", m = "5.0 / 9.0", s = "°F", l = "degrees Fahrenheit"},
]

[[family.quantity]]
name = "Speed"
dims = {m = 1, s = -1}
units = [
    {v = "MetrePerSecond", m = "1.0", s = "m/s", l = "metres per second"},
    {v = "KilometrePerHour", m = "1000.0 / 3600.0", s = "km/h", l = "kilometres per hour"},
]
"#;

    /// Both halves of the test family: its quantities, then its unit enums.
    fn family() -> String {
        let sys = system(THERMAL);
        let m = gen_family_rs(&sys.family[0], &sys);
        m.quantities + &m.units
    }

    /// A unit is a type: its dimension, and its size and zero written exactly.
    #[test]
    fn units_are_types() {
        let out = family();
        assert!(out.contains("/// The dimension of `Speed`.\npub const SPEED: Dim = Dim { m: 1, s: -1, ..Dim::DIMENSIONLESS };\npub type Speed<N> = Qty<SPEED, N>;\n"));
        assert!(out.contains(
            "    pub type KilometrePerHour = SIUnit<SPEED, { SIUnitConsts { size: Exact::int(1000) / Exact::int(3600), zero: Exact::ZERO, symbol: \"km/h\", name: \"kilometres per hour\" } }>;"
        ));
        assert!(out.contains(
            "    pub type Celsius = SIUnit<THERMODYNAMIC_TEMPERATURE, { SIUnitConsts { size: Exact::int(1), zero: Exact::dec(27315, -2), symbol: \"°C\", name: \"degrees Celsius\" } }>;"
        ));
    }

    /// Each quantity's enum names its unit types, the coherent one the default.
    #[test]
    fn unit_enums_name_their_types() {
        let out = family();
        assert!(out.contains("pub enum SpeedUnit {\n    #[default]\n    #[unit(MetrePerSecond)]\n    MetrePerSecond,\n    #[unit(KilometrePerHour)]\n    KilometrePerHour,\n}"));
        assert!(out.contains("#[unit(of = Speed<N>, number = N, for<N: const From<Exact>"));
        assert!(out.contains("            SpeedUnit::KilometrePerHour => KilometrePerHour {}.into(),\n"));
    }

    /// A type name two quantities' units share is prefixed with the quantity
    /// for each; a `t` is kept, and a unique name stays bare.
    #[test]
    fn shared_type_names_are_prefixed() {
        let out = family();
        assert!(out.contains("    pub type KelvinScale = "));
        assert!(out.contains("    pub type Kelvin = "));
        assert!(out.contains("    #[unit(KelvinScale)]\n    Kelvin,\n"));
        let sys = system(
            r#"
[[family]]
name = "thermal"
[[family.quantity]]
name = "Time"
dims = {s = 1}
units = [{v = "Minute", m = "60.0", s = "min", l = "minutes"}]
[[family.quantity]]
name = "Angle"
dims = {m = 1}
units = [{v = "Minute", m = "2.9e-4", s = "'", l = "minutes of arc"}]
"#,
        );
        let m = gen_family_rs(&sys.family[0], &sys);
        assert!(m.quantities.contains("    pub type TimeMinute = "));
        assert!(m.quantities.contains("    pub type AngleMinute = "));
        assert!(m.units.contains("    #[unit(TimeMinute)]\n    Minute,\n"));
    }

    /// The unit type and its one `Unit` impl, and the dynamic unit.
    #[test]
    fn generated_lib() {
        let sys = system(THERMAL);
        let out = gen_lib_rs(&sys);
        assert!(out.contains("pub struct SIUnitConsts {"));
        assert!(out.contains("pub struct SIUnit<const D: Dim, const C: SIUnitConsts>;"));
        assert!(out.contains("const impl<const D: Dim, const C: SIUnitConsts, N> Unit<Qty<D, N>, N> for SIUnit<D, C>"));
        let dynamic = gen_dynamic_rs(&sys, &[]);
        assert!(dynamic.contains("#[dynamic(SIUnit<const D: Dim, const C: SIUnitConsts> => { dim: D, consts: C })]"));
        assert!(dynamic.contains("impl Unit<Option<DynQty>, Option<f64>> for DynamicSIUnit {"));
    }

    /// Without an affine or a logarithmic kind no variant is marked; the
    /// products stay partial, as for any dimension with a kind field.
    #[test]
    fn labels_alone_mark_no_variant() {
        let sys: UnitSystem = toml::from_str(&BASE.replace("affine = \"Affine\"\nlog = \"Log\"\n", "")).unwrap();
        let out = gen_lib_rs(&sys);
        assert!(out.contains("    #[kind]\n    pub kind: Kind,\n"));
        assert!(!out.contains("#[kind(affine)]"));
        let dynamic = gen_dynamic_rs(&sys, &[]);
        assert!(dynamic.contains("    Mul -> Option<Self>,\n    Div -> Option<Self>,\n"));
    }

    #[test]
    #[should_panic(expected = "`div` is not `add` or `sub` - kinds are labels")]
    fn mul_and_div_rules_are_rejected() {
        let sys: UnitSystem = toml::from_str(&BASE.replace("add(Rank) = invalid", "div(Rank) = Rank")).unwrap();
        gen_lib_rs(&sys);
    }

    /// A rule is a sum; which variant is affine or logarithmic is its own key.
    #[test]
    #[should_panic(expected = "is not `op(Operand) = Result`")]
    fn bare_rules_are_rejected() {
        let sys: UnitSystem = toml::from_str(&BASE.replace("add(Rank) = invalid", "Affine")).unwrap();
        gen_lib_rs(&sys);
    }

    #[test]
    #[should_panic(expected = "[kind] affine `Point` is not one of the variants")]
    fn named_kinds_are_variants() {
        let sys: UnitSystem = toml::from_str(&BASE.replace("affine = \"Affine\"", "affine = \"Point\"")).unwrap();
        gen_lib_rs(&sys);
    }

    /// A unit is a size and a zero: any other description is an error, not
    /// a field quietly left out.
    #[test]
    fn unknown_unit_fields_are_rejected() {
        let parsed: Result<UnitSystem, _> = toml::from_str(&format!(
            "{BASE}{}",
            r#"
[[family]]
name = "thermal"
[[family.quantity]]
name = "Length"
dims = {m = 1}
units = [{v = "Foot", m = "0.3048", s = "ft", l = "feet", by = "FOOT"}]
"#
        ));
        assert!(parsed.unwrap_err().to_string().contains("unknown field `by`"));
    }

    #[test]
    fn exact_literals_are_exact() {
        assert_eq!(exact_literals("273.15"), "Exact::dec(27315, -2)");
        assert_eq!(exact_literals("-40587.0 * 86400.0"), "-Exact::int(40587) * Exact::int(86400)");
        assert_eq!(exact_literals("1.0 / core::f64::consts::PI"), "Exact::int(1) / Exact::PI");
    }
}
