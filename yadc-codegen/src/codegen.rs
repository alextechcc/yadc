//! The generator: a `UnitSystem` to a crate's `src/*.rs` (and a starting
//! `Cargo.toml`).
//!
//! The output is laid out by `rustfmt` after it is written, so it is
//! fmt-clean and the same for the same input.

use crate::model::*;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;

/// The names a generated file refers to: the system's own types, by role.
struct Names<'a> {
    /// `SIQuantity`: a number of a unit.
    qty: &'a str,
    /// `DynamicSIQuantity`: a quantity of a unit chosen at run time.
    dyn_: &'a str,
    /// `SIUnit`: the quantity's const parameter.
    unit: &'a str,
}

impl<'a> Names<'a> {
    fn of(sys: &'a SystemConfig) -> Self {
        Names {
            qty: sys.qty_type.as_deref().unwrap_or("Qty"),
            dyn_: sys.dynqty_type.as_deref().unwrap_or("DynQty"),
            unit: sys.unit_type.as_deref().unwrap_or("QtyUnit"),
        }
    }
}

/// The exact number a unit's size and zero are written in, as `src/exact.rs`.
const EXACT: &str = include_str!("../templates/exact.rs");

/// The unit type, as `src/unit.rs`. `__UNIT__` and `__QTY__` are the
/// system's type names, `__F__` and `__B__` how many named units a unit and
/// how many base units a named unit hold.
const UNIT: &str = include_str!("../templates/unit.rs");

/// The SI prefixes: name, symbol, power of ten.
const PREFIXES: [(&str, &str, i32); 20] = [
    ("Yotta", "Y", 24),
    ("Zetta", "Z", 21),
    ("Exa", "E", 18),
    ("Peta", "P", 15),
    ("Tera", "T", 12),
    ("Giga", "G", 9),
    ("Mega", "M", 6),
    ("Kilo", "k", 3),
    ("Hecto", "h", 2),
    ("Deca", "da", 1),
    ("Deci", "d", -1),
    ("Centi", "c", -2),
    ("Milli", "m", -3),
    ("Micro", "μ", -6),
    ("Nano", "n", -9),
    ("Pico", "p", -12),
    ("Femto", "f", -15),
    ("Atto", "a", -18),
    ("Zepto", "z", -21),
    ("Yocto", "y", -24),
];

// --- Public entry point -------------------------------------------------------

pub fn write_crate(system: &UnitSystem, output_dir: &Path) -> std::io::Result<()> {
    for fam in &system.family {
        assert!(
            system.system.family_order.contains(&fam.name),
            "family `{}` is not listed in [system].family_order: add it",
            fam.name,
        );
    }
    let src = output_dir.join("src");
    let dynamic = src.join("dynamic");
    std::fs::create_dir_all(&dynamic)?;
    let mut written = Vec::new();
    let mut write = |path: std::path::PathBuf, text: &str| -> std::io::Result<()> {
        std::fs::write(&path, text)?;
        written.push(path);
        Ok(())
    };
    let ctx = Context::new(system);
    write(src.join("lib.rs"), &gen_lib_rs(&ctx))?;
    write(src.join("exact.rs"), EXACT)?;
    write(src.join("unit.rs"), &gen_unit_rs(&ctx))?;
    std::fs::write(output_dir.join("Cargo.toml"), gen_cargo_toml(system))?;

    // Each module's quantities and units at the root, its unit enums in
    // `dynamic`.
    let mut unit_mods: Vec<(String, Option<String>)> = Vec::new();
    for m in ctx.modules() {
        let out = gen_module(&ctx, &m);
        write(src.join(format!("{}.rs", m.name)), &out.quantities)?;
        if !out.units.is_empty() {
            write(dynamic.join(format!("{}.rs", m.name)), &out.units)?;
            unit_mods.push((m.name.to_string(), m.feature.map(str::to_string)));
        }
    }
    write(dynamic.join("mod.rs"), &gen_dynamic_rs(&ctx, &unit_mods))?;
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

// --- The system as the generator sees it ----------------------------------------

/// One module: `dimensionless`, always present, or a family behind its
/// feature.
struct ModuleRef<'a> {
    name: &'a str,
    feature: Option<&'a str>,
    quantities: Vec<QuantityRef<'a>>,
}

/// One quantity of a module, as its units see it.
struct QuantityRef<'a> {
    name: &'a str,
    /// Its exponents, in `[dim]` order, without zeros.
    dims: Vec<(String, i64)>,
    /// The units, each variant once.
    units: Vec<&'a UnitDef>,
}

/// What a unit is, in its quantity's coherent unit.
#[derive(PartialEq, Debug)]
enum Shape {
    /// The coherent unit itself: size one, no zero, no scale.
    Coherent,
    /// `n · size` coherent units.
    Scaled { size: String },
    /// `n · size + zero` coherent units.
    Offset { size: String, zero: String },
    /// `reference · base^(n / per)` coherent units.
    Log {
        base: String,
        per: String,
        reference: String,
    },
}

fn shape(u: &UnitDef) -> Shape {
    let size = exact_literals(&u.multiplier);
    match (&u.zero, &u.log, &u.per) {
        (None, None, None) if is_one(&u.multiplier) => Shape::Coherent,
        (None, None, None) => Shape::Scaled { size },
        (Some(z), None, None) => Shape::Offset {
            size,
            zero: exact_literals(z),
        },
        (None, Some(base), Some(per)) => Shape::Log {
            base: exact_literals(base),
            per: exact_literals(per),
            reference: size,
        },
        _ => panic!(
            "unit `{}`: a unit has a zero (`z`), a log scale (`log` and `per`, both), or neither",
            u.variant
        ),
    }
}

/// A base unit, from `[dim]`: its symbol, constant (`METRE`) and row.
struct Base<'a> {
    symbol: &'a str,
    constant: String,
    row: &'a UnitDef,
}

/// The whole system, read once: its modules, base units and the names of
/// the coherent units.
struct Context<'a> {
    system: &'a UnitSystem,
    bases: Vec<Base<'a>>,
    /// The lib's unit constants: name -> its row.
    lib_units: HashMap<String, &'a UnitDef>,
    /// Coherent units printed by name: their exponents and symbol.
    named: Vec<(Vec<(String, i64)>, String)>,
}

fn screaming_of(u: &UnitDef) -> String {
    screaming_snake(u.type_name.as_deref().unwrap_or(&u.variant))
}

impl<'a> Context<'a> {
    fn new(system: &'a UnitSystem) -> Self {
        let mut ctx = Context {
            system,
            bases: Vec::new(),
            lib_units: HashMap::new(),
            named: Vec::new(),
        };
        let modules = ctx.modules();
        for f in &system.dim.fields {
            let alone = vec![(f.clone(), 1)];
            let q = modules
                .iter()
                .flat_map(|m| &m.quantities)
                .find(|q| q.dims == alone && q.units.iter().any(|u| u.symbol == *f))
                .unwrap_or_else(|| {
                    panic!(
                        "[dim] field `{f}`: no quantity of `{f}` alone has a unit with the symbol \
                         `{f}`, its base unit"
                    )
                });
            let row = *q.units.iter().find(|u| u.symbol == *f).unwrap();
            assert!(
                shape(row) == Shape::Coherent,
                "[dim] field `{f}`: its base unit `{}` is not of size one in `{}`",
                row.variant,
                q.name
            );
            let base = Base {
                symbol: f,
                constant: screaming_of(row),
                row,
            };
            ctx.lib_units.insert(base.constant.clone(), row);
            ctx.bases.push(base);
        }
        for symbol in &system.system.named {
            let (dims, _) = modules
                .iter()
                .flat_map(|m| &m.quantities)
                .flat_map(|q| q.units.iter().map(move |u| (&q.dims, u)))
                .find(|(d, u)| !d.is_empty() && u.symbol == *symbol && shape(u) == Shape::Coherent)
                .unwrap_or_else(|| {
                    panic!(
                        "[system] named `{symbol}`: no unit of size one has that symbol in a \
                         quantity with a dimension"
                    )
                });
            if !ctx.named.iter().any(|(d, _)| d == dims) {
                ctx.named.push((dims.clone(), symbol.clone()));
            }
        }
        ctx
    }

