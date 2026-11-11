//! Fractional exponents: dimensions like volts per square-root hertz.
//!
//! Most quantities have whole-number exponents - m², s⁻¹, kg·m·s⁻². But some
//! common ones do not:
//!
//! - **Noise.** Random electrical noise spreads its *power* evenly over
//!   frequency, so power per hertz is the natural measure. Power goes with
//!   voltage *squared*, so the voltage per hertz goes with the *square root*
//!   of the bandwidth: a datasheet quotes an amplifier's noise as, say,
//!   4 nV/√Hz, and over a 10 kHz bandwidth that is 4 nV × √10000 = 400 nV.
//!   The unit V/√Hz has the time exponent -5/2 (V has s⁻³, and 1/√Hz is
//!   s^½).
//! - **CGS-Gaussian units.** The old centimetre-gram-second system has no
//!   separate unit of charge: charge is *defined* by Coulomb's law,
//!   F = q₁·q₂ / r², with no constant in front. Solving for q gives
//!   q = r·√F, so the unit of charge (the statcoulomb) is
//!   cm · √(g·cm/s²) = g^½ · cm^(3/2) · s⁻¹.
//!
//! With whole-number exponents those dimensions cannot even be written down,
//! and `x.root::<2>()` refuses anything whose exponents are odd. Declaring the
//! dimension's fields as [`yadc::Exponent`] - a fraction - instead of `i8`
//! makes every root exist, and the rest of yadc works unchanged. Neither
//! system below needs a label (a `#[kind]` field), so neither has one: only
//! the exponents decide.
//!
//! Run with: cargo run -p yadc --example fractional_exponents
#![feature(generic_const_exprs, min_adt_const_params, const_trait_impl)]
#![feature(fn_traits, unboxed_closures, const_destruct)]
#![feature(
    const_cmp,
    const_default,
    derive_const,
    const_ops,
    const_convert,
    const_try
)]
#![allow(incomplete_features)]

use yadc::{PowiAs, RootAs, Unit};

// --- Part 1: SI with fractional exponents, for a noise budget -----------------

mod si {
    use core::marker::ConstParamTy;
    use yadc::{Dimension, Exponent, PowiAs, RootAs, Unit};

    /// The exponent of each base unit is a fraction, not an integer.
    #[derive_const(PartialEq)]
    #[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]
    pub struct Dim {
        pub kg: Exponent,
        pub m: Exponent,
        pub s: Exponent,
        /// The ampere: printed as `A`, not by its field's name.
        #[dimension(symbol = "A")]
        pub a: Exponent,
        #[dimension(symbol = "K")]
        pub k: Exponent,
    }

    #[derive(Copy, Clone, PartialEq, PartialOrd, Debug, yadc::Quantity)]
    pub struct Qty<const D: Dim>(f64);

    const fn e(n: i8) -> Exponent {
        Exponent::int(n)
    }

