use crate::model::*;
use std::collections::{BTreeMap, HashSet};
use std::path::Path;

// --- Type-name helpers --------------------------------------------------------

fn dim_ty(sys: &SystemConfig) -> &str  { sys.dim_type.as_deref().unwrap_or("Dim") }
fn qty_ty(sys: &SystemConfig) -> &str  { sys.qty_type.as_deref().unwrap_or("Qty") }
fn dyn_ty(sys: &SystemConfig) -> &str  { sys.dynqty_type.as_deref().unwrap_or("DynQty") }

// --- Public entry point -------------------------------------------------------

pub fn write_crate(system: &UnitSystem, output_dir: &Path) -> std::io::Result<()> {
    let src = output_dir.join("src");
    std::fs::create_dir_all(&src)?;

    std::fs::write(src.join("lib.rs"), gen_lib_rs(system))?;
    std::fs::write(src.join("dimensionless.rs"), gen_dimensionless_rs(system))?;
    std::fs::write(output_dir.join("Cargo.toml"), gen_cargo_toml(system))?;

    for fam_name in &system.system.family_order {
        let Some(fam) = system.family.iter().find(|f| &f.name == fam_name) else {
            continue;
        };
        if fam.quantity.is_empty() {
            continue;
        }
        std::fs::write(
            src.join(format!("{fam_name}.rs")),
            gen_family_rs(fam, system),
        )?;
    }

    Ok(())
}

// --- lib.rs -------------------------------------------------------------------