    /// The modules: `dimensionless`, then the families in order.
    fn modules(&self) -> Vec<ModuleRef<'a>> {
        let system = self.system;
        let mut out = vec![ModuleRef {
            name: "dimensionless",
            feature: None,
            quantities: vec![QuantityRef::new(
                "Dimensionless",
                &BTreeMap::new(),
                &system.dimensionless.units,
                &system.dim.fields,
            )],
        }];
        for name in &system.system.family_order {
            let Some(fam) = system.family.iter().find(|f| &f.name == name) else {
                continue;
            };
            if fam.quantity.is_empty() {
                continue;
            }
            out.push(ModuleRef {
                name,
                feature: Some(name),
                quantities: fam
                    .quantity
                    .iter()
                    .map(|q| QuantityRef::new(&q.name, &q.dims, &q.units, &system.dim.fields))
                    .collect(),
            });
        }
        out
    }

    /// A unit the lib declares: a base unit.
    fn is_lib_unit(&self, u: &UnitDef) -> bool {
        self.lib_units
            .get(&screaming_of(u))
            .is_some_and(|r| r.symbol == u.symbol && shape(r) == shape(u))
    }

    /// The coherent unit of these exponents, the product of the base-unit
    /// constants: `(KILOGRAM * METRE / (SECOND * SECOND)).unit()`.
    fn coherent(&self, dims: &[(String, i64)]) -> String {
        let unit = Names::of(&self.system.system).unit;
        let side = |positive: bool| -> Vec<String> {
            let mut out = Vec::new();
            for (f, n) in dims.iter().filter(|(_, n)| (*n > 0) == positive) {
                let c = &self.bases.iter().find(|b| b.symbol == f).unwrap().constant;
                for _ in 0..n.unsigned_abs() {
                    out.push(c.clone());
                }
            }
            out
        };
        let (num, den) = (side(true), side(false));
        match (num.len(), den.len()) {
            (0, 0) => format!("{unit}::ONE"),
            (1, 0) => format!("{}.unit()", num[0]),
            (_, 0) => format!("({}).unit()", num.join(" * ")),
            // No base unit above the line: a reciprocal, by the unit's own algebra.
            (0, _) => {
                let mut e = format!("{unit}::ONE");
                for c in &den {
                    e = format!("{e}.times({c}.unit(), -1)");
                }
                e
            }
            (_, 1) => format!("({} / {}).unit()", num.join(" * "), den[0]),
            (_, _) => format!("({} / ({})).unit()", num.join(" * "), den.join(" * ")),
        }
    }

    /// A quantity's base-unit constant: its coherent unit's (a base unit's,
    /// or one of its own units), else one named after its exponents
    /// (`METRE_PER_SECOND`), which `own_base` says to write.
    fn base_const(&self, q: &QuantityRef<'_>, names: &BTreeMap<(&str, &str), String>) -> String {
        match q.units.iter().find(|u| self.is_base(q, u)) {
            Some(u) => names
                .get(&(q.name, u.variant.as_str()))
                .cloned()
                .unwrap_or_else(|| screaming_of(u)),
            None => screaming_snake(&coherent_variant(self, &q.dims)),
        }
    }

    /// Whether a quantity's base-unit constant is written beside it: it has
    /// no unit of size one of its own, its exponents are not a base unit's,
    /// and the crate root doesn't have it (for a quantity several modules
    /// share).
    fn own_base(&self, q: &QuantityRef<'_>) -> bool {
        !q.units.iter().any(|u| self.is_base(q, u))
            && !self.bases.iter().any(|b| q.dims == vec![(b.symbol.to_string(), 1)])
            && !self.shared_quantities().iter().any(|(_, d)| *d == q.dims)
    }

    /// Whether a unit is its quantity's coherent unit, a base unit or a
    /// product of base units: the quantity's first unit of size one, without
    /// a zero or a log scale (`m²` for an area, `N · m` for a torque). A
    /// later unit of size one (`°C` for a temperature step) is a named unit
    /// of its own.
    fn is_base(&self, q: &QuantityRef<'_>, u: &UnitDef) -> bool {
        q.units.iter().find(|x| shape(x) == Shape::Coherent).is_some_and(|first| std::ptr::eq(*first, u))
    }

    /// A prefixed unit: one whose symbol is an SI prefix's followed by
    /// another unit's of the same quantity - a base unit, a named unit or
    /// one of size one - and whose size is the prefix's times that unit's,
    /// exactly. The prefix's constant and the other unit.
    fn prefixed<'q>(&self, q: &QuantityRef<'q>, u: &UnitDef) -> Option<(String, &'q UnitDef)> {
        if self.is_lib_unit(u) || self.is_base(q, u) {
            return None;
        }
        let linear = |u: &UnitDef| matches!(shape(u), Shape::Coherent | Shape::Scaled { .. });
        if !linear(u) {
            return None;
        }
        let size = exact_value(&u.multiplier)?;
        for (name, symbol, exp) in PREFIXES {
            let Some(rest) = u.symbol.strip_prefix(symbol) else { continue };
            if rest.is_empty() {
                continue;
            }
            let Some(other) = q.units.iter().find(|o| o.symbol == rest && linear(o)) else {
                continue;
            };
            if self.prefixed(q, other).is_some() {
                continue;
            }
            let Some(of) = exact_value(&other.multiplier) else { continue };
            let ten = num_rational::BigRational::from_integer(10.into());
            if size == (of.0 * num_traits::Pow::pow(ten, exp), of.1) {
                return Some((screaming_snake(name), *other));
            }
        }
        None
    }

    /// The quantities whose name more than one module has, each with its
    /// exponents: their aliases are defined once, at the crate root, so they
    /// exist whatever modules are compiled and glob imports of two modules
    /// don't clash.
    fn shared_quantities(&self) -> Vec<(&'a str, Vec<(String, i64)>)> {
        let mut first: Vec<(&'a str, Vec<(String, i64)>)> = Vec::new();
        let mut shared: Vec<&'a str> = Vec::new();
        for m in self.modules() {
            for q in &m.quantities {
                match first.iter().find(|(n, _)| *n == q.name) {
                    Some((_, dims)) => {
                        assert!(
                            *dims == q.dims,
                            "quantity `{}` is of two dimensions, in `{}` and another module: rename one",
                            q.name,
                            m.name
                        );
                        if !shared.contains(&q.name) {
                            shared.push(q.name);
                        }
                    }
                    None => first.push((q.name, q.dims.clone())),
                }
            }
        }
        first.retain(|(n, _)| shared.contains(n));
        first
    }

    /// A unit as a const expression: its shape, of its quantity's coherent
    /// unit.
    fn unit_expr(&self, q: &QuantityRef<'_>, u: &UnitDef, names: &BTreeMap<(&str, &str), String>) -> String {
        let unit = Names::of(&self.system.system).unit;
        let coherent = format!("{}.unit()", self.base_const(q, names));
        let symbol = rust_str(&u.symbol);
        match shape(u) {
            Shape::Coherent if self.is_base(q, u) => self.coherent(&q.dims),
            Shape::Coherent => format!("{unit}::scaled(\"{symbol}\", Exact::int(1), {coherent})"),
            Shape::Scaled { size } => format!("{unit}::scaled(\"{symbol}\", {size}, {coherent})"),
            Shape::Offset { size, zero } => {
                format!("{unit}::offset(\"{symbol}\", {size}, {zero}, {coherent})")
            }
            Shape::Log {
                base,
                per,
                reference,
            } => format!(
                "{unit}::from_log(\"{symbol}\", {base}, {per}, {reference}, {coherent})"
            ),
        }
    }
}

impl<'a> QuantityRef<'a> {
    fn new(
        name: &'a str,
        dims: &BTreeMap<String, i64>,
        units: &'a [UnitDef],
        fields: &[String],
    ) -> Self {
        for f in dims.keys() {
            assert!(
                fields.contains(f),
                "quantity `{name}`: `{f}` is not a [dim] field {fields:?}"
            );
        }
        let dims = fields
            .iter()
            .filter_map(|f| dims.get(f).filter(|n| **n != 0).map(|n| (f.clone(), *n)))
            .collect();
        let mut seen = HashSet::new();
        let units = units
            .iter()
            .filter(|u| seen.insert(u.variant.as_str()))
            .collect();
        QuantityRef { name, dims, units }
    }
}