    pub type Time = Qty<
        {
            Dim {
                s: e(1),
                ..Dim::DIMENSIONLESS
            }
        },
    >;
    pub type Length = Qty<
        {
            Dim {
                m: e(1),
                ..Dim::DIMENSIONLESS
            }
        },
    >;
    pub type Mass = Qty<
        {
            Dim {
                kg: e(1),
                ..Dim::DIMENSIONLESS
            }
        },
    >;
    pub type Current = Qty<
        {
            Dim {
                a: e(1),
                ..Dim::DIMENSIONLESS
            }
        },
    >;
    pub type Frequency = Qty<
        {
            Dim {
                s: e(-1),
                ..Dim::DIMENSIONLESS
            }
        },
    >;
    pub type Temperature = Qty<
        {
            Dim {
                k: e(1),
                ..Dim::DIMENSIONLESS
            }
        },
    >;
    /// V = kg·m²·s⁻³·A⁻¹.
    pub type Voltage = Qty<
        {
            Dim {
                kg: e(1),
                m: e(2),
                s: e(-3),
                a: e(-1),
                ..Dim::DIMENSIONLESS
            }
        },
    >;
    /// Ω = V/A.
    pub type Resistance = Qty<
        {
            Dim {
                kg: e(1),
                m: e(2),
                s: e(-3),
                a: e(-2),
                ..Dim::DIMENSIONLESS
            }
        },
    >;
    /// J/K, the unit of the Boltzmann constant.
    pub type HeatCapacity = Qty<
        {
            Dim {
                kg: e(1),
                m: e(2),
                s: e(-2),
                k: e(-1),
                ..Dim::DIMENSIONLESS
            }
        },
    >;
    /// V/√Hz: the voltage, per square root of the bandwidth. Note `s: -5/2`.
    pub type NoiseDensity = Qty<
        {
            Dim {
                kg: e(1),
                m: e(2),
                s: Exponent::new(-5, 2),
                a: e(-1),
                ..Dim::DIMENSIONLESS
            }
        },
    >;

    // The base constants: the only raw numbers.
    pub const SECOND: Time = Qty(1.0);
    pub const METRE: Length = Qty(1.0);
    pub const KILOGRAM: Mass = Qty(1.0);
    pub const AMPERE: Current = Qty(1.0);
    pub const KELVIN: Temperature = Qty(1.0);

    // Everything else, from them.
    pub const HERTZ: Frequency = SECOND.powi::<-1>();
    pub const JOULE_PER_KELVIN: HeatCapacity =
        KILOGRAM * METRE * METRE / (SECOND * SECOND * KELVIN);
    pub const VOLT: Voltage = KILOGRAM * METRE * METRE / (SECOND * SECOND * SECOND * AMPERE);
    pub const OHM: Resistance = VOLT / AMPERE;
    pub const KILOHERTZ: Frequency = 1e3 * HERTZ;
    pub const MICROVOLT: Voltage = 1e-6 * VOLT;
    pub const KILOOHM: Resistance = 1e3 * OHM;
    /// The Boltzmann constant (exact).
    pub const BOLTZMANN: HeatCapacity = 1.380649e-23 * JOULE_PER_KELVIN;
    /// V/√Hz. A root needs `powf`, which is not `const`, so this one is built
    /// as a square: (V/√Hz)² is V²/Hz, which needs no root.
    pub const VOLT_PER_ROOT_HERTZ_SQUARED: Qty<
        {
            Dim {
                kg: e(2),
                m: e(4),
                s: e(-5),
                a: e(-2),
                ..Dim::DIMENSIONLESS
            }
        },
    > = VOLT * VOLT / HERTZ;

    // Unit types, for printing: each names a constant, with its labels.
    #[derive(Copy, Clone, Unit)]
    #[unit("kHz", "kilohertz", of = Frequency, by = KILOHERTZ)]
    pub struct Kilohertz;

    #[derive(Copy, Clone, Unit)]
    #[unit("µV", "microvolts", of = Voltage, by = MICROVOLT)]
    pub struct Microvolt;

    #[derive(Copy, Clone, Unit)]
    #[unit("kΩ", "kiloohms", of = Resistance, by = KILOOHM)]
    pub struct Kiloohm;

    #[derive(Copy, Clone, Unit)]
    #[unit("K", "kelvin", of = Temperature, by = KELVIN)]
    pub struct Kelvin;

    /// One nV/√Hz: the root of a constant, taken at run time.
    fn nanovolt_per_root_hertz() -> NoiseDensity {
        1e-9 * VOLT_PER_ROOT_HERTZ_SQUARED.root::<2>()
    }

