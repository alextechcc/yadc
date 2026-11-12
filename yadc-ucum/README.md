# yadc-ucum

[UCUM](https://ucum.org), the Unified Code for Units of Measure, as a yadc
unit system: generated from the official `ucum-essence.xml` by
[`yadc-codegen`](../yadc-codegen/) with [`ucum.toml`](../yadc-codegen/ucum.toml).

- **UCUM's base units are the axes.** The dimension `Dim` has one exponent
  per UCUM base unit: `m`, `s`, `g`, `rad`, `K`, `C` (the coulomb) and `cd`.
  The mole is a number, as UCUM defines it (6.02214076 × 10²³), and an
  ampere is a coulomb per second.
- **Every value is exact.** Each constant is the XML's definition, read
  exactly: `intcust::consts::INCH` is `METRE * Exact::dec(254, -4)`, the US
  survey inch `METRE * (Exact::int(100) / Exact::int(3937))`. A number type
  is chosen where a value is converted, with `From`: `let d: Length<f64> =
  INCH.into();`.
- **Units are UCUM codes.** Each family's unit enums print and parse a unit
  by its case-sensitive UCUM code (`"[in_i]".parse::<LengthUnit>()`, `Cel`),
  `{:#}` by its name, and count in any number type the constants convert to
  (`LengthUnit::Inch.quantity(3.0)`).
- **One module per UCUM class**, each a Cargo feature: `si`, `heat`,
  `iso1000`, `intcust`, `cgs`, `infotech`, `us`, `british`, `clinical`,
  `constants`, `misc`. A family module (`intcust`) holds its quantity
  aliases and `consts`; its unit enums are in the dynamic half, beside the
  dynamic quantity - what is chosen at run time, a dimension or a unit:
  `yadc_ucum::dynamic::intcust::LengthUnit`. Special units (logarithms, pH) and arbitrary ones
  (international units) are left out; Celsius and Fahrenheit are scales.

## Updating

The source is pinned to a UCUM release by its tag in `ucum.toml`. To move to
a new release, change the tag, then regenerate and review the diff:

```bash
cargo run -p yadc-codegen -- yadc-codegen/ucum.toml -o yadc-ucum
```

The generated `src/` is never edited by hand; `tests/` and this README are.
