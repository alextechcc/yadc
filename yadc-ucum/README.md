# yadc-ucum

[UCUM](https://ucum.org), the Unified Code for Units of Measure, as a yadc
unit system: generated from the official `ucum-essence.xml` by
[`yadc-codegen`](../yadc-codegen/) with [`ucum.toml`](../yadc-codegen/ucum.toml).

- **UCUM's base units are the axes.** The dimension `Dim` has one exponent
  per UCUM base unit: `m`, `s`, `g`, `rad`, `K`, `C` (the coulomb) and `cd`.
  The mole is a number, as UCUM defines it (6.02214076 × 10²³), and an
  ampere is a coulomb per second.
- **Every unit is a type, its value exact.** A unit is a `UcumUnit<D, C>`,
  its definition `C` a `UcumUnitConsts` - the XML's value read exactly as
  the unit's size in coherent units, a zero, its code and its name:
  `intcust::units::Inch` is 0.0254 m (`Exact::dec(254, -4)`), the US survey
  inch `us::units::Inch` 100/3937 m. It counts in any number type the exact
  size converts to: `Inch {}.quantity(3.0_f64)` builds, `Inch {}.value(d)`
  reads.
- **Units are UCUM codes.** A unit prints its case-sensitive UCUM code
  (`[in_i]`, `Cel`), `{:#}` its name; each quantity's unit enum prints and
  parses them (`"[in_i]".parse::<LengthUnit>()`) and counts in any number
  type (`LengthUnit::Inch.quantity(3.0_f64)`).
- **One module per UCUM class**, each a Cargo feature: `si`, `heat`,
  `iso1000`, `intcust`, `cgs`, `infotech`, `us`, `british`, `clinical`,
  `constants`, `misc`. A family module (`intcust`) holds its quantity
  aliases and its `units`; its unit enums are in the dynamic half, beside
  the dynamic quantity `DynQty` and the dynamic unit `DynamicUcumUnit` -
  what is chosen at run time, a dimension or a unit:
  `yadc_ucum::dynamic::intcust::LengthUnit`. Special units (logarithms, pH)
  and arbitrary ones (international units) are left out; Celsius and
  Fahrenheit are units with a zero (`heat::units::Celsius`, `Fahrenheit`;
  `KelvinScale` beside them).

## Updating

The source is pinned to a UCUM release by its tag in `ucum.toml`. To move to
a new release, change the tag, then regenerate and review the diff:

```bash
cargo run -p yadc-codegen -- yadc-codegen/ucum.toml -o yadc-ucum
```

The generated `src/` is never edited by hand; `tests/` and this README are.