    /// Its step is only known at run time (a root is not `const`) - so this
    /// unit is written by hand. The step is a quantity, and every quantity is
    /// a unit of its own kind, so both directions are the step's. A unit
    /// written by hand is its two methods, `quantity` and `value`; the
    /// constants and the derived units are also called like functions.
    #[derive(Copy, Clone)]
    pub struct NanovoltPerRootHertz;
    impl Unit<NoiseDensity> for NanovoltPerRootHertz {
        fn quantity(&self, n: f64) -> NoiseDensity {
            nanovolt_per_root_hertz()(n)
        }
        fn value(&self, q: NoiseDensity) -> f64 {
            nanovolt_per_root_hertz()(q)
        }
    }
    impl core::fmt::Display for NanovoltPerRootHertz {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.pad(if f.alternate() {
                "nanovolts per root hertz"
            } else {
                "nV/√Hz"
            })
        }
    }
}

// --- Part 2: CGS-Gaussian electrostatics ----------------------------------------

mod cgs {
    use core::marker::ConstParamTy;
    use yadc::{Dimension, Exponent, Unit};

    /// Three base units - centimetre, gram, second - and no electrical one.
    #[derive_const(PartialEq)]
    #[derive(Copy, Clone, Debug, Eq, ConstParamTy, yadc::Dimension)]
    pub struct Dim {
        pub cm: Exponent,
        pub g: Exponent,
        pub s: Exponent,
    }

    #[derive(Copy, Clone, PartialEq, PartialOrd, Debug, yadc::Quantity)]
    pub struct Qty<const D: Dim>(f64);

    const fn e(n: i8) -> Exponent {
        Exponent::int(n)
    }

    pub type Length = Qty<
        {
            Dim {
                cm: e(1),
                ..Dim::DIMENSIONLESS
            }
        },
    >;
    /// The dyne: g·cm·s⁻².
    pub type Force = Qty<
        {
            Dim {
                g: e(1),
                cm: e(1),
                s: e(-2),
            }
        },
    >;
    /// The statcoulomb: g^½·cm^(3/2)·s⁻¹ - whatever makes F = q₁q₂/r² come out
    /// in dynes.
    pub type Charge = Qty<
        {
            Dim {
                g: Exponent::new(1, 2),
                cm: Exponent::new(3, 2),
                s: e(-1),
            }
        },
    >;

    pub type Mass = Qty<
        {
            Dim {
                g: e(1),
                ..Dim::DIMENSIONLESS
            }
        },
    >;
    pub type Time = Qty<
        {
            Dim {
                s: e(1),
                ..Dim::DIMENSIONLESS
            }
        },
    >;

    // The base constants. The statcoulomb is defined by Coulomb's law,
    // q = r·√F, so it is the root of g·cm³·s⁻² - and a root needs `powf`,
    // which is not `const`. So it is given here as the third constant (its
    // value checked in `main`): in CGS-Gaussian units it is coherent.
    pub const CENTIMETRE: Length = Qty(1.0);
    pub const GRAM: Mass = Qty(1.0);
    pub const SECOND: Time = Qty(1.0);
    pub const STATCOULOMB: Charge = Qty(1.0);

    pub const DYNE: Force = GRAM * CENTIMETRE / (SECOND * SECOND);
    /// The elementary charge in CGS-Gaussian units.
    pub const ELEMENTARY_CHARGE: Charge = 4.803_204_712_570_263e-10 * STATCOULOMB;

    #[derive(Copy, Clone, Unit)]
    #[unit("cm", "centimetres", of = Length, by = CENTIMETRE)]
    pub struct Centimetre;

    #[derive(Copy, Clone, Unit)]
    #[unit("dyn", "dynes", of = Force, by = DYNE)]
    pub struct Dyne;

    /// A choice of two units of charge.
    #[derive(Copy, Clone, Unit)]
    #[unit(of = Charge)]
    pub enum ChargeUnit {
        #[unit("statC", "statcoulombs", by = STATCOULOMB)]
        Statcoulomb,
        #[unit("e", "elementary charges", by = ELEMENTARY_CHARGE)]
        ElementaryCharge,
    }
}

