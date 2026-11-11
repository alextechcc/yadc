# yadc-derive

Internal proc-macro crate for [`yadc`](../yadc/). **Don't depend on this directly** — use `yadc`, which re-exports every derive.

| Derive | On | Writes |
|---|---|---|
| `Dimension` | your dimension struct: exponent axes (integers, or `yadc::Exponent` for fractions) and an optional `#[kind]` field - or your kind enum (`#[default]` the ordinary kind, `#[kind(affine)]` the points, `#[kind(log)]` the levels and gains, `#[kind(add(X) = Y, sub(X) = Y)]` rules for a label of your own, `= invalid` for no result) | `yadc::Dimension` (and, on the enum, `yadc::Label`) and the const std operators and traits it asks for: `*` / `/` (total, or `Option` with a `#[kind]` field), `+` / `-` returning `Option`, `PartialPowi`, `PartialRoot`, `DIMENSIONLESS`, `plain`, scaling by every primitive number type (`Mul<n>` / `Div<n> -> Option<Self>`); plus `Display` (`kg·m⁻¹·s⁻²`, `K{Affine}`) |
| `Quantity` | a static family `Qty<const D: Dim>(f64)` or `Qty<const D: Dim, N = f64>(N)` | `Quantity` and `Unit<Self, N>` for every member that can be scaled, the cross-type operators wherever the dimension's own has a result, the refusals of `yadc::gates` (so a refused operator fails with its reason); and the extras - `Display`, `Neg`, the compound assignments, `Powi`, `Root`, `Inv`, `Sum`, `Cast`, `From`, `Fn` (a unit is called: `KILOMETRE(3.0)`, `FOOT(d)`) - all by default, exactly those listed with `#[quantity(impl(..))]` |
| `Unit` | an enum of units of yours (or a struct that is one unit) | `yadc::Unit<Q, N>` per `of = Q`, each unit naming the unit value it is with `by = U` - a quantity constant, a unit type of yours (a scale `Scale(zero, step)`), or a field marked `#[unit(by)]` - and optionally `via = P` (the value measures `P`, converted with std `From`); plus `Display` (`{}` the symbol, `{:#}` the name), `FromStr`, and std `Fn` (a unit value is called: `unit(25.0)`, `unit(q)`), unless `#[unit(impl(..))]` lists fewer. `const` unless `non_const` |
| `UcumKind`, `UcumSerialize`, `UcumDeserialize` | label enum / dimension struct | a UCUM string wire format for dimensions (`"m.kg.s-2"`, `"rad"`, `"K{Affine}"`) |

Each derive writes its own trait, its supertraits and the extras it lists - trait impls and nothing else: no inherent methods or constants on your own type, and no hidden coupling to other types beyond those named in attributes - so anything it produces can be written by hand.

The full reference (attributes, exactly what is written, required crate features) is in the doc comments of each derive in [`src/lib.rs`](src/lib.rs), rendered by `cargo doc -p yadc`.
