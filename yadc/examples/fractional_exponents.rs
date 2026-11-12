//! Fractional exponents: dimensions like volts per square-root hertz.
//!
//! Most quantities have whole-number exponents - m², s⁻¹, kg·m·s⁻². Noise
//! does not. Random electrical noise spreads its *power* evenly over
//! frequency, so power per hertz is the natural measure. Power goes with
//! voltage *squared*, so the voltage per hertz goes with the *square root* of
//! the bandwidth: a datasheet quotes an amplifier's noise as, say, 4 nV/√Hz,
//! and over a 10 kHz bandwidth that is 4 nV × √10000 = 400 nV. The unit V/√Hz
//! has the time exponent -5/2 (V has s⁻³, and 1/√Hz is s^½).
//!
//! With whole-number exponents that dimension cannot even be written down,
//! and `x.root::<2>()` refuses anything whose exponents are odd. Declaring the
//! dimension's fields as [`yadc::Exponent`] - a fraction - instead of `i8`
//! makes every root exist, and the rest of yadc works unchanged. The system
//! below needs no label (a `#[kind]` field), so it has none: only the
//! exponents decide.
//!
//! Run with: cargo run -p yadc --example fractional_exponents
#![feature(generic_const_exprs, min_adt_const_params, const_trait_impl)]
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

// --- SI with fractional exponents, for a noise budget -------------------------

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
    /// a unit of its own kind, so both directions are the step's: its
    /// `quantity` and `value`, the two methods of the `Unit` trait that every
    /// unit, constant or derived or written by hand, is.
    #[derive(Copy, Clone)]
    pub struct NanovoltPerRootHertz;
    impl Unit<NoiseDensity> for NanovoltPerRootHertz {
        fn quantity(&self, n: f64) -> NoiseDensity {
            nanovolt_per_root_hertz().quantity(n)
        }
        fn value(&self, q: NoiseDensity) -> f64 {
            nanovolt_per_root_hertz().value(q)
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

fn main() {
    use si::{Kelvin, Kilohertz, Kiloohm, Microvolt, NanovoltPerRootHertz};
    println!("== Noise budget (V/√Hz)");

    // The amplifier's datasheet: 4 nV/√Hz of input voltage noise.
    let amplifier = NanovoltPerRootHertz.quantity(4.0);

    // A 1 kΩ resistor at room temperature adds thermal (Johnson) noise of
    // density √(4·k·T·R). The units work out by themselves: J/K × K × Ω is
    // V²/Hz, and its square root is V/√Hz.
    let r = Kiloohm.quantity(1.0);
    let t = Kelvin.quantity(300.0);
    let resistor: si::NoiseDensity = (4.0 * si::BOLTZMANN * t * r).root::<2>();
    let nv = NanovoltPerRootHertz;
    println!("resistor (1 kΩ, 300 K): {:.2} {nv}", nv.value(resistor));

    // Independent noise sources add "in quadrature": square, add, square root.
    let total: si::NoiseDensity = (amplifier.powi::<2>() + resistor.powi::<2>()).root::<2>();
    println!("amplifier + resistor:   {:.2} {nv}", nv.value(total));

    // Over a bandwidth, density × √bandwidth is the RMS noise voltage.
    let bandwidth = Kilohertz.quantity(20.0);
    let rms: si::Voltage = total * bandwidth.root::<2>();
    println!(
        "over {} {Kilohertz}: {:.3} {Microvolt} rms",
        Kilohertz.value(bandwidth),
        Microvolt.value(rms)
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
}
