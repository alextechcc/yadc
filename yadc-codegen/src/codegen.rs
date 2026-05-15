use crate::model::*;
use std::collections::{BTreeMap, HashSet};
use std::path::Path;

// ─── Public entry point ───────────────────────────────────────────────────────

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

// ─── lib.rs ───────────────────────────────────────────────────────────────────

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

    s += "pub use yadc::{Dimension, DynamicQuantity, Format, StaticQuantity, Unit};\n";
    s += "\n";

    // Kind enum — with the block comment from the actual lib.rs
    s += "// The Kind enum tags non-numeric \"kind\" of a quantity. It's mutually exclusive\n";
    s += "// with the seven SI numeric axes, and lets the type system distinguish quantities\n";
    s += "// that share the same base dimension (Angle vs Dimensionless, T vs \u{0394}T, etc.).\n";
    s += "//\n";
    s += "// `#[derive(Kind)]` generates ConstParamTy_, ZERO/INVALID consts, impl const\n";
    s += "// PartialEq/Eq, and impl const Mul/Div/Add/Sub.\n";
    s += "// Variant names match the standard roles; rename with `#[kind(<Role>)]` to use\n";
    s += "// custom variant identifiers.\n";
    s += "#[derive(Copy, Clone, Debug, yadc::Kind)]\n";
    s += "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]\n";
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
    s += "    derive(yadc::DimensionSerialize, yadc::DimensionDeserialize)\n";
    s += ")]\n";
    s += "pub struct Dim {\n";
    for field in &system.dim.fields {
        s += &format!("    pub {field}: i8,\n");
    }
    s += "    #[kind]\n";
    s += "    pub kind: Kind,\n";
    s += "}\n\n";

    // Qty — with comment
    s += "// Qty serializes transparently as just `f64` (its only field). The dimension\n";
    s += "// `D` is determined by the deserialization target \u{2014} schema-trusted, no dim\n";
    s += "// metadata on the wire. If you need the dim recorded too, hand-roll a custom\n";
    s += "// impl on `Qty` (the inner f64 is reachable from this module).\n";
    s += "#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, StaticQuantity)]\n";
    s += "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]\n";
    s += "pub struct Qty<const D: Dim>(f64);\n\n";

    // DynQty
    s += "#[derive(Copy, Clone, PartialEq, Debug, DynamicQuantity)]\n";
    s += "#[static_quantity(Qty)]\n";
    s += "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]\n";
    s += "pub struct DynQty {\n";
    s += "    value: f64,\n";
    s += "    dim: Dim,\n";
    s += "}\n\n";

    s += "pub mod dimensionless;\n";

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

// ─── dimensionless.rs ────────────────────────────────────────────────────────

pub fn gen_dimensionless_rs(system: &UnitSystem) -> String {
    let mut s = String::new();
    s += "use crate::{Dim, Qty};\n";
    s += "use yadc::{Dimension, Unit};\n\n";

    s += "pub type Dimensionless = Qty<{ Dim { ..Dim::ZERO } }>;\n\n";

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
        s += &fmt_unit_attr(&unit.multiplier, &unit.symbol, &unit.long, unit.offset);
        s += &format!("    {},\n", unit.variant);
    }
    s += "}\n";
    s
}

// ─── <family>.rs ─────────────────────────────────────────────────────────────

pub fn gen_family_rs(family: &FamilyDef, system: &UnitSystem) -> String {
    let uses_kind = family.quantity.iter().any(|q| q.kind.is_some());

    let mut s = String::new();
    if uses_kind {
        s += "use crate::{Dim, Kind, Qty};\n";
    } else {
        s += "use crate::{Dim, Qty};\n";
    }
    s += "use yadc::{Dimension, Unit};\n";

    for qty in &family.quantity {
        s += "\n";
        s += &fmt_type_alias(
            &qty.name,
            &qty.dims,
            qty.kind.as_deref(),
            &system.dim.fields,
        );
        s += "\n";
        s += &fmt_derive_attr(&system.system.unit_derives);
        s += &format!("#[dim({})]\n", qty.name);
        if let Some(dd) = &qty.dim_delta {
            s += &format!("#[dim_delta({dd})]\n");
        }
        s += &format!("pub enum {}Unit {{\n", qty.name);
        let mut seen: HashSet<String> = HashSet::new();
        for unit in &qty.units {
            if seen.contains(&unit.variant) {
                continue;
            }
            seen.insert(unit.variant.clone());
            s += &fmt_unit_attr(&unit.multiplier, &unit.symbol, &unit.long, unit.offset);
            s += &format!("    {},\n", unit.variant);
        }
        s += "}\n";
    }

    s
}

// ─── Cargo.toml ──────────────────────────────────────────────────────────────

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

// ─── Dim type alias formatting ────────────────────────────────────────────────

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
) -> String {
    let parts = dim_field_exprs(dims, kind, fields);

    if is_multiline(dims, kind) {
        let mut s = format!("pub type {qty_name} = Qty<\n    {{\n        Dim {{\n");
        for part in &parts {
            s += &format!("            {part},\n");
        }
        s += "            ..Dim::ZERO\n";
        s += "        }\n    },\n>;\n";
        s
    } else {
        // Single-line: 0 or 1 non-zero numeric field, no kind
        let inner = if parts.is_empty() {
            "..Dim::ZERO".to_string()
        } else {
            format!("{}, ..Dim::ZERO", parts.join(", "))
        };
        format!("pub type {qty_name} = Qty<{{ Dim {{ {inner} }} }}>;\n")
    }
}

// ─── Derive attribute formatting ─────────────────────────────────────────────

fn fmt_derive_attr(extra: &[String]) -> String {
    if extra.is_empty() {
        "#[derive(Unit)]\n".to_string()
    } else {
        format!("#[derive(Unit, {})]\n", extra.join(", "))
    }
}

// ─── Unit attribute formatting ────────────────────────────────────────────────

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

/// Format a `#[unit(...)]` attribute with its trailing newline.
/// Uses a single line when ≤ 80 chars; falls back to the 4-arg multi-line form.
fn fmt_unit_attr(mult: &str, sym: &str, long: &str, offset: Option<f64>) -> String {
    let sym_r = rust_str(sym);
    let long_r = rust_str(long);

    let single = match offset {
        Some(off) => format!(
            "    #[unit({mult}, \"{sym_r}\", \"{long_r}\", {})]",
            fmt_f64(off)
        ),
        None => format!("    #[unit({mult}, \"{sym_r}\", \"{long_r}\")]"),
    };

    if single.chars().count() <= 83 {
        return format!("{single}\n");
    }

    // Multi-line form
    let mut s = format!("    #[unit(\n        {mult},\n        \"{sym_r}\",\n        \"{long_r}\"");
    if let Some(off) = offset {
        s += &format!(",\n        {}", fmt_f64(off));
    }
    s += "\n    )]\n";
    s
}