pub fn gen_lib_rs(system: &UnitSystem) -> String {
    let mut s = String::new();

    // Feature / lint flags
    s += "#![cfg_attr(not(feature = \"std\"), no_std)]\n";
    s += "#![feature(generic_const_exprs)]\n";
    s += "#![feature(adt_const_params)]\n";
    s += "#![feature(unsized_const_params)]\n";
    s += "#![feature(const_trait_impl)]\n";
    s += "#![feature(const_ops)]\n";
    s += "#![feature(const_cmp)]\n";
    s += "#![feature(structural_match)]\n";
    s += "#![allow(incomplete_features)]\n";
    s += "#![allow(clippy::excessive_precision)]\n";
    s += "#![allow(clippy::approx_constant)]\n";
    s += "\n";

    s += "pub use yadc::{Dimension, DynamicQuantity, Format, Quantity, StaticQuantity, Unit};\n";
    s += "\n";

    // Kind enum - with the block comment from the actual lib.rs
    s += "// The Kind enum tags non-numeric \"kind\" of a quantity. It's mutually exclusive\n";
    s += "// with the seven SI numeric axes, and lets the type system distinguish quantities\n";
    s += "// that share the same base dimension (Angle vs Dimensionless, T vs delta-T, etc.).\n";
    s += "//\n";
    s += "// `#[derive(Kind)]` generates ConstParamTy_, ZERO/INVALID consts, impl const\n";
    s += "// PartialEq/Eq, and impl const Mul/Div/Add/Sub.\n";
    s += "// Variant names match the standard roles; rename with `#[kind(<Role>)]` to use\n";
    s += "// custom variant identifiers.\n";
    s += "#[derive(Copy, Clone, Debug, yadc::Kind)]\n";
    s += "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize, yadc::UcumKind))]\n";
    s += "pub enum Kind {\n";
    for v in &system.kind.variants {
        s += &format!("    {v},\n");
    }
    s += "}\n\n";

    // Dim struct
    s += "#[derive(Copy, Clone, Debug, Dimension)]\n";
    s += "#[allow(non_snake_case)]\n";
    // The cfg_attr line is long enough to warrant multi-line formatting.
    s += "#[cfg_attr(\n";
    s += "    feature = \"serde\",\n";
    s += "    derive(yadc::UcumSerialize, yadc::UcumDeserialize)\n";
    s += ")]\n";
    let dim = dim_ty(&system.system);
    let qty = qty_ty(&system.system);
    let dyn_ = dyn_ty(&system.system);

    s += &format!("pub struct {dim} {{\n");
    for field in &system.dim.fields {
        s += &format!("    pub {field}: i8,\n");
    }
    s += "    #[kind]\n";
    s += "    pub kind: Kind,\n";
    s += "}\n\n";

    s += &format!("// {qty} serializes transparently as just `f64` (its only field). The dimension\n");
    s += "// `D` is determined by the deserialization target - schema-trusted, no dim\n";
    s += &format!("// metadata on the wire. If you need the dim recorded too, hand-roll a custom\n");
    s += &format!("// impl on `{qty}` (the inner f64 is reachable from this module).\n");
    s += "#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, StaticQuantity)]\n";
    s += "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]\n";
    s += &format!("pub struct {qty}<const D: {dim}>(f64);\n\n");

    s += "#[derive(Copy, Clone, PartialEq, Debug, DynamicQuantity)]\n";
    s += &format!("#[static_quantity({qty})]\n");
    s += "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]\n";
    s += &format!("pub struct {dyn_} {{\n");
    s += "    value: f64,\n";
    s += &format!("    dim: {dim},\n");
    s += "}\n\n";

    s += "pub mod dimensionless;\n";

    // Hand-written, non-generated modules (e.g. uom_bridge). Declared here but
    // their files are never written by the codegen, so regeneration leaves them
    // intact.
    if !system.system.extra_modules.is_empty() {
        s += "\n";
        for m in &system.system.extra_modules {
            if let Some(feat) = &m.feature {
                s += &format!("#[cfg(feature = \"{feat}\")]\n");
            }
            s += &format!("pub mod {};\n", m.name);
        }
    }

    // Module declarations in alphabetical order (matches hand-written yadc-si)
    s += "\n";
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

// --- dimensionless.rs --------------------------------------------------------

pub fn gen_dimensionless_rs(system: &UnitSystem) -> String {
    let dim = dim_ty(&system.system);
    let qty = qty_ty(&system.system);

    let mut s = String::new();
    s += &format!("use crate::{{{dim}, {qty}}};\n");
    s += "use yadc::{Dimension, Unit};\n\n";

    s += &format!("pub type Dimensionless = {qty}<{{ {dim} {{ ..{dim}::ZERO }} }}>;\n\n");

    s += "// Variants mirror uom's `ratio.rs` quantity.\n";
    s += &fmt_derive_attr(&system.system.unit_derives);
    s += "#[dim(Dimensionless)]\n";
    s += "pub enum DimensionlessUnit {\n";
    let mut seen: HashSet<String> = HashSet::new();
    for unit in &system.dimensionless.units {
        if seen.contains(&unit.variant) {
            continue;
        }
        seen.insert(unit.variant.clone());
        s += &fmt_unit_attr(unit);
        s += &format!("    {},\n", unit.variant);
    }
    s += "}\n";
    s += &gen_zero_dim_conversions(system);
    s
}

/// Emit `From` impls between each zero-dim non-Linear quantity and `Dimensionless`.
///
/// Conversions go via unit division/multiplication (no internal field access).
/// Only `<-> Dimensionless` is generated: that is the one reinterpretation with a
/// clear meaning (drop/attach the kind tag on a bare scalar). Cross-kind pairs
/// (e.g. `Angle <-> SolidAngle`) are deliberately *not* emitted -- reinterpreting
/// one non-Linear kind as an unrelated one is almost always a mistake, and a user
/// who really wants it can compose two `Dimensionless` hops explicitly.
fn gen_zero_dim_conversions(system: &UnitSystem) -> String {
    // Every quantity with no numeric exponents and a non-default (non-Linear) kind.
    let zero_dim: Vec<(&str, &str)> = system
        .family
        .iter()
        .flat_map(|f| {
            f.quantity
                .iter()
                .filter(|q| q.dims.is_empty() && q.kind.is_some())
                .map(move |q| (f.name.as_str(), q.name.as_str()))
        })
        .collect();

    let mut s = String::new();
    for &(fam, qty) in &zero_dim {
        let cfg = format!("#[cfg(feature = \"{fam}\")]");
        let path = format!("crate::{fam}::{qty}");
        let unit = format!("<{path}>::from_canonical(1.0)");

        s += &format!("\n{cfg}\n");
        s += &format!("impl ::core::convert::From<{path}> for Dimensionless {{\n");
        s += &format!("    fn from(q: {path}) -> Self {{ q / {unit} }}\n");
        s += "}\n";

        s += &format!("\n{cfg}\n");
        s += &format!("impl ::core::convert::From<Dimensionless> for {path} {{\n");
        s += &format!("    fn from(q: Dimensionless) -> Self {{ q * {unit} }}\n");
        s += "}\n";
    }
    s
}

// --- <family>.rs -------------------------------------------------------------

pub fn gen_family_rs(family: &FamilyDef, system: &UnitSystem) -> String {
    let uses_kind = family.quantity.iter().any(|q| q.kind.is_some());
    let dim = dim_ty(&system.system);
    let qty = qty_ty(&system.system);

    let mut s = String::new();
    if uses_kind {
        s += &format!("use crate::{{{dim}, Kind, {qty}}};\n");
    } else {
        s += &format!("use crate::{{{dim}, {qty}}};\n");
    }
    s += "use yadc::{Dimension, Unit};\n";

    for qty_def in &family.quantity {
        s += "\n";
        s += &fmt_type_alias(
            &qty_def.name,
            &qty_def.dims,
            qty_def.kind.as_deref(),
            &system.dim.fields,
            dim,
            qty,
        );
        s += "\n";
        s += &fmt_derive_attr(&system.system.unit_derives);
        s += &format!("#[dim({})]\n", qty_def.name);
        s += &format!("pub enum {}Unit {{\n", qty_def.name);
        let mut seen: HashSet<String> = HashSet::new();
        for unit in &qty_def.units {
            if seen.contains(&unit.variant) {
                continue;
            }
            seen.insert(unit.variant.clone());
            s += &fmt_unit_attr(unit);
            s += &format!("    {},\n", unit.variant);
        }
        s += "}\n";
    }

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
    s += "\n[dependencies]\n";
    s += "yadc = { path = \"../yadc\" }\n";
    s += "serde = { version = \"1\", features = [\"derive\"], optional = true }\n";
    s += "\n[features]\n";
    s += "default = [\n";
    s += "    \"std\",\n";
    for f in &nonempty {
        s += &format!("    \"{f}\",\n");
    }
    s += "]\n";
    s += "std = []\n";
    for f in &nonempty {
        s += &format!("{f} = []\n");
    }
    s += "serde = [\"dep:serde\"]\n";
    s
}

// --- Dim type alias formatting ------------------------------------------------

/// Returns true when the dim struct literal needs multi-line layout.
/// Rule: multi-line if a `kind` override is present OR if 2+ numeric fields are non-zero.
fn is_multiline(dims: &BTreeMap<String, i64>, kind: Option<&str>) -> bool {
    let nonzero = dims.values().filter(|&&v| v != 0).count();
    kind.is_some() || nonzero >= 2
}

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
    let parts = dim_field_exprs(dims, kind, fields);

    if is_multiline(dims, kind) {
        let mut s = format!("pub type {qty_name} = {qty_ty}<\n    {{\n        {dim_ty} {{\n");
        for part in &parts {
            s += &format!("            {part},\n");
        }
        s += &format!("            ..{dim_ty}::ZERO\n");
        s += "        }\n    },\n>;\n";
        s
    } else {
        let inner = if parts.is_empty() {
            format!("..{dim_ty}::ZERO")
        } else {
            format!("{}, ..{dim_ty}::ZERO", parts.join(", "))
        };
        format!("pub type {qty_name} = {qty_ty}<{{ {dim_ty} {{ {inner} }} }}>;\n")
    }
}

