# yadc-ucum

[UCUM](https://ucum.org), the Unified Code for Units of Measure, as a
[yadc](../) unit system: generated from the official `ucum-essence.xml` by
[`yadc-codegen`](../yadc-codegen/) with [`ucum.toml`](../yadc-codegen/ucum.toml).

- **UCUM's base units are the base units:** `m`, `s`, `g`, `rad`, `K`, `C`
  (the coulomb) and `cd` (`METER`, `SECOND`, `GRAM`, ...). The mole is a
  number, as UCUM defines it (6.02214076 × 10²³), and an ampere a coulomb
  per second.
- **Every unit is a constant, its value exact.** `intcust::INCH` is
  0.0254 m, the US survey inch `us::INCH` 100/3937 m. A quantity is
  `Qty<U, N, B>`, its unit a `UcumUnit`, and a quantity alias names its
  base unit by default (an alias several modules share is at the crate
  root: `yadc_ucum::Length`):

  ```rust
  use yadc_ucum::dynamic::intcust::LengthUnit;
  use yadc_ucum::heat::{CELSIUS, ThermodynamicTemperature};
  use yadc_ucum::intcust::INCH;
  use yadc_ucum::{Length, METER, UcumUnit};

  let d: Length<f64> = (3.0_f64 * INCH).into();                         // 0.0762 m
  let t: ThermodynamicTemperature<f64> = (25.0_f64 * CELSIUS).into();   // 298.15 K
  let u: LengthUnit = "[in_i]".parse().unwrap();
  let inches = UcumUnit::from(u).from_base(f64::from(d / METER));       // 3.0
  let same = f64::from(d / INCH);                                        // 3.0
  ```

- **Units are UCUM codes.** A unit prints, and each quantity's unit enum
  prints and parses, its case-sensitive UCUM code (`[in_i]`, `Cel`).
- **One module per UCUM class**, each a Cargo feature: `si`, `heat`,
  `iso1000`, `intcust`, `cgs`, `infotech`, `us`, `british`, `clinical`,
  `constants`, `misc`; `dimensionless` is always present. Unit enums are in
  `dynamic::<module>`, beside the dynamic quantity `DynQty`. Special units
  (logarithms, pH) and arbitrary ones (international units) are left out;
  Celsius and Fahrenheit are units with a zero (`heat::CELSIUS`,
  `heat::FAHRENHEIT`).

## Updating

The source is pinned to a UCUM release by its tag in `ucum.toml`. To move to
a new release, change the tag, then regenerate and review the diff:

```bash
cargo run -p yadc-codegen -- yadc-codegen/ucum.toml -o yadc-ucum
```

The generated `src/` is never edited by hand; `tests/` and this README are.