// --- lib.rs -------------------------------------------------------------------

/// The quantity type and what is the same for every system: `__QTY__`,
/// `__UNIT__` and `__DYN__` are the system's type names, `__FROM__` the
/// mixed-number pairs.
const LIB: &str = r#"#![cfg_attr(not(feature = "std"), no_std)]
#![feature(generic_const_exprs)]
// A unit is a struct with `&'static str`s, as a const parameter.
#![feature(adt_const_params)]
#![feature(unsized_const_params)]
#![feature(const_trait_impl)]
#![feature(const_cmp)]
#![feature(const_default)]
#![feature(derive_const)]
#![feature(const_ops)]
#![feature(const_convert)]
// The dynamic type's const_derive_more impls drop their operands in `const` code.
#![feature(const_destruct)]
#![allow(incomplete_features)]
#![allow(clippy::excessive_precision)]
#![allow(clippy::approx_constant)]

pub mod exact;
pub use exact::Exact;
pub mod unit;
pub use unit::__UNIT__;

use core::ops::{Add, Div, Mul, Sub};

/// Whether a quantity's unit is a base unit or a product of base units
/// (`BASE`), or not (`NOT_BASE`). A product is `BASE` where both factors are;
/// a sum needs both the same.
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, core::marker::ConstParamTy)]
pub enum Base {
    Yes,
    No,
}

/// A base unit or a product of base units.
pub const BASE: Base = Base::Yes;
/// Any other unit.
pub const NOT_BASE: Base = Base::No;

impl Base {
    const fn and(self, o: Base) -> Base {
        if self == BASE && o == BASE { BASE } else { NOT_BASE }
    }
}

const impl Mul for Base {
    type Output = Option<Base>;
    fn mul(self, o: Base) -> Option<Base> {
        Some(self.and(o))
    }
}
const impl Div for Base {
    type Output = Option<Base>;
    fn div(self, o: Base) -> Option<Base> {
        Some(self.and(o))
    }
}
/// The same on both sides adds; nothing else does.
const impl Add for Base {
    type Output = Option<Base>;
    fn add(self, o: Base) -> Option<Base> {
        if self == o { Some(self) } else { None }
    }
}
const impl Sub for Base {
    type Output = Option<Base>;
    fn sub(self, o: Base) -> Option<Base> {
        if self == o { Some(self) } else { None }
    }
}
/// A quantity scaled by a number keeps its unit, and so its `Base`.
const impl Mul<f64> for Base {
    type Output = Base;
    fn mul(self, _: f64) -> Base {
        self
    }
}
const impl Div<f64> for Base {
    type Output = Base;
    fn div(self, _: f64) -> Base {
        self
    }
}

/// A quantity: its number, counted in the unit `U` its type names, in
/// numbers of type `N`; `B` says whether `U` is a base unit or a product of
/// base units. Quantities of one unit add and compare; a product is counted
/// in the product of the units (`mass * acceleration` in kg·m/s^2, a
/// `Force`), and is `BASE` where both units are. Every unit is a constant,
/// one of it counted in `Exact`: a number times it is a quantity
/// (`2.0_f64 * UNIT`), `From` converts between it and the base units of its
/// dimension, and a quantity of a plain number is a number
/// (`f64::from(d / UNIT)`).
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, const_derive_more::ConstOps)]
#[const_ops(from(Exact => __FROM__))]
pub struct __QTY__<
    #[ops(one = __UNIT__::ONE)] const U: __UNIT__,
    N,
    #[ops(one = BASE)] const B: Base,
>(N);

/// It serializes as just its number: the unit is the deserialization
/// target's.
#[cfg(feature = "serde")]
impl<const U: __UNIT__, N: serde::Serialize, const B: Base> serde::Serialize
    for __QTY__<U, N, B>
{
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(s)
    }
}

#[cfg(feature = "serde")]
impl<'de, const U: __UNIT__, N: serde::Deserialize<'de>, const B: Base>
    serde::Deserialize<'de> for __QTY__<U, N, B>
{
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        N::deserialize(d).map(__QTY__)
    }
}

impl<const U: __UNIT__, N, const B: Base> __QTY__<U, N, B> {
    /// Its unit.
    pub const fn unit(&self) -> __UNIT__ {
        U
    }
}

impl<const U: __UNIT__, const B: Base> __QTY__<U, Exact, B> {
    /// One of the unit.
    pub const ONE: Self = __QTY__(Exact::ONE);
}

/// One of a unit, counted exactly: every unit constant is one
/// (`pub const KILOMETRE: Unit<{ (KILO * METRE).unit() }> = Unit::ONE;`), a
/// base unit marked `BASE`.
pub type Unit<const U: __UNIT__, const B: Base = NOT_BASE> = __QTY__<U, Exact, B>;

/// The number, a space and the unit: `2 km`, `9.81 N`.
impl<const U: __UNIT__, N: core::fmt::Display, const B: Base> core::fmt::Display
    for __QTY__<U, N, B>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // The flags are the number's: the unit is written plain.
        self.0.fmt(f)?;
        write!(f, " {U}")
    }
}

/// A condition on units, as a type: `Check<{ U.same_dimension(V) }>: True`
/// holds where `U` and `V` measure the same dimension.
pub struct Check<const B: bool>;

/// What a true `Check` is.
pub trait True {}
impl True for Check<true> {}

/// How a unit's numbers convert to and from base units, in numbers `N`:
/// `Check<false>` for a linear unit (`n · size + zero`), `Check<true>` for a
/// log unit (`reference · base^(n / per)`, so `N` is `Real`).
pub trait Conversion<N> {
    /// The number of base units `n` of `unit` is.
    fn to_coherent(unit: __UNIT__, n: N) -> N;
    /// The number of `unit` `x` base units is.
    fn from_coherent(unit: __UNIT__, x: N) -> N;
}

impl<N> Conversion<N> for Check<false>
where
    N: From<Exact> + Mul<Output = N> + Add<Output = N> + Sub<Output = N> + Div<Output = N>,
{
    fn to_coherent(unit: __UNIT__, n: N) -> N {
        unit.to_coherent(n)
    }
    fn from_coherent(unit: __UNIT__, x: N) -> N {
        unit.from_coherent(x)
    }
}

impl<N: num_traits::real::Real + From<Exact>> Conversion<N> for Check<true> {
    fn to_coherent(unit: __UNIT__, n: N) -> N {
        unit.log_to_coherent(n)
    }
    fn from_coherent(unit: __UNIT__, x: N) -> N {
        unit.log_from_coherent(x)
    }
}

/// A quantity of any unit in a base unit of its dimension, the one the
/// target type names: `let d: Length<f64> = (2.0_f64 * UNIT).into();`.
impl<const U: __UNIT__, const V: __UNIT__, N> From<__QTY__<U, N, NOT_BASE>> for __QTY__<V, N, BASE>
where
    N: From<Exact> + Sub<Output = N> + Div<Output = N>,
    Check<{ U.same_dimension(V) }>: True,
    Check<{ U.is_log() }>: Conversion<N>,
{
    fn from(q: __QTY__<U, N, NOT_BASE>) -> Self {
        __QTY__(V.from_coherent(<Check<{ U.is_log() }> as Conversion<N>>::to_coherent(U, q.0)))
    }
}

/// A quantity in a base unit in any unit of its dimension, the one the
/// target type names.
impl<const U: __UNIT__, const V: __UNIT__, N> From<__QTY__<V, N, BASE>> for __QTY__<U, N, NOT_BASE>
where
    N: From<Exact> + Mul<Output = N> + Add<Output = N>,
    Check<{ U.same_dimension(V) }>: True,
    Check<{ U.is_log() }>: Conversion<N>,
{
    fn from(q: __QTY__<V, N, BASE>) -> Self {
        __QTY__(<Check<{ U.is_log() }> as Conversion<N>>::from_coherent(U, V.to_coherent(q.0)))
    }
}
"#;