// --- Derive attribute formatting ---------------------------------------------

fn fmt_derive_attr(extra: &[String]) -> String {
    if extra.is_empty() {
        "#[derive(Unit)]\n".to_string()
    } else {
        format!("#[derive(Unit, {})]\n", extra.join(", "))
    }
}

// --- Unit attribute formatting ------------------------------------------------

/// Escape a string for a Rust string literal (escapes `\` and `"`).
fn rust_str(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn fmt_f64(v: f64) -> String {
    let s = format!("{v}");
    // Rust infers f64 from context in attribute positions, but we always want
    // an explicit decimal point for clarity.
    if s.contains('.')
        || s.contains('e')
        || s.contains('E')
        || s.contains("inf")
        || s.contains("nan")
    {
        s
    } else {
        format!("{s}.0")
    }
}

/// Format a unit conversion attribute with its trailing newline.
/// Uses a single line when short enough; falls back to a multi-line form.
fn fmt_unit_attr(unit: &UnitDef) -> String {
    let mult = &unit.multiplier;
    let sym_r = rust_str(&unit.symbol);
    let long_r = rust_str(&unit.long);

    // Nonlinear units. `m` is the SI `reference` (omitted when 1.0); the TOML
    // scale function names a `#[log]` base (negating coeff for `neglog10`) or
    // the `#[tan]` family.
    if let Some(scale) = &unit.scale {
        let coeff = unit.coeff.as_deref().unwrap_or("1.0");
        let reference = if mult == "1.0" {
            String::new()
        } else {
            format!(", reference = {mult}")
        };
        return match scale.as_str() {
            "tan" => format!("    #[tan(\"{sym_r}\", \"{long_r}\", {coeff}{reference})]\n"),
            "pow10" | "exp" | "pow2" | "neglog10" => {
                let (base, coeff) = match scale.as_str() {
                    "pow10" => ("10.0".to_string(), coeff.to_string()),
                    "exp" => ("core::f64::consts::E".to_string(), coeff.to_string()),
                    "pow2" => ("2.0".to_string(), coeff.to_string()),
                    _ => ("10.0".to_string(), format!("-({coeff})")), // neglog10
                };
                format!("    #[log(\"{sym_r}\", \"{long_r}\", {base}, {coeff}{reference})]\n")
            }
            other => panic!("unknown scale function {other:?} in unit `{}`", unit.variant),
        };
    }

    let single = match unit.offset {
        Some(off) => format!(
            "    #[affine(\"{sym_r}\", \"{long_r}\", {mult}, {})]",
            fmt_f64(off)
        ),
        None => format!("    #[linear(\"{sym_r}\", \"{long_r}\", {mult})]"),
    };

    if single.chars().count() <= 83 {
        return format!("{single}\n");
    }

    // Multi-line form
    let attr = if unit.offset.is_some() {
        "affine"
    } else {
        "linear"
    };
    let mut s =
        format!("    #[{attr}(\n        \"{sym_r}\",\n        \"{long_r}\",\n        {mult}");
    if let Some(off) = unit.offset {
        s += &format!(",\n        {}", fmt_f64(off));
    }
    s += "\n    )]\n";
    s
}
