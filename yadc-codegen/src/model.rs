use serde::Deserialize;
use std::collections::BTreeMap;

/// Root of a `.toml` unit system definition.
#[derive(Deserialize, Debug)]
pub struct UnitSystem {
    pub system: SystemConfig,
    pub kind: KindConfig,
    pub dim: DimConfig,
    /// Units to place in the always-present `Dimensionless` type alias.
    pub dimensionless: DimensionlessDef,
    /// One entry per quantity family (absent when `[ucum]` drives generation).
    #[serde(default)]
    pub family: Vec<FamilyDef>,
    /// When present, ignore `[[family]]` and generate families from a UCUM XML source.
    #[serde(default)]
    pub ucum: Option<UcumSection>,
}

#[derive(Deserialize, Debug)]
pub struct SystemConfig {
    /// The output crate name, e.g. `"yadc-si"`.
    pub crate_name: String,
    /// Canonical family order used for Cargo.toml features and lib.rs mod declarations.
    pub family_order: Vec<String>,
    /// Extra traits to derive on every unit enum, e.g. `["Debug", "Clone", "Hash"]`.
    #[serde(default)]
    pub unit_derives: Vec<String>,
    /// Name for the generated dimension struct. Defaults to `"Dim"`.
    #[serde(default)]
    pub dim_type: Option<String>,
    /// Name for the generated static-quantity tuple struct. Defaults to `"Qty"`.
    #[serde(default)]
    pub qty_type: Option<String>,
    /// Name for the generated dynamic-quantity struct. Defaults to `"DynQty"`.
    #[serde(default)]
    pub dynqty_type: Option<String>,
    /// Name for the generated unit type, `<name><D, C>`; its const parameter is
    /// `<name>Consts` and its dynamic version `Dynamic<name>`. Defaults to
    /// `"QtyUnit"`.
    #[serde(default)]
    pub unit_type: Option<String>,
    /// Hand-written modules that live in the crate's `src/` but are NOT generated
    /// (e.g. a `uom_bridge` whose orphan-rule impls must sit beside the quantity
    /// type). The codegen emits a `pub mod` declaration for each in `lib.rs` but
    /// never writes or overwrites their files, so regeneration preserves them.
    #[serde(default)]
    pub extra_modules: Vec<ExtraModule>,
}

/// A hand-written module declared by `lib.rs` but not generated. See
/// [`SystemConfig::extra_modules`].
#[derive(Deserialize, Debug)]
pub struct ExtraModule {
    /// Module name (the `src/<name>.rs` file the user maintains by hand).
    pub name: String,
    /// Optional Cargo feature gating the `pub mod` declaration.
    #[serde(default)]
    pub feature: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct KindConfig {
    /// Variant names for the `Kind` label enum, in declaration order.
    pub variants: Vec<String>,
    /// The plain label: the variant marked `#[default]` (dimension one's
    /// label, what `*` and `/` leave beside anything but a plain number) -
    /// the kind of an ordinary, linear amount.
    pub default: String,
    /// The affine kind: the variant whose quantities are points - two differ
    /// by a plain amount, a plain amount moves one, and one has no sum with
    /// another, no scaling and no products. Marked on its variant,
    /// `#[kind(affine)]`.
    #[serde(default)]
    pub affine: Option<String>,
    /// The logarithmic kind: the variant whose quantities are logarithms -
    /// levels and gains, whose `+` and `-` add and subtract the exponents.
    /// Marked on its variant, `#[kind(log)]`.
    #[serde(default)]
    pub log: Option<String>,
    /// Labels of amounts with a true zero, read on scales with other zeros (a
    /// temperature): two differ by a plain amount, and a plain amount moves
    /// one. Marked on the variant, `#[kind(absolute)]`.
    #[serde(default)]
    pub absolute: Vec<String>,
    /// The sums of a label of the system's own, emitted verbatim as that
    /// variant's `#[kind(..)]`: `Rank = ["add(Rank) = invalid"]`. Labels: `*`
    /// and `/` are fixed, so a rule is an `add(..)` or `sub(..)` entry, its
    /// result a variant or `invalid` (`None`).
    #[serde(default)]
    pub rules: BTreeMap<String, Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct DimConfig {
    /// Base dimension field names for the `Dim` struct, in declaration order.
    pub fields: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct DimensionlessDef {
    pub units: Vec<UnitDef>,
}

#[derive(Deserialize, Debug)]
pub struct FamilyDef {
    /// The feature name, e.g. `"mechanical"`.
    pub name: String,
    #[serde(default)]
    pub quantity: Vec<QuantityDef>,
}

#[derive(Deserialize, Debug)]
pub struct QuantityDef {
    /// PascalCase quantity name, e.g. `"Length"`.
    pub name: String,
    /// Non-zero dimension exponents; absent fields are 0.
    #[serde(default)]
    pub dims: BTreeMap<String, i64>,
    /// Optional kind variant name (without `Kind::` prefix).
    /// `None` means the plain label of `Dim::DIMENSIONLESS`.
    pub kind: Option<String>,
    pub units: Vec<UnitDef>,
}

/// One row in a `units = [...]` inline array.
/// Field names are abbreviated (`v`/`m`/`z`/`s`/`l`/`t`) to keep each TOML line compact.
///
/// A unit is its size and its zero in the quantity's coherent unit: a number
/// `n` in it is `n · m + z` coherent units (`m = "0.0254"` for the inch, `m =
/// "1.0", z = "273.15"` for the Celsius scale). It is a constant of its own
/// type, `SIUnit<D, { SIUnitConsts { .. } }>`, named by `t` or its variant in
/// SCREAMING_SNAKE_CASE.
#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct UnitDef {
    /// PascalCase enum variant name, e.g. `"Kilometre"`.
    #[serde(rename = "v")]
    pub variant: String,
    /// The size in coherent units, a Rust expression of number literals and
    /// `core::f64::consts::PI`, read exactly: `"1000.0"`, `"5.0 / 9.0"`.
    #[serde(rename = "m")]
    pub multiplier: String,
    /// The zero in coherent units, the same way: `"273.15"`. None is `0`.
    #[serde(rename = "z", default)]
    pub zero: Option<String>,
    /// Short symbol string, e.g. `"km"`.
    #[serde(rename = "s")]
    pub symbol: String,
    /// Long plural name, e.g. `"kilometres"`.
    #[serde(rename = "l")]
    pub long: String,
    /// The unit's name when not the variant's: `t = "KelvinScale"` names the
    /// constant `KELVIN_SCALE`.
    #[serde(rename = "t", default)]
    pub type_name: Option<String>,
}

// --- UCUM-mode config ---------------------------------------------------------

/// Configuration for UCUM-driven generation (`[ucum]` table).
#[derive(Deserialize, Debug)]
pub struct UcumSection {
    /// Where to fetch `ucum-essence.xml`.
    ///
    /// | Value | Behaviour |
    /// |-------|-----------|
    /// | `"latest"` | query GitHub releases API for the newest tag |
    /// | `"https://..."` or `"http://..."` | fetch this URL directly |
    /// | any other string | read as a local file path |
    pub source: String,
}