/// The number types an `Exact` meets: a unit constant scales any of them.
const FROM: &str = "f64, f32, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize";

/// A quantity of a plain number as a number of each primitive type: the
/// number scaled by its unit's size. An integer reads only units whose size
/// is a whole number (`m/km` is not one; `km/m` is), so nothing rounds.
fn gen_numbers(qty: &str, unit: &str) -> String {
    let mut s = String::new();
    for p in FROM.split(", ") {
        if p.starts_with('f') {
            s += &format!("/// A plain number in `{p}`: its number times its unit's size -\n");
            s += &format!("/// `{p}::from(d / UNIT)` is `d` counted in `UNIT`.\n");
        } else {
            s += &format!("/// A plain number in `{p}`, where its unit's size is a whole number:\n");
            s += "/// its number times that size.\n";
        }
        let check = if p.starts_with('f') {
            "U.is_plain_number()"
        } else {
            "U.is_whole_plain_number()"
        };
        s += &format!("impl<const U: {unit}, const B: Base> From<{qty}<U, {p}, B>> for {p}\n");
        s += &format!("where\n    Check<{{ {check} }}>: True,\n{{\n");
        s += &format!("    fn from(q: {qty}<U, {p}, B>) -> {p} {{\n");
        s += &format!("        q.0 * const {{ <{p} as From<Exact>>::from(U.size()) }}\n");
        s += "    }\n}\n\n";
    }
    s
}

fn gen_unit_rs(ctx: &Context<'_>) -> String {
    let n = Names::of(&ctx.system.system);
    let f = ctx.system.dim.fields.len().max(1).to_string();
    UNIT.replace("__UNIT__", n.unit)
        .replace("__QTY__", n.qty)
        .replace("__F__", &f)
        .replace("__B__", &f)
}

