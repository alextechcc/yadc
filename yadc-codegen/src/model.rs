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
}

#[derive(Deserialize, Debug)]
pub struct KindConfig {
    /// Variant names for the `Kind` enum, in declaration order.
    pub variants: Vec<String>,
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
    /// `None` means the default Linear kind from `Dim::ZERO`.
    pub kind: Option<String>,
    /// If set, emits `#[dim_delta(<value>)]` on the unit enum (for affine types).
    pub dim_delta: Option<String>,
    pub units: Vec<UnitDef>,
}

/// One row in a `units = [...]` inline array.
/// Field names are abbreviated (`v`/`m`/`s`/`l`/`o`) to keep each TOML line compact.
#[derive(Deserialize, Debug)]
pub struct UnitDef {
    /// PascalCase enum variant name, e.g. `"Kilometer"`.
    #[serde(rename = "v")]
    pub variant: String,
    /// Rust expression for the multiplier; stored verbatim, e.g. `"1000.0"` or `"5.0 / 9.0"`.
    #[serde(rename = "m")]
    pub multiplier: String,
    /// Short symbol string, e.g. `"km"`.
    #[serde(rename = "s")]
    pub symbol: String,
    /// Long plural name, e.g. `"kilometers"`.
    #[serde(rename = "l")]
    pub long: String,
    /// Affine offset (absolute base-unit reference point), e.g. `273.15` for Celsius.
    #[serde(rename = "o")]
    pub offset: Option<f64>,
}

// ─── UCUM-mode config ─────────────────────────────────────────────────────────

/// Configuration for UCUM-driven generation (`[ucum]` table).
#[derive(Deserialize, Debug)]
pub struct UcumSection {
    /// Where to fetch `ucum-essence.xml`.
    ///
    /// | Value | Behaviour |
    /// |-------|-----------|
    /// | `"latest"` | query GitHub releases API for the newest tag |
    /// | `"https://…"` or `"http://…"` | fetch this URL directly |
    /// | any other string | read as a local file path |
    pub source: String,

    /// How each UCUM base-unit code is seeded in the resolver.
    ///
    /// Omitted base units default to self-seeding: the code is treated as its own
    /// unit-dimension field with exponent 1 and multiplier 1.0.
    #[serde(default)]
    pub base: Vec<UcumBaseMap>,
}

/// One entry in `[[ucum.base]]`.
#[derive(Deserialize, Debug)]
pub struct UcumBaseMap {
    /// UCUM base-unit code, e.g. `"g"`, `"C"`, `"rad"`.
    pub code: String,
    /// Target dimension exponents in the output `[dim]` field set.
    /// Empty (`{}`) means dimensionless.
    #[serde(default)]
    pub dims: BTreeMap<String, i64>,
    /// Scale factor from this unit to SI base units.
    /// E.g. `1e-3` for gram→kilogram.
    #[serde(default = "default_multiplier")]
    pub multiplier: f64,
    /// Kind variant name to assign (without `Kind::` prefix).
    /// Only meaningful when `dims` is empty (dimensionless quantity).
    pub kind: Option<String>,
}

fn default_multiplier() -> f64 { 1.0 }
