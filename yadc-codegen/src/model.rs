use serde::Deserialize;
use std::collections::BTreeMap;

/// Root of a `.toml` unit system definition.
#[derive(Deserialize, Debug)]
pub struct UnitSystem {
    pub system: SystemConfig,
    pub kind: KindConfig,
    pub dim: DimConfig,
    /// The base constants: the only raw numbers of the generated crate, in
    /// its always-present `base` module. Every other constant is defined
    /// from them.
    #[serde(default)]
    pub base: Vec<BaseConst>,
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

/// One base constant: `pub const METRE: SIQuantity<..> =
/// SIQuantity(1.0);` in the generated `base` module.
#[derive(Deserialize, Debug)]
pub struct BaseConst {
    /// `METRE`.
    pub name: String,
    /// Its dimension's exponents; absent fields are 0.
    #[serde(default)]
    pub dims: BTreeMap<String, i64>,
    /// Its label, a `[kind]` variant; none for the plain label.
    pub kind: Option<String>,
    /// The number, `"1.0"` unless said otherwise (`"0.0"` for a zero).
    #[serde(default = "one")]
    pub value: String,
    /// Its doc comment.
    pub doc: String,
}

fn one() -> String {
    "1.0".into()
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
    /// Constants of this module defined by an expression of other constants,
    /// verbatim: a zero point (`ICE_POINT = ABSOLUTE_ZERO + 273.15 * KELVIN`),
    /// or - with a `ty` - a scale (`CELSIUS: Scale<ThermodynamicTemperature,
    /// TemperatureInterval> = Scale(ICE_POINT, KELVIN)`).
    #[serde(default)]
    pub consts: Vec<ExtraConst>,
    pub units: Vec<UnitDef>,
}

/// `{ name = "ICE_POINT", value = "ABSOLUTE_ZERO + 273.15 * KELVIN", doc = ".." }`.
#[derive(Deserialize, Debug, Clone)]
pub struct ExtraConst {
    pub name: String,
    /// Its type, when not the quantity it is listed under: a scale's
    /// `Scale<Zero, Step>`.
    #[serde(default)]
    pub ty: Option<String>,
    pub value: String,
    pub doc: String,
}

/// One row in a `units = [...]` inline array.
/// Field names are abbreviated (`v`/`m`/`s`/`l`/`c`) to keep each TOML line compact.
///
/// A unit is one of:
///
/// - **a multiple** (`m`): its constant, `m * COHERENT` - `pub const KILOMETRE:
///   Length = 1000.0 * METRE;` - where `COHERENT` is the quantity's coherent
///   unit (the first with `m = "1.0"`, itself the product of base constants:
///   `NEWTON = METRE * KILOGRAM / (SECOND * SECOND)`); the enum variant reads
///   `#[unit("km", "kilometres", by = KILOMETRE)]`;
/// - **a unit value named in the TOML** (`by`), verbatim, with no constant of
///   its own: another constant (`by = "KELVIN"` for the degree Celsius of a
///   temperature step), a scale (`by = "CELSIUS"`, a constant of the
///   quantity's `consts`), or a path into a hand-written module.
#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct UnitDef {
    /// PascalCase enum variant name, e.g. `"Kilometre"`.
    #[serde(rename = "v")]
    pub variant: String,
    /// Rust expression for the multiplier - the unit's size in coherent
    /// units, verbatim: `"1000.0"`, `"5.0 / 9.0"`.
    #[serde(rename = "m", default)]
    pub multiplier: Option<String>,
    /// Short symbol string, e.g. `"km"`.
    #[serde(rename = "s")]
    pub symbol: String,
    /// Long plural name, e.g. `"kilometres"`.
    #[serde(rename = "l")]
    pub long: String,
    /// The constant's name, when not the variant's in SCREAMING_SNAKE_CASE
    /// (`DEGREE_FAHRENHEIT` for the `Fahrenheit` step).
    #[serde(rename = "c", default)]
    pub constant: Option<String>,
    /// The unit value this unit is, verbatim (no constant of its own).
    #[serde(default)]
    pub by: Option<String>,
    /// The type the `by` unit value measures, when not this quantity: the
    /// two convert with std `From` (a level's scale reading a power).
    #[serde(default)]
    pub via: Option<String>,
    /// The unit's own `Unit` impl is not `const` (a level read as a power
    /// converts through a logarithm), so neither is the enum's.
    #[serde(default)]
    pub non_const: bool,
}

impl UnitDef {
    /// A multiple of the coherent unit, with its own constant.
    pub fn is_multiple(&self) -> bool {
        self.by.is_none()
    }
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
    /// E.g. `1e-3` for gram->kilogram.
    #[serde(default = "default_multiplier")]
    pub multiplier: f64,
    /// Kind variant name to assign (without `Kind::` prefix).
    /// Only meaningful when `dims` is empty (dimensionless quantity).
    pub kind: Option<String>,
}

fn default_multiplier() -> f64 {
    1.0
}