fn gen_lib_rs(ctx: &Context<'_>) -> String {
    let system = ctx.system;
    let n = Names::of(&system.system);
    let (qty, unit) = (n.qty, n.unit);
    let mut s = String::new();
    s += "//! A unit system built on yadc, generated by yadc-codegen.\n";
    s += "//!\n";
    s += &format!(
        "//! - `{qty}<U, N, B>`: a quantity, its unit `U` (a `{unit}`) in its type, counted in\n//!   `N`; `B` says whether `U` is a base unit or a product of base units.\n"
    );
    // The first base unit, a quantity of it alone, and another unit of that
    // quantity, as examples.
    let base = &ctx.bases[0];
    let alone = vec![(base.symbol.to_string(), 1)];
    let modules = ctx.modules();
    let quantities = || modules.iter().flat_map(|m| m.quantities.iter().map(move |q| (m, q)));
    let (_, first) = quantities()
        .find(|(_, q)| q.dims == alone)
        .expect("a quantity of the first base unit alone");
    let other = quantities()
        .filter(|(_, q)| q.dims == alone)
        .find_map(|(m, q)| {
            let u = q.units.iter().find(|u| !ctx.is_base(q, u))?;
            Some(unit_names(ctx, &m.quantities)[&(q.name, u.variant.as_str())].clone())
        });
    s += &format!(
        "//! - The prefixes and the base units, constants here (`KILO`, `{}`); each\n",
        base.constant
    );
    s += &format!(
        "//!   module's quantities (`{}<N>`, in the coherent unit by default) and units\n",
        first.name
    );
    s += &format!(
        "//!   {}each unit a constant: one of it.\n",
        other.map_or(String::new(), |o| format!("(`{o}`), "))
    );
    s += &format!(
        "//! - `dynamic`: `{}`, a quantity of a unit chosen at run time; and each\n//!   module's unit enums, a choice among a quantity's units.\n",
        n.dyn_
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
    s += &LIB
        .replace("__QTY__", qty)
        .replace("__UNIT__", unit)
        .replace("__FROM__", FROM);
    s += "\n";
    s += &gen_numbers(qty, unit);

    s += "/// The coherent units printed by name, not as their product of base units.\n";
    s += &format!(
        "static NAMED: [({unit}, &str); {}] = [\n",
        ctx.named.len()
    );
    for (dims, symbol) in &ctx.named {
        s += &format!("    ({}, \"{}\"),\n", ctx.coherent(dims), rust_str(symbol));
    }
    s += "];\n\n";

    for (name, symbol, exp) in PREFIXES {
        let size = prefix_size(exp);
        s += &format!("/// The prefix {} (`{symbol}`): 10^{exp}.\n", name.to_lowercase());
        s += &format!(
            "pub const {}: Unit<{{ {unit}::prefix(\"{symbol}\", {size}) }}> = Unit::ONE;\n",
            screaming_snake(name)
        );
    }
    s += "\n";

    for b in &ctx.bases {
        s += &format!(
            "/// {}: a base unit, `{}`.\n",
            capitalized(&b.row.long),
            b.symbol
        );
        s += &format!(
            "pub const {}: Unit<{{ {unit}::base(\"{}\") }}, BASE> = Unit::ONE;\n",
            b.constant,
            rust_str(b.symbol)
        );
    }
    s += "\n";

    // The aliases of quantities several modules have.
    let n = Names::of(&ctx.system.system);
    let mut written: HashSet<String> = HashSet::new();
    for (name, dims) in ctx.shared_quantities() {
        let single = ctx.bases.iter().find(|b| dims == vec![(b.symbol.to_string(), 1)]);
        let base = match single {
            Some(b) => b.constant.clone(),
            None => screaming_snake(&coherent_variant(ctx, &dims)),
        };
        s += &format!(
            "/// {}, in `N`s of the unit `U`: by default its base unit, `{base}`.\n",
            article(name)
        );
        s += &format!(
            "pub type {name}<N, const U: {} = {{ {base}.unit() }}, const B: Base = BASE> = {}<U, N, B>;\n",
            n.unit, n.qty
        );
        if single.is_none() && written.insert(base.clone()) {
            s += &format!(
                "/// The base unit of `{name}`: a product of base units.\npub const {base}: Unit<{{ {} }}, BASE> = Unit::ONE;\n",
                ctx.coherent(&dims)
            );
        }
        s += "\n";
    }
    s += "pub mod dimensionless;\n";
    s += "pub mod dynamic;\n";
    s += "\n";
    // Hand-written, non-generated modules (e.g. uom_bridge): declared here,
    // their files never written.
    for m in &system.system.extra_modules {
        if let Some(feat) = &m.feature {
            s += &format!("#[cfg(feature = \"{feat}\")]\n");
        }
        s += &format!("pub mod {};\n", m.name);
    }
    if !system.system.extra_modules.is_empty() {
        s += "\n";
    }
    let mut mods: Vec<ModuleRef<'_>> = ctx.modules().into_iter().skip(1).collect();
    mods.sort_by_key(|m| m.name);
    for m in mods {
        s += &format!("#[cfg(feature = \"{}\")]\n", m.name);
        s += &format!("pub mod {};\n", m.name);
    }
    s
}

// --- Modules -------------------------------------------------------------------

/// A module's two halves: its quantities and units (`<module>.rs`), and its
/// unit enums (`dynamic/<module>.rs`).
pub struct Module {
    pub quantities: String,
    pub units: String,
}

/// Each unit's constant in a module, by quantity and variant: its `t`, or
/// its variant, in SCREAMING_SNAKE_CASE - prefixed with its quantity where
/// two quantities' units share one (`TEMPERATURE_INTERVAL_KELVIN`), so the
/// result does not depend on declaration order. A `t` is never prefixed. A
/// unit the lib declares keeps the lib's constant.
fn unit_names<'a>(ctx: &Context<'_>, quantities: &[QuantityRef<'a>]) -> BTreeMap<(&'a str, &'a str), String> {
    let mut count: BTreeMap<String, usize> = BTreeMap::new();
    for name in ctx.lib_units.keys() {
        count.insert(name.clone(), 1);
    }
    for q in quantities {
        for u in &q.units {
            if !ctx.is_lib_unit(u) {
                *count.entry(screaming_of(u)).or_default() += 1;
            }
        }
    }
    let mut names = BTreeMap::new();
    let mut seen = HashSet::new();
    for q in quantities {
        for u in &q.units {
            let b = screaming_of(u);
            let name = if ctx.is_lib_unit(u) {
                b
            } else {
                let name = if count[&b] > 1 && u.type_name.is_none() {
                    screaming_snake(&format!("{}{}", q.name, u.variant))
                } else {
                    b
                };
                assert!(
                    seen.insert(name.clone()) && !ctx.lib_units.contains_key(&name),
                    "unit constant `{name}` is still ambiguous after prefixing: give one a `t`"
                );
                name
            };
            names.insert((q.name, u.variant.as_str()), name);
        }
    }
    names
}

fn gen_module(ctx: &Context<'_>, m: &ModuleRef<'_>) -> Module {
    let n = Names::of(&ctx.system.system);
    let (qty, unit) = (n.qty, n.unit);
    let names = unit_names(ctx, &m.quantities);
    let shared = ctx.shared_quantities();

    let mut s = String::new();
    if m.name == "dimensionless" {
        s += "//! Plain numbers: ratios, levels and their units.\n\n";
    } else {
        s += &format!("//! The quantities of `{}` and their units.\n\n", m.name);
    }
    s += "#[allow(unused_imports)]\nuse crate::*;\n";
    // Base-unit constants written in this module: two quantities of one
    // dimension share one.
    let mut written: HashSet<String> = HashSet::new();
    for q in &m.quantities {
        let base = ctx.base_const(q, &names);
        match shared.iter().any(|(name, _)| *name == q.name) {
            true => s += &format!("\n// `{}` is `crate::{}`.\n", q.name, q.name),
            false => {
                s += &format!(
                    "\n/// {}, in `N`s of the unit `U`: by default its base unit, `{base}`.\n",
                    article(q.name)
                );
                s += &format!(
                    "pub type {}<N, const U: {unit} = {{ {base}.unit() }}, const B: Base = BASE> = {qty}<U, N, B>;\n",
                    q.name
                );
                if ctx.own_base(q) && written.insert(base.clone()) {
                    s += &format!(
                        "\n/// The base unit of `{}`: a product of base units.\npub const {base}: Unit<{{ {} }}, BASE> = Unit::ONE;\n",
                        q.name,
                        ctx.coherent(&q.dims)
                    );
                }
            }
        }
        // The base unit first, then the others.
        let mut units: Vec<&&UnitDef> = q.units.iter().collect();
        units.sort_by_key(|u| !ctx.is_base(q, u));
        for u in units {
            if ctx.is_lib_unit(u) {
                continue;
            }
            let name = &names[&(q.name, u.variant.as_str())];
            let label = if u.long.is_empty() {
                u.variant.clone()
            } else {
                capitalized(&u.long)
            };
            let symbol = u.symbol.replace('\n', " ");
            let symbol = if symbol.is_empty() || symbol.contains('`') {
                String::new()
            } else {
                format!(" (`{symbol}`)")
            };
            let prefixed = ctx.prefixed(q, u);
            let what = match (&u.zero, &u.log) {
                _ if ctx.is_base(q, u) => "the coherent unit, a product of base units".to_string(),
                _ if prefixed.is_some() => {
                    let (p, o) = prefixed.as_ref().unwrap();
                    format!("`{p} * {}`", names[&(q.name, o.variant.as_str())])
                }
                (Some(z), _) => format!(
                    "`n` of them is `n · {} + {}` base units",
                    u.multiplier.trim(),
                    z.trim()
                ),
                (None, Some(b)) => format!(
                    "`n` of them is `{} · {}^(n / {})` base units",
                    u.multiplier.trim(),
                    b.trim(),
                    u.per.as_deref().unwrap_or("").trim()
                ),
                (None, None) => format!(
                    "`n` of them is `n · {}` base units",
                    u.multiplier.trim()
                ),
            };
            s += &format!("\n/// {label}{symbol}, a unit of `{}`: {what}.\n", q.name);
            s += &match prefixed {
                Some((p, o)) => {
                    let o = &names[&(q.name, o.variant.as_str())];
                    format!("pub const {name}: Unit<{{ ({p} * {o}).unit() }}> = Unit::ONE;\n")
                }
                None if ctx.is_base(q, u) => {
                    format!("pub const {name}: Unit<{{ {} }}, BASE> = Unit::ONE;\n", ctx.unit_expr(q, u, &names))
                }
                None => format!("pub const {name}: Unit<{{ {} }}> = Unit::ONE;\n", ctx.unit_expr(q, u, &names)),
            };
        }
    }

    let mut enums = String::new();
    for q in &m.quantities {
        enums += &gen_unit_enum(ctx, q, &names);
    }
    let mut u = String::new();
    if !enums.is_empty() {
        u += &format!(
            "//! The units of `{}`'s quantities, chosen at run time: for a menu, a\n//! config file, a parser.\n\n",
            m.name
        );
        u += "#[allow(unused_imports)]\n";
        u += &format!("use crate::{}::*;\n", m.name);
        u += "#[allow(unused_imports)]\n";
        u += "use crate::*;\n";
        u += &enums;
    }
    Module {
        quantities: s,
        units: u,
    }
}

/// `Length` -> `A length`; `Information` -> `An information`.
fn article(name: &str) -> String {
    let words = split_words(name);
    let lower = words.join(" ").to_lowercase();
    let a = if lower.starts_with(['a', 'e', 'i', 'o', 'u']) {
        "An"
    } else {
        "A"
    };
    format!("{a} {lower}")
}

fn split_words(ident: &str) -> Vec<String> {
    screaming_snake(ident)
        .split('_')
        .map(str::to_string)
        .collect()
}

// --- Unit enums ----------------------------------------------------------------

/// `<Q>Unit`, the unit enum of quantity `q`: a variant per unit, naming its
/// unit - printed and parsed as it, and converted to it - and the coherent
/// unit, the `#[default]`, first where no unit is it. Nothing for a quantity
/// without units.
fn gen_unit_enum(
    ctx: &Context<'_>,
    q: &QuantityRef<'_>,
    names: &BTreeMap<(&str, &str), String>,
) -> String {
    if q.units.is_empty() {
        return String::new();
    }
    let n = Names::of(&ctx.system.system);
    // The coherent unit is the `#[default]`: what a menu starts on.
    let default = q.units.iter().position(|u| ctx.is_base(q, u));
    let mut variants = String::new();
    if default.is_none() {
        // Named after its base units, or `Coherent` where a unit of another
        // symbol has that name (`m · s`, `MetreSecond`).
        let mut name = coherent_variant(ctx, &q.dims);
        if q.units.iter().any(|u| u.variant == name) {
            name = "Coherent".to_string();
        }
        variants += &format!("    #[default]\n    #[value({}.unit())]\n    {name},\n", ctx.base_const(q, names));
    }
    for (i, unit) in q.units.iter().enumerate() {
        let c = &names[&(q.name, unit.variant.as_str())];
        if Some(i) == default {
            variants += "    #[default]\n";
        }
        variants += &format!("    #[value({c}.unit())]\n    {},\n", unit.variant);
    }
    let mut derives = vec![
        "const_derive_more::Values".to_string(),
        "const_derive_more::Variants".to_string(),
    ];
    derives.extend(ctx.system.system.unit_derives.iter().cloned());
    let mut s = String::new();
    s += &format!(
        "\n/// The units of `{}`: a choice among them, printed and parsed as its\n/// unit, and converted to it.\n",
        q.name
    );
    s += "#[derive_const(Default)]\n";
    s += &format!("#[derive({})]\n", derives.join(", "));
    s += &format!("#[values(into({}))]\n", n.unit);
    s += &format!("pub enum {}Unit {{\n{variants}}}\n", q.name);
    s
}

/// The variant of a coherent unit no unit names: its base units' variants,
/// the positive powers, then `Per` and the negative ones -
/// `MetrePerSecondSquared`; `One` for a plain number.
fn coherent_variant(ctx: &Context<'_>, dims: &[(String, i64)]) -> String {
    let part = |positive: bool| -> String {
        dims.iter()
            .filter(|(_, n)| (*n > 0) == positive)
            .map(|(f, n)| {
                let b = &ctx.bases.iter().find(|b| b.symbol == f).unwrap().row.variant;
                match n.abs() {
                    1 => b.clone(),
                    2 => format!("{b}Squared"),
                    3 => format!("{b}Cubed"),
                    p => format!("{b}Pow{p}"),
                }
            })
            .collect()
    };
    let (num, den) = (part(true), part(false));
    match (num.is_empty(), den.is_empty()) {
        (true, true) => "One".to_string(),
        (_, true) => num,
        (true, _) => format!("Per{den}"),
        _ => format!("{num}Per{den}"),
    }
}

// --- dynamic/mod.rs --------------------------------------------------------------

/// The dynamic quantity: `__QTY__`, `__UNIT__` and `__DYN__` are the
/// system's type names.
const DYNAMIC: &str = r#"/// A quantity of a unit chosen at run time: the dynamic version of
/// `__QTY__<U, f64, B>`, built with const_derive_more. Converting (`From`)
/// moves the number into `value` and records `U` in `unit` and `B` in
/// `is_base`; `TryFrom<__DYN__> for __QTY__<U, f64, B>` is the checked way
/// back (the error hands the original back). A bare `f64` is the same value
/// as a quantity of the plain number's unit (`shared`).
///
/// Its arithmetic is every field's own: the unit's operator on `unit`,
/// `Base`'s on `is_base`, the number's on `value`. Any of them may have no
/// result - `None` where the units don't combine: a sum of two units, a
/// product with a unit that has a zero or a log scale - so a calculation
/// reads `(((a * b)? + c)? * 2.0)?`.
/// Values of one unit are ordered by number, `None` otherwise. A number in
/// another unit is `unit.from_base(other.to_base(value))`.
#[derive(Copy, Clone, PartialEq, Debug, const_derive_more::Dynamic)]
#[dynamic(__QTY__<const U: __UNIT__, f64, const B: Base> => { unit: U, is_base: B, value: .0 })]
#[dynamic(shared f64 => { unit: __UNIT__::ONE, is_base: BASE, value: self })]
#[dynamic(impl(
    Mul -> Option<Self>,
    Div -> Option<Self>,
    Add -> Option<Self>,
    Sub -> Option<Self>,
    Mul<f64> -> Option<Self>,
    Div<f64> -> Option<Self>,
    PartialOrd,
))]
pub struct __DYN__ {
    pub unit: __UNIT__,
    /// Whether `unit` is a base unit or a product of base units, combined as
    /// the static quantities' `B` is.
    pub is_base: Base,
    pub value: f64,
}
"#;

