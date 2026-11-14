use serde::Deserialize;
use std::collections::BTreeMap;

/// Root of a `.toml` unit system definition.
#[derive(Deserialize, Debug)]
pub struct UnitSystem {
    pub system: SystemConfig,
    pub dim: DimConfig,
    /// The units of a plain number, in the always-present `dimensionless` module.
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
    /// Name for the quantity type, `<name><const U: <unit>, N>`. Defaults to
    /// `"Qty"`.
    #[serde(default)]
    pub qty_type: Option<String>,
    /// Name for the dynamic quantity type. Defaults to `"DynQty"`.
    #[serde(default)]
    pub dynqty_type: Option<String>,
    /// Name for the unit type, the quantity type's const parameter. Defaults
    /// to `"QtyUnit"`.
    #[serde(default)]
    pub unit_type: Option<String>,
    /// Symbols of coherent units printed by name: a quantity printed in the
    /// coherent unit of a unit listed here (`"N"`, `"J"`, ..) prints its
    /// symbol rather than the product of base units. Each names a unit of
    /// size one in its quantity; the first listed wins where two share a
    /// coherent unit.
    #[serde(default)]
    pub named: Vec<String>,
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
pub struct DimConfig {
    /// The base units' symbols, in order: a base unit per field, the unit
    /// whose symbol it is in the quantity of that field alone (`"m"`,
    /// `METRE`).
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
    pub units: Vec<UnitDef>,
}

/// One row in a `units = [...]` inline array.
/// Field names are abbreviated (`v`/`m`/`z`/`s`/`l`/`t`) to keep each TOML line compact.
///
/// A unit is its size and its zero in the quantity's coherent unit: a number
/// `n` in it is `n · m + z` coherent units (`m = "0.0254"` for the inch, `m =
/// "1.0", z = "273.15"` for the Celsius scale). A unit on a logarithmic
/// scale has a `log` base and a `per`: `n` of it is `m · log^(n / per)`
/// coherent units (dBm: `m = "0.001"`, `log = "10"`, `per = "10"`). A unit of
/// size one, without a zero or a scale, is the coherent unit itself. Each is
/// a constant quantity of one of it, named by `t` or its variant in
/// SCREAMING_SNAKE_CASE.
#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct UnitDef {
    /// PascalCase enum variant name, e.g. `"Kilometre"`.
    #[serde(rename = "v")]
    pub variant: String,
    /// The size in coherent units (the reference of a log unit), a Rust
    /// expression of number literals and `core::f64::consts::PI`, read
    /// exactly: `"1000.0"`, `"5.0 / 9.0"`.
    #[serde(rename = "m")]
    pub multiplier: String,
    /// The zero in coherent units, the same way: `"273.15"`. None is `0`.
    #[serde(rename = "z", default)]
    pub zero: Option<String>,
    /// The base of a log unit, the same way: `"10"`.
    #[serde(default)]
    pub log: Option<String>,
    /// How many of a log unit make one power of its base: `"10"` for a
    /// decibel.
    #[serde(default)]
    pub per: Option<String>,
    /// Short symbol string, e.g. `"km"`.
    #[serde(rename = "s")]
    pub symbol: String,
    /// Long plural name, e.g. `"kilometres"`.
    #[serde(rename = "l")]
    pub long: String,
    /// The unit's name when not the variant's: `t = "DegreeCelsius"` names
    /// the constant `DEGREE_CELSIUS`.
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