fn main() {
    // --- Part 1 -------------------------------------------------------------------
    use si::{Kelvin, Kilohertz, Kiloohm, Microvolt, NanovoltPerRootHertz};
    println!("== Noise budget (V/√Hz)");

    // The amplifier's datasheet: 4 nV/√Hz of input voltage noise.
    let amplifier = NanovoltPerRootHertz.quantity(4.0);

    // A 1 kΩ resistor at room temperature adds thermal (Johnson) noise of
    // density √(4·k·T·R). The units work out by themselves: J/K × K × Ω is
    // V²/Hz, and its square root is V/√Hz.
    let r = Kiloohm(1.0);
    let t = Kelvin(300.0);
    let resistor: si::NoiseDensity = (4.0 * si::BOLTZMANN * t * r).root::<2>();
    let nv = NanovoltPerRootHertz;
    println!("resistor (1 kΩ, 300 K): {:.2} {nv}", nv.value(resistor));

    // Independent noise sources add "in quadrature": square, add, square root.
    let total: si::NoiseDensity = (amplifier.powi::<2>() + resistor.powi::<2>()).root::<2>();
    println!("amplifier + resistor:   {:.2} {nv}", nv.value(total));

    // Over a bandwidth, density × √bandwidth is the RMS noise voltage.
    let bandwidth = Kilohertz(20.0);
    let rms: si::Voltage = total * bandwidth.root::<2>();
    println!(
        "over {} {Kilohertz}: {:.3} {Microvolt} rms",
        Kilohertz(bandwidth),
        Microvolt(rms)
    );
    // A quantity's dimension is its type's const parameter.
    fn si_dim<const D: si::Dim>(_: si::Qty<D>) -> si::Dim {
        D
    }
    println!(
        "time exponent of V/√Hz: {} (an Exponent, printed as a fraction)",
        si_dim(total).s
    );
    // The whole dimension prints with fractional superscripts.
    println!("dimension of V/√Hz: {}", si_dim(total));

    // --- Part 2 -------------------------------------------------------------------
    use cgs::{Centimetre, ChargeUnit, Dyne};
    println!("\n== CGS-Gaussian electrostatics");

    // Two charges of 1 statC, 1 cm apart, repel with 1 dyn - that is the
    // definition. Coulomb's law has no constant in this system.
    let q1 = ChargeUnit::Statcoulomb(1.0);
    let q2 = ChargeUnit::Statcoulomb(1.0);
    let d = Centimetre(1.0);
    let f: cgs::Force = q1 * q2 / d.powi::<2>();
    println!("1 statC, 1 statC, 1 cm apart: {} {Dyne}", Dyne(f));

    // Two electrons 1 Ångström (1e-8 cm) apart.
    let e = ChargeUnit::ElementaryCharge(1.0);
    let a = Centimetre(1e-8);
    let fe: cgs::Force = e * e / a.powi::<2>();
    println!("two electrons 1 Å apart: {:.3e} {Dyne}", Dyne(fe));

    // And back: the charge that gives a force F at distance r is r·√F, a
    // statcoulomb again - its exponents are halves.
    let q: cgs::Charge = d * f.root::<2>();
    let statc_unit = ChargeUnit::Statcoulomb;
    println!("r·√F = {} {statc_unit}", statc_unit(q));
    fn cgs_dim<const D: cgs::Dim>(_: cgs::Qty<D>) -> cgs::Dim {
        D
    }
    let dim = cgs_dim(q);
    // ... and the statcoulomb constant is that root: √(dyn·cm²).
    let statc: cgs::Charge = (cgs::DYNE * cgs::CENTIMETRE * cgs::CENTIMETRE).root::<2>();
    println!("√(dyn·cm²) = {} {statc_unit}", statc_unit(statc));
    println!("statC = g^{} · cm^{} · s^{}", dim.g, dim.cm, dim.s);
}