/// `dynamic/mod.rs`: the dynamic quantity, and the modules of unit enums
/// (`dynamic::mechanical::LengthUnit`), each behind its family's feature.
fn gen_dynamic_rs(ctx: &Context<'_>, unit_mods: &[(String, Option<String>)]) -> String {
    let n = Names::of(&ctx.system.system);
    let mut s = String::new();
    s += "//! The dynamic half: what is chosen at run time - a quantity's unit\n";
    s += &format!(
        "//! (`{}`), and a choice among a quantity's units (each module's unit\n//! enums, `dynamic::mechanical::LengthUnit`).\n\n",
        n.dyn_
    );
    s += &format!("use crate::{{{}, {}, BASE, Base}};\n\n", n.qty, n.unit);
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
        .replace("__DYN__", n.dyn_)
        .replace("__QTY__", n.qty)
        .replace("__UNIT__", n.unit);
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
    // `num-traits` and `serde` come without their default features, so
    // `--no-default-features` on this crate is `no_std`; `std` and `libm`
    // pick where `num-traits` finds the `Real` impls the log unit
    // conversions need.
    s += "\n[dependencies]\n";
    s += "const_derive_more = { path = \"../const_derive_more\" }\n";
    s += "num-traits = { version = \"0.2\", default-features = false }\n";
    s += "serde = { version = \"1\", default-features = false, features = [\"derive\"], optional = true }\n";
    s += "\n[features]\n";
    s += "default = [\n";
    s += "    \"std\",\n";
    for f in &nonempty {
        s += &format!("    \"{f}\",\n");
    }
    s += "]\n";
    s += "std = [\"num-traits/std\", \"serde?/std\"]\n";
    s += "# Software float math for a `no_std` build: `Real` (and so the log unit\n";
    s += "# conversions) needs either this or `std`.\n";
    s += "libm = [\"num-traits/libm\"]\n";
    for f in &nonempty {
        s += &format!("{f} = []\n");
    }
    s += "serde = [\"dep:serde\"]\n";
    s
}

// --- Helpers -------------------------------------------------------------------

/// An expression with its number literals written exactly, as `Exact`s:
/// `273.15` is `Exact::dec(27315, -2)`, `1000.0` is `Exact::int(1000)`, and
/// `core::f64::consts::PI` is `Exact::PI`. Everything else is kept as written.
fn exact_literals(expr: &str) -> String {
    let expr = expr.trim().replace("core::f64::consts::PI", "Exact::PI");
    let chars: Vec<char> = expr.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        let prev_ident = i > 0 && (chars[i - 1].is_alphanumeric() || chars[i - 1] == '_');
        let starts = c.is_ascii_digit()
            || (c == '.' && chars.get(i + 1).is_some_and(|d| d.is_ascii_digit()));
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

/// An expression of number literals, `*`, `/`, parentheses and
/// `core::f64::consts::PI`, exactly: a rational and a power of π. `None`
/// for anything else.
fn exact_value(expr: &str) -> Option<(num_rational::BigRational, i32)> {
    use num_rational::BigRational;
    struct P<'a>(&'a [u8], usize);
    impl P<'_> {
        fn skip(&mut self) {
            while self.1 < self.0.len() && self.0[self.1] == b' ' {
                self.1 += 1;
            }
        }
        fn eat(&mut self, c: u8) -> bool {
            self.skip();
            let hit = self.0.get(self.1) == Some(&c);
            if hit {
                self.1 += 1;
            }
            hit
        }
        fn product(&mut self) -> Option<(BigRational, i32)> {
            let mut v = self.factor()?;
            loop {
                if self.eat(b'*') {
                    let f = self.factor()?;
                    v = (v.0 * f.0, v.1 + f.1);
                } else if self.eat(b'/') {
                    let f = self.factor()?;
                    if num_traits::Zero::is_zero(&f.0) {
                        return None;
                    }
                    v = (v.0 / f.0, v.1 - f.1);
                } else {
                    return Some(v);
                }
            }
        }
        fn factor(&mut self) -> Option<(BigRational, i32)> {
            if self.eat(b'(') {
                let v = self.product()?;
                return self.eat(b')').then_some(v);
            }
            self.skip();
            let rest = std::str::from_utf8(&self.0[self.1..]).ok()?;
            if let Some(after) = rest.strip_prefix("core::f64::consts::PI") {
                self.1 = self.0.len() - after.len();
                return Some((BigRational::from_integer(1.into()), 1));
            }
            let len = rest
                .char_indices()
                .find(|&(i, c)| {
                    !(c.is_ascii_digit()
                        || c == '.'
                        || c == '_'
                        || ((c == 'e' || c == 'E') && i > 0)
                        || ((c == '-' || c == '+')
                            && i > 0
                            && matches!(rest.as_bytes()[i - 1], b'e' | b'E')))
                })
                .map_or(rest.len(), |(i, _)| i);
            if len == 0 {
                return None;
            }
            let lit: String = rest[..len].chars().filter(|c| *c != '_').collect();
            self.1 += len;
            let (mantissa, exp) = match lit.find(['e', 'E']) {
                Some(k) => (&lit[..k], lit[k + 1..].parse::<i32>().ok()?),
                None => (lit.as_str(), 0),
            };
            let (int, frac) = mantissa.split_once('.').unwrap_or((mantissa, ""));
            let digits: num_bigint::BigInt = format!("{int}{frac}").parse().ok()?;
            let exp = exp - frac.len() as i32;
            let ten = BigRational::from_integer(10.into());
            let scale = num_traits::Pow::pow(ten, exp);
            Some((BigRational::from_integer(digits) * scale, 0))
        }
    }
    let mut p = P(expr.trim().as_bytes(), 0);
    let v = p.product()?;
    p.skip();
    (p.1 == p.0.len()).then_some(v)
}

/// One number literal as an `Exact`.
fn exact_literal(lit: &str) -> String {
    let (mantissa, exp) = match lit.find(['e', 'E']) {
        Some(k) => (
            &lit[..k],
            lit[k + 1..]
                .parse::<i32>()
                .expect("a number literal's exponent"),
        ),
        None => (lit, 0),
    };
    let (int, frac) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let mut digits = format!("{int}{frac}");
    let mut exp = exp - frac.len() as i32;
    let trimmed = digits.trim_start_matches('0');
    digits = if trimmed.is_empty() {
        "0".into()
    } else {
        trimmed.to_string()
    };
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

/// `10^exp` as an `Exact`.
fn prefix_size(exp: i32) -> String {
    if exp >= 0 {
        format!("Exact::int(1{})", "0".repeat(exp as usize))
    } else {
        format!("Exact::dec(1, {exp})")
    }
}

/// `kilometres` -> `Kilometres`.
fn capitalized(s: &str) -> String {
    let mut c = s.chars();
    c.next()
        .map_or_else(String::new, |f| f.to_uppercase().chain(c).collect())
}

fn is_one(m: &str) -> bool {
    m.trim().parse::<f64>() == Ok(1.0)
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
qty_type = "SIQuantity"
unit_type = "SIUnit"
named = ["W"]

[dim]
fields = ["m", "kg", "s", "K"]

[dimensionless]
units = [
    {v = "Ratio", m = "1.0", s = "", l = ""},
    {v = "Decibel", m = "1.0", log = "10.0", per = "10.0", s = "dB", l = "decibels"},
]
"#;

    const THERMAL: &str = r#"
[[family]]
name = "thermal"

[[family.quantity]]
name = "Length"
dims = {m = 1}
units = [
    {v = "Metre", m = "1.0", s = "m", l = "metres"},
    {v = "Kilometre", m = "1000.0", s = "km", l = "kilometres"},
]

[[family.quantity]]
name = "Mass"
dims = {kg = 1}
units = [
    {v = "Kilogram", m = "1.0", s = "kg", l = "kilograms"},
    {v = "Gram", m = "0.001", s = "g", l = "grams"},
]

[[family.quantity]]
name = "Time"
dims = {s = 1}
units = [{v = "Second", m = "1.0", s = "s", l = "seconds"}]

[[family.quantity]]
name = "ThermodynamicTemperature"
dims = {K = 1}
units = [
    {v = "Kelvin", m = "1.0", s = "K", l = "kelvin"},
    {v = "Celsius", m = "1.0", z = "273.15", s = "°C", l = "degrees Celsius"},
]

[[family.quantity]]
name = "TemperatureInterval"
dims = {K = 1}
units = [
    {v = "Kelvin", m = "1.0", s = "K", l = "kelvin"},
    {v = "Rankine", m = "5.0 / 9.0", s = "°R", l = "degrees Rankine"},
    {v = "DegreeCelsius", m = "1.0", s = "°C", l = "degrees Celsius"},
]

[[family.quantity]]
name = "Velocity"
dims = {m = 1, s = -1}
units = [{v = "KilometrePerHour", m = "1000.0 / 3600.0", s = "km/h", l = "kilometres per hour"}]

[[family.quantity]]
name = "Power"
dims = {m = 2, kg = 1, s = -3}
units = [
    {v = "Watt", m = "1.0", s = "W", l = "watts"},
    {v = "DecibelMilliwatt", m = "0.001", log = "10.0", per = "10.0", s = "dBm", l = "decibels-milliwatt"},
]
"#;

    fn system() -> UnitSystem {
        toml::from_str(&format!("{BASE}{THERMAL}")).unwrap()
    }

    /// The thermal module's two halves, and the lib.
    fn generated() -> (Module, String) {
        let sys = system();
        let ctx = Context::new(&sys);
        let m = ctx.modules().into_iter().nth(1).unwrap();
        (gen_module(&ctx, &m), gen_lib_rs(&ctx))
    }

    /// The prefixes and base units are the lib's constants; a base unit is
    /// `BASE`, a prefix is not.
    #[test]
    fn base_units_are_in_the_lib() {
        let (m, lib) = generated();
        assert!(lib.contains(
            "pub const KILO: Unit<{ SIUnit::prefix(\"k\", Exact::int(1000)) }> = Unit::ONE;"
        ));
        assert!(lib.contains(
            "pub const MICRO: Unit<{ SIUnit::prefix(\"μ\", Exact::dec(1, -6)) }>"
        ));
        assert!(lib.contains(
            "pub const METRE: Unit<{ SIUnit::base(\"m\") }, BASE> = Unit::ONE;"
        ));
        assert!(lib.contains(
            "pub const KILOGRAM: Unit<{ SIUnit::base(\"kg\") }, BASE> = Unit::ONE;"
        ));
        assert!(!m.quantities.contains("pub const METRE:"));
        assert!(!m.quantities.contains("pub const KELVIN"));
        // The kilogram is a base unit, not a prefixed gram.
        assert!(m.quantities.contains(
            "pub const GRAM: Unit<{ SIUnit::scaled(\"g\", Exact::dec(1, -3), KILOGRAM.unit()) }> = Unit::ONE;"
        ));
    }

    /// The lib's doc names the system's own constants and types.
    #[test]
    fn the_lib_doc_names_the_systems_items() {
        let (_, lib) = generated();
        assert!(lib.contains("(`KILO`, `METRE`)"));
        assert!(lib.contains("(`Length<N>`, in the coherent unit by default)"));
        assert!(lib.contains("(`KILOMETRE`)"));
        assert!(lib.contains("`dynamic`: `DynQty`"));
    }

    /// A unit whose symbol is a prefix's and another unit's, of a size the
    /// prefix's times that unit's, is their product.
    #[test]
    fn prefixed_units_are_products() {
        let (m, _) = generated();
        assert!(m.quantities.contains(
            "pub const KILOMETRE: Unit<{ (KILO * METRE).unit() }> = Unit::ONE;"
        ));
    }

    /// A unit of size one with a symbol of its own is a named unit, not the
    /// coherent unit.
    #[test]
    fn named_units_of_size_one() {
        let (m, _) = generated();
        assert!(m.quantities.contains(
            "pub const DEGREE_CELSIUS: Unit<{ SIUnit::scaled(\"°C\", Exact::int(1), KELVIN.unit()) }> = Unit::ONE;"
        ));
    }

    /// A plain number reads as a number of each primitive type, scaled by
    /// its unit's size; an integer only for a whole size.
    #[test]
    fn plain_numbers_are_numbers() {
        let (_, lib) = generated();
        assert!(lib.contains(
            "impl<const U: SIUnit, const B: Base> From<SIQuantity<U, f64, B>> for f64\nwhere\n    Check<{ U.is_plain_number() }>: True,\n{\n    fn from(q: SIQuantity<U, f64, B>) -> f64 {\n        q.0 * const { <f64 as From<Exact>>::from(U.size()) }\n    }\n}"
        ));
        assert!(lib.contains(
            "impl<const U: SIUnit, const B: Base> From<SIQuantity<U, u8, B>> for u8\nwhere\n    Check<{ U.is_whole_plain_number() }>: True,"
        ));
    }

    /// A quantity's alias that several modules share is defined once, at
    /// the crate root.
    #[test]
    fn aliases_are_defined_once() {
        let sys: UnitSystem = toml::from_str(&format!(
            "{}{THERMAL}{}",
            BASE.replace("family_order = [\"thermal\"]", "family_order = [\"thermal\", \"other\"]"),
            r#"
[[family]]
name = "other"
[[family.quantity]]
name = "Length"
dims = {m = 1}
units = [{v = "Foot", m = "0.3048", s = "ft", l = "feet"}]
"#
        ))
        .unwrap();
        let ctx = Context::new(&sys);
        for m in ctx.modules().into_iter().skip(1) {
            let out = gen_module(&ctx, &m);
            assert!(!out.quantities.contains("pub type Length"));
        }
        let other = ctx.modules().into_iter().nth(2).unwrap();
        let out = gen_module(&ctx, &other);
        assert!(out.quantities.contains("// `Length` is `crate::Length`."));
        assert!(out.quantities.contains("pub const FOOT"));
        assert!(gen_lib_rs(&ctx).contains("pub type Length<N, const U: SIUnit = "));
    }

    /// The two conversions, generic over every unit: into a base unit of
    /// the same dimension and back.
    #[test]
    fn conversions_are_generic() {
        let (m, lib) = generated();
        assert!(lib.contains(
            "impl<const U: SIUnit, const V: SIUnit, N> From<SIQuantity<U, N, NOT_BASE>> for SIQuantity<V, N, BASE>\nwhere"
        ));
        assert!(lib.contains("    Check<{ U.same_dimension(V) }>: True,\n    Check<{ U.is_log() }>: Conversion<N>,\n"));
        assert!(lib.contains(
            "impl<const U: SIUnit, const V: SIUnit, N> From<SIQuantity<V, N, BASE>> for SIQuantity<U, N, NOT_BASE>\nwhere"
        ));
        assert!(!lib.contains("macro_rules!"));
        assert!(!m.quantities.contains("impl"));
    }

    /// Each quantity is an alias whose unit defaults to the product of the
    /// base units by its exponents.
    #[test]
    fn quantities_default_to_coherent_units() {
        let (m, _) = generated();
        let q = &m.quantities;
        assert!(q.contains(
            "pub type Length<N, const U: SIUnit = { METRE.unit() }, const B: Base = BASE> = SIQuantity<U, N, B>;"
        ));
        assert!(q.contains(
            "pub type Power<N, const U: SIUnit = { WATT.unit() }, const B: Base = BASE> = SIQuantity<U, N, B>;"
        ));
        // A unit of size one is the coherent unit, a base-unit product.
        assert!(q.contains(
            "pub const WATT: Unit<{ (METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND)).unit() }, BASE> = Unit::ONE;"
        ));
    }

    /// A unit is a constant, one of it, its unit in its type: scaled, with
    /// a zero, or on a log scale, of its quantity's coherent unit.
    #[test]
    fn units_are_constants() {
        let (m, _) = generated();
        let q = &m.quantities;
        assert!(q.contains(
            "pub const KILOMETRE_PER_HOUR: Unit<{ SIUnit::scaled(\"km/h\", Exact::int(1000) / Exact::int(3600), METRE_PER_SECOND.unit()) }> = Unit::ONE;"
        ));
        assert!(q.contains(
            "pub const CELSIUS: Unit<{ SIUnit::offset(\"°C\", Exact::int(1), Exact::dec(27315, -2), KELVIN.unit()) }> = Unit::ONE;"
        ));
        assert!(q.contains(
            "pub const DECIBEL_MILLIWATT: Unit<{ SIUnit::from_log(\"dBm\", Exact::int(10), Exact::int(10), Exact::dec(1, -3), WATT.unit()) }>"
        ));
        assert!(q.contains(
            "pub const RANKINE: Unit<{ SIUnit::scaled(\"°R\", Exact::int(5) / Exact::int(9), KELVIN.unit()) }>"
        ));
    }

    /// The plain number's levels: log units of `SIUnit::ONE`.
    #[test]
    fn levels_of_a_number() {
        let sys = system();
        let ctx = Context::new(&sys);
        let d = ctx.modules().into_iter().next().unwrap();
        let out = gen_module(&ctx, &d);
        assert!(out.quantities.contains(
            "pub const DECIBEL: Unit<{ SIUnit::from_log(\"dB\", Exact::int(10), Exact::int(10), Exact::int(1), RATIO.unit()) }>"
        ));
        assert!(out.quantities.contains("pub const RATIO: Unit<{ SIUnit::ONE }, BASE>"));
    }

    /// A listed symbol names its coherent unit in the lib's table.
    #[test]
    fn named_units_print_by_name() {
        let (_, lib) = generated();
        assert!(lib.contains(
            "static NAMED: [(SIUnit, &str); 1] = [\n    ((METRE * METRE * KILOGRAM / (SECOND * SECOND * SECOND)).unit(), \"W\"),\n];"
        ));
    }

    /// Each quantity's enum names its units, the coherent one the default,
    /// and converts to the unit type.
    #[test]
    fn unit_enums_name_their_units() {
        let (m, _) = generated();
        assert!(m.units.contains(
            "#[values(into(SIUnit))]\npub enum LengthUnit {\n    #[default]\n    #[value(METRE.unit())]\n    Metre,\n    #[value(KILOMETRE.unit())]\n    Kilometre,\n}"
        ));
        // No unit of `Velocity` is its coherent unit: the enum adds it.
        assert!(m.units.contains(
            "pub enum VelocityUnit {\n    #[default]\n    #[value(METRE_PER_SECOND.unit())]\n    MetrePerSecond,\n    #[value(KILOMETRE_PER_HOUR.unit())]\n    KilometrePerHour,\n}"
        ));
    }

    /// The quantity's operators are `ConstOps`'s, lifted over the unit's
    /// own; the crate depends on nothing of yadc.
    #[test]
    fn operators_are_const_ops() {
        let (_, lib) = generated();
        assert!(lib.contains(
            "#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, const_derive_more::ConstOps)]\n#[const_ops(from(Exact => f64, f32, "
        ));
        assert!(lib.contains(
            "pub struct SIQuantity<\n    #[ops(one = SIUnit::ONE)] const U: SIUnit,\n    N,\n    #[ops(one = BASE)] const B: Base,\n>(N);"
        ));
        assert!(!lib.contains("yadc::"));
        let cargo = gen_cargo_toml(&system());
        assert!(cargo.contains("const_derive_more = { path = \"../const_derive_more\" }\n"));
        assert!(cargo.contains("num-traits = { version = \"0.2\", default-features = false }\n"));
        assert!(!cargo.contains("yadc"));
        assert!(!UNIT.contains("yadc::"));
        assert!(!EXACT.contains("yadc::"));
    }

    /// The dynamic quantity records the unit and whether it is a base
    /// unit, and combines both with their own operators.
    #[test]
    fn dynamic_quantity() {
        let sys = system();
        let ctx = Context::new(&sys);
        let out = gen_dynamic_rs(&ctx, &[]);
        assert!(out.contains(
            "#[dynamic(SIQuantity<const U: SIUnit, f64, const B: Base> => { unit: U, is_base: B, value: .0 })]"
        ));
        assert!(out.contains(
            "#[dynamic(shared f64 => { unit: SIUnit::ONE, is_base: BASE, value: self })]"
        ));
        assert!(out.contains("    pub is_base: Base,\n"));
        assert!(!out.contains("#[dynamic(same)]"));
    }

    /// A unit is a size, a zero or a log scale: any other description is an
    /// error, not a field quietly left out.
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
        assert!(
            parsed
                .unwrap_err()
                .to_string()
                .contains("unknown field `by`")
        );
    }

    #[test]
    #[should_panic(
        expected = "a unit has a zero (`z`), a log scale (`log` and `per`, both), or neither"
    )]
    fn a_log_unit_has_both_base_and_per() {
        let sys: UnitSystem = toml::from_str(&format!(
            "{}{THERMAL}",
            BASE.replace("log = \"10.0\", per = \"10.0\"", "log = \"10.0\"")
        ))
        .unwrap();
        let ctx = Context::new(&sys);
        let d = ctx.modules().into_iter().next().unwrap();
        gen_module(&ctx, &d);
    }

    #[test]
    fn exact_literals_are_exact() {
        assert_eq!(exact_literals("273.15"), "Exact::dec(27315, -2)");
        assert_eq!(
            exact_literals("-40587.0 * 86400.0"),
            "-Exact::int(40587) * Exact::int(86400)"
        );
        assert_eq!(
            exact_literals("1.0 / core::f64::consts::PI"),
            "Exact::int(1) / Exact::PI"
        );
    }
}
