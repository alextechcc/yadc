//! The dynamic type on small families written here: `Qty<const D: Dim>(f64)`
//! (a const parameter), `Bag<F: Fruit>` (a type parameter), plain types
//! (`f64`, `Instant`, a fruit bowl). The derived `DynQty` sits next to
//! `HandQty`, the same type with every impl written by hand, and the two
//! must agree. Then the other lifted forms: `same` fields, methods and
//! getters, several static types in one struct, a generic dynamic type.
#![feature(adt_const_params, const_trait_impl, derive_const)]
#![feature(
    const_cmp,
    const_ops,
    const_convert,
    const_destruct,
    const_try,
    const_default
)]

use core::marker::{ConstParamTy, PhantomData};
use core::ops::{Add, Div, Mul, Neg, Sub};
use dynamickit::Dynamic;

// ===== A family with a const parameter ==========================================

/// Exponents of metre and second.
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Eq, ConstParamTy)]
pub struct Dim {
    pub m: i8,
    pub s: i8,
}

/// The dimension's own operators: a product and a quotient always exist, a
/// sum or difference only within one dimension.
const impl Mul for Dim {
    type Output = Dim;
    fn mul(self, r: Dim) -> Dim {
        Dim {
            m: self.m + r.m,
            s: self.s + r.s,
        }
    }
}
const impl Div for Dim {
    type Output = Dim;
    fn div(self, r: Dim) -> Dim {
        Dim {
            m: self.m - r.m,
            s: self.s - r.s,
        }
    }
}
const impl Add for Dim {
    type Output = Option<Dim>;
    fn add(self, r: Dim) -> Option<Dim> {
        if self == r { Some(self) } else { None }
    }
}
const impl Sub for Dim {
    type Output = Option<Dim>;
    fn sub(self, r: Dim) -> Option<Dim> {
        if self == r { Some(self) } else { None }
    }
}

/// What scaling does to a dimension: it stays, where it adds to itself.
#[allow(clippy::suspicious_arithmetic_impl)]
const impl Mul<f64> for Dim {
    type Output = Option<Dim>;
    fn mul(self, _: f64) -> Option<Dim> {
        self + self
    }
}
#[allow(clippy::suspicious_arithmetic_impl)]
const impl Div<f64> for Dim {
    type Output = Option<Dim>;
    fn div(self, _: f64) -> Option<Dim> {
        self + self
    }
}
const impl Neg for Dim {
    type Output = Option<Dim>;
    fn neg(self) -> Option<Dim> {
        self + self
    }
}

/// A trait of the test's own, with a method to lift: an integer power that
/// may not exist (a dimension's always does; a number's does not for 0^-1).
pub trait Pow: Sized {
    fn pow(self, n: i8) -> Option<Self>;
}
impl Pow for Dim {
    fn pow(self, n: i8) -> Option<Dim> {
        Some(Dim {
            m: self.m.checked_mul(n)?,
            s: self.s.checked_mul(n)?,
        })
    }
}
impl Pow for f64 {
    fn pow(self, n: i8) -> Option<f64> {
        let p = self.powi(n.into());
        if p.is_finite() { Some(p) } else { None }
    }
}

/// A getter trait: a value's dimension.
pub const trait HasDim {
    type D;
    fn dim(&self) -> Self::D;
}

/// The static family: the dimension is in the type.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Qty<const D: Dim>(f64);

pub const NUMBER: Dim = Dim { m: 0, s: 0 };
pub const METRE: Dim = Dim { m: 1, s: 0 };
pub const SECOND: Dim = Dim { m: 0, s: 1 };
pub const AREA: Dim = Dim { m: 2, s: 0 };
pub type Length = Qty<METRE>;
pub type Time = Qty<SECOND>;
pub type Area = Qty<AREA>;

// ===== The dynamic type, derived ==============================================

#[derive(Copy, Clone, PartialEq, Debug, Dynamic)]
#[dynamic(Qty<const D: Dim> => { dim: D, value: .0 })]
#[dynamic(shared f64 => { dim: NUMBER, value: self })]
#[dynamic(impl(
    Mul -> Self, Div -> Self, Add -> Option<Self>, Sub -> Option<Self>,
    Mul<f64> -> Option<Self>, Div<f64> -> Option<Self>, Neg -> Option<Self>,
    PartialOrd,
    Pow::pow(self, n: i8) -> Option<Self>,
    HasDim<D = Dim>::dim(&self) -> Dim = self.dim,
))]
pub struct DynQty {
    pub dim: Dim,
    pub value: f64,
}

// ===== The same type, by hand =================================================

/// Every impl `#[derive(Dynamic)]` writes for `DynQty`, written out: each is
/// a line or two.
pub mod by_hand {
    use super::{Dim, HasDim, NUMBER, Pow, Qty};
    use core::cmp::Ordering;
    use core::ops::{Add, Div, Mul, Neg, Sub};
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct HandQty {
        pub dim: Dim,
        pub value: f64,
    }

    // `Qty<const D: Dim> => { dim: D, value: .0 }`
    const impl<const D: Dim> From<Qty<D>> for HandQty {
        fn from(q: Qty<D>) -> Self {
            HandQty { dim: D, value: q.0 }
        }
    }
    const impl<const D: Dim> TryFrom<HandQty> for Qty<D> {
        type Error = HandQty;
        fn try_from(h: HandQty) -> Result<Self, HandQty> {
            if h.dim == D { Ok(Qty(h.value)) } else { Err(h) }
        }
    }

    // `f64 => { dim: NUMBER, value: self }`
    const impl From<f64> for HandQty {
        fn from(x: f64) -> Self {
            HandQty {
                dim: NUMBER,
                value: x,
            }
        }
    }
    const impl TryFrom<HandQty> for f64 {
        type Error = HandQty;
        fn try_from(h: HandQty) -> Result<Self, HandQty> {
            if h.dim == NUMBER { Ok(h.value) } else { Err(h) }
        }
    }

    // `Mul -> Self`, `Div -> Self`: every field by its own operator.
    const impl Mul for HandQty {
        type Output = HandQty;
        fn mul(self, r: HandQty) -> HandQty {
            HandQty {
                dim: self.dim * r.dim,
                value: self.value * r.value,
            }
        }
    }
    const impl Div for HandQty {
        type Output = HandQty;
        fn div(self, r: HandQty) -> HandQty {
            HandQty {
                dim: self.dim / r.dim,
                value: self.value / r.value,
            }
        }
    }
    // `Add -> Option<Self>`, `Sub -> Option<Self>`: `None` if any field's is.
    const impl Add for HandQty {
        type Output = Option<HandQty>;
        fn add(self, r: HandQty) -> Option<HandQty> {
            Some(HandQty {
                dim: (self.dim + r.dim)?,
                value: self.value + r.value,
            })
        }
    }
    const impl Sub for HandQty {
        type Output = Option<HandQty>;
        fn sub(self, r: HandQty) -> Option<HandQty> {
            Some(HandQty {
                dim: (self.dim - r.dim)?,
                value: self.value - r.value,
            })
        }
    }
    // `Mul<f64>`, `Div<f64>`, `Neg`: every field by its own operator.
    const impl Mul<f64> for HandQty {
        type Output = Option<HandQty>;
        fn mul(self, k: f64) -> Option<HandQty> {
            Some(HandQty {
                dim: (self.dim * k)?,
                value: self.value * k,
            })
        }
    }
    const impl Div<f64> for HandQty {
        type Output = Option<HandQty>;
        fn div(self, k: f64) -> Option<HandQty> {
            Some(HandQty {
                dim: (self.dim / k)?,
                value: self.value / k,
            })
        }
    }
    const impl Neg for HandQty {
        type Output = Option<HandQty>;
        fn neg(self) -> Option<HandQty> {
            Some(HandQty {
                dim: (-self.dim)?,
                value: -self.value,
            })
        }
    }
    // `PartialOrd`
    impl PartialOrd for HandQty {
        fn partial_cmp(&self, o: &HandQty) -> Option<Ordering> {
            if self.dim == o.dim {
                self.value.partial_cmp(&o.value)
            } else {
                None
            }
        }
    }
    // `Pow::pow(self, n: i8) -> Option<Self>`: the method on every field.
    impl Pow for HandQty {
        fn pow(self, n: i8) -> Option<HandQty> {
            Some(HandQty {
                dim: self.dim.pow(n)?,
                value: self.value.pow(n)?,
            })
        }
    }
    // `HasDim<D = Dim>::dim(&self) -> Dim = self.dim`: a getter.
    const impl HasDim for HandQty {
        type D = Dim;
        fn dim(&self) -> Dim {
            self.dim
        }
    }
}
use by_hand::HandQty;

// ===== Derived and hand-written agree =========================================

fn agree(d: DynQty, h: HandQty) -> bool {
    d.dim == h.dim && (d.value == h.value || (d.value.is_nan() && h.value.is_nan()))
}
fn agree_opt(d: Option<DynQty>, h: Option<HandQty>) -> bool {
    match (d, h) {
        (Some(d), Some(h)) => agree(d, h),
        (None, None) => true,
        _ => false,
    }
}

#[test]
fn derived_matches_by_hand() {
    let (l, t): (Length, Time) = (Qty(3.0), Qty(1.5));
    let (dl, dt) = (DynQty::from(l), DynQty::from(t));
    let (hl, ht) = (HandQty::from(l), HandQty::from(t));
    assert!(agree(dl, hl));
    assert!(agree(DynQty::from(2.5), HandQty::from(2.5)));
    assert!(agree(dl * dt, hl * ht));
    assert!(agree(dl / dt, hl / ht));
    assert!(agree_opt(dl + dl, hl + hl));
    assert!(agree_opt(dl + dt, hl + ht));
    assert!(agree_opt(dl - dl, hl - hl));
    assert!(agree_opt(dl - dt, hl - ht));
    assert!(agree_opt(dl * 2.0, hl * 2.0));
    assert!(agree_opt(dl / 2.0, hl / 2.0));
    assert!(agree_opt(-dl, -hl));
    assert!(agree_opt(dl.pow(2), hl.pow(2)));
    assert!(agree_opt(
        DynQty::from(0.0).pow(-1),
        HandQty::from(0.0).pow(-1)
    ));
    assert_eq!(dl.dim(), hl.dim());
    assert_eq!(dl.partial_cmp(&dt), hl.partial_cmp(&ht));
    assert_eq!(
        dl.partial_cmp(&(dl * 2.0).unwrap()),
        hl.partial_cmp(&(hl * 2.0).unwrap())
    );
    assert_eq!(Length::try_from(dl), Length::try_from(hl).map_err(|_| dl));
    assert_eq!(Time::try_from(dl).ok(), Time::try_from(hl).ok());
    assert_eq!(
        f64::try_from(dl / dl),
        f64::try_from(hl / hl).map_err(|_| dl)
    );
}

/// Generic code over a dynamic version of `T`: both directions are std.
fn round_trip<T, D: From<T>>(t: T) -> Result<T, D>
where
    T: TryFrom<D, Error = D>,
{
    T::try_from(D::from(t))
}

#[test]
fn from_and_try_from() {
    let l: Length = Qty(3.0);
    let d = DynQty::from(l);
    assert_eq!(
        d,
        DynQty {
            dim: METRE,
            value: 3.0
        }
    );
    assert_eq!(Length::try_from(d), Ok(l));
    assert_eq!(Time::try_from(d), Err(d));
    assert_eq!(round_trip::<Length, DynQty>(l), Ok(l));
    assert_eq!(round_trip::<f64, DynQty>(1.5), Ok(1.5));
    assert_eq!(round_trip::<Length, HandQty>(l), Ok(l));
}

#[test]
fn lifted_operators() {
    let (l, t) = (
        DynQty::from(Length::from_raw(3.0)),
        DynQty::from(Time::from_raw(2.0)),
    );
    assert_eq!(Area::try_from(l * l), Ok(Qty(9.0)));
    assert_eq!(l + t, None);
    assert_eq!(l / t * t, l);
    assert_eq!((l * 2.0).and_then(|x| x - l), Some(l));
    assert_eq!(l.partial_cmp(&t), None);
    assert!(l < (l * 2.0).unwrap());
}

impl<const D: Dim> Qty<D> {
    const fn from_raw(x: f64) -> Self {
        Qty(x)
    }
}

/// The operators and the getter are `const`.
const AREA_9: DynQty = {
    let l = DynQty::from(Qty::<METRE>(3.0));
    l * l
};
const AREA_DIM: Dim = AREA_9.dim();

#[test]
fn in_const() {
    assert_eq!(Area::try_from(AREA_9), Ok(Qty(9.0)));
    assert_eq!(AREA_DIM, AREA);
}

// ===== A family with a type parameter =========================================

/// What a fruit is: its kind, as a constant of the type.
pub trait Fruit {
    const KIND: FruitKind;
}

/// A fruit as a value: no arithmetic, only equality.
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug)]
pub struct FruitKind(u8);

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Apple;
impl Fruit for Apple {
    const KIND: FruitKind = FruitKind(1);
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Orange;
impl Fruit for Orange {
    const KIND: FruitKind = FruitKind(2);
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Bag<F: Fruit> {
    pub count: i64,
    pub fruit: PhantomData<F>,
}
const fn bag<F: Fruit>(count: i64) -> Bag<F> {
    Bag {
        count,
        fruit: PhantomData,
    }
}

/// A bowl of mixed fruit: a plain type with the same kind of facts, told
/// apart by its kind (no fruit has 0).
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Bowl {
    pub pieces: i64,
}
pub const MIXED: FruitKind = FruitKind(0);

#[derive(Copy, Clone, PartialEq, Debug, Dynamic)]
#[dynamic(Bag<F: Fruit> { fruit: PhantomData<F> } => { fruit: F::KIND, count: .count })]
#[dynamic(Bowl => { fruit: MIXED, count: .pieces })]
#[dynamic(impl(Add -> Option<Self>, Div -> Option<Self>, Mul<i64> -> Self, PartialOrd))]
pub struct AnyBag {
    #[dynamic(same)]
    pub fruit: FruitKind,
    pub count: i64,
}

#[test]
fn type_parameters() {
    let a = AnyBag::from(bag::<Apple>(250));
    let b = AnyBag::from(bag::<Apple>(100));
    let d = AnyBag::from(bag::<Orange>(100));
    assert_eq!(Bag::<Apple>::try_from((a + b).unwrap()), Ok(bag(350)));
    assert_eq!(a + d, None);
    assert_eq!(Bag::<Orange>::try_from(a), Err(a));
    assert_eq!(Bag::<Orange>::try_from(d * 3), Ok(bag(300)));
    let g = AnyBag::from(Bowl { pieces: 500 });
    assert_eq!(Bowl::try_from(g), Ok(Bowl { pieces: 500 }));
    assert_eq!(Bag::<Apple>::try_from(g), Err(g));
    assert_eq!(a + g, None);
    assert!(b < a);
    assert_eq!(a.partial_cmp(&d), None);
}

/// Operands that differ in a `same` field have no result, and their other
/// fields are never combined: no division by an orange's zero count.
#[test]
fn same_checked_first() {
    let a = AnyBag::from(bag::<Apple>(6));
    let none = AnyBag::from(bag::<Orange>(0));
    assert_eq!(a / none, None);
    assert_eq!(
        Bag::<Apple>::try_from((a / AnyBag::from(bag::<Apple>(3))).unwrap()),
        Ok(bag(2))
    );
}

// ===== Several static types, a fact with operators of its own ===================

/// A point in time: a plain type whose payload is two levels down.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Instant {
    pub since: Time,
}

/// A point of any dimension: a family whose payload is a named field.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point<const D: Dim> {
    pub offset: f64,
}

/// What a reading is: an amount, a point, an instant. Its own `-` says what
/// a difference is - two of one kind differ by an amount, and a point or an
/// instant less an amount is itself; scaling keeps it.
#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug)]
pub struct Tag(pub u8);
const impl Sub for Tag {
    type Output = Option<Tag>;
    fn sub(self, r: Tag) -> Option<Tag> {
        match (self.0, r.0) {
            (0, 0) | (1, 1) | (2, 2) => Some(Tag(0)),
            (1, 0) | (2, 0) => Some(self),
            _ => None,
        }
    }
}
const impl Mul<f64> for Tag {
    type Output = Tag;
    fn mul(self, _: f64) -> Tag {
        self
    }
}

/// Three static types, told apart by a tag.
#[derive(Copy, Clone, PartialEq, Debug, Dynamic)]
#[dynamic(Qty<const D: Dim>       => { tag: Tag(0), dim: D,      value: .0 })]
#[dynamic(Point<const D: Dim>     => { tag: Tag(1), dim: D,      value: .offset })]
#[dynamic(Instant { since: Time } => { tag: Tag(2), dim: SECOND, value: .since.0 })]
#[dynamic(impl(PartialOrd, Sub -> Option<Self>, Mul<f64> -> Option<Self>))]
pub struct Reading {
    pub tag: Tag,
    pub dim: Dim,
    pub value: f64,
}

#[test]
fn several_types() {
    let t0 = Reading::from(Instant { since: Qty(10.0) });
    let t1 = Reading::from(Instant { since: Qty(12.5) });
    let dt = (t1 - t0).unwrap();
    assert_eq!(Time::try_from(dt), Ok(Qty(2.5)));
    assert_eq!(
        Instant::try_from((t1 - dt).unwrap()).unwrap().since,
        Qty(10.0)
    );
    assert_eq!(dt - t0, None);
    let p = Reading::from(Point::<METRE> { offset: 4.0 });
    assert_eq!(Point::<METRE>::try_from((p * 2.0).unwrap()).unwrap().offset, 8.0);
    assert_eq!(Point::<SECOND>::try_from(p), Err(p));
    assert_eq!(t0.partial_cmp(&t1), Some(core::cmp::Ordering::Less));
    assert_eq!(dt.partial_cmp(&t1), None);
}

// ===== Tuple fields, several payload fields ===================================

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Step<const D: Dim, const F: u8> {
    pub x: f64,
    pub y: f64,
}

#[derive_const(PartialEq)]
#[derive(Copy, Clone, Debug, Dynamic)]
#[dynamic(Step<const D: Dim, const F: u8> => { 0: F, 1: .x, 2: D, 3: .y })]
#[dynamic(impl(Add -> Option<Self>, Mul<f64> -> Option<Self>, PartialOrd))]
pub struct AnyStep(#[dynamic(same)] u8, f64, Dim, f64);

#[test]
fn several_fields() {
    let s = AnyStep::from(Step::<METRE, 7> { x: 1.0, y: 2.0 });
    assert_eq!(s.0, 7);
    let twice = (s + s).unwrap();
    assert_eq!(
        Step::<METRE, 7>::try_from(twice),
        Ok(Step { x: 2.0, y: 4.0 })
    );
    assert_eq!(Step::<METRE, 7>::try_from((s * 3.0).unwrap()).unwrap().y, 6.0);
    assert!(Step::<METRE, 8>::try_from(s).is_err());
    assert!(s < twice);
}

// ===== A generic dynamic type =================================================

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Tagged<V, const T: u16>(pub V);

#[derive(Copy, Clone, PartialEq, Debug, Dynamic)]
#[dynamic(Tagged<V, const T: u16> => { tag: T, value: .0 })]
#[dynamic(impl(Add -> Option<Self>, Mul<V> -> Self, PartialOrd))]
pub struct AnyTagged<V = i32> {
    #[dynamic(same)]
    pub tag: u16,
    pub value: V,
}

#[test]
fn generic_dynamic_type() {
    let _: AnyTagged = AnyTagged::from(Tagged::<i32, 3>(0));
    let a = AnyTagged::from(Tagged::<i32, 1>(4));
    let b = AnyTagged::from(Tagged::<i32, 2>(5));
    assert_eq!((a + a).map(|x| x.value), Some(8));
    assert_eq!(a + b, None);
    assert_eq!(Tagged::<i32, 1>::try_from(a * 3), Ok(Tagged(12)));
    assert_eq!(Tagged::<i32, 2>::try_from(a), Err(a));
    assert!(a < AnyTagged::from(Tagged::<i32, 1>(9)));
}

// ===== Raw identifiers ========================================================

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Kinded<const K: u8> {
    pub r#type: f64,
}

/// A field named by a raw identifier, filled and lifted like any other.
#[derive(Copy, Clone, PartialEq, Debug, Dynamic)]
#[dynamic(Kinded<const K: u8> => { r#match: K, r#type: .r#type })]
#[dynamic(impl(Add -> Option<Self>, Neg -> Self))]
pub struct AnyKinded {
    #[dynamic(same)]
    pub r#match: u8,
    pub r#type: f64,
}

#[test]
fn raw_identifiers() {
    let a = AnyKinded::from(Kinded::<2> { r#type: 1.5 });
    assert_eq!(Kinded::<2>::try_from((a + a).unwrap()), Ok(Kinded { r#type: 3.0 }));
    assert_eq!(Kinded::<2>::try_from(-a), Ok(Kinded { r#type: -1.5 }));
    assert_eq!(Kinded::<3>::try_from(a), Err(a));
}

/// A `shared` mapping whose partner is decided by constants written two
/// ways (`UNIT` and `Unit(0)`): equal when compiled, so they are partners.
mod shared_by_constants {
    #[derive_const(PartialEq)]
    #[derive(Copy, Clone, Debug, Eq)]
    pub struct Unit(i8);
    pub const UNIT: Unit = Unit(0);
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct Ratio(pub f64);

    #[derive(Copy, Clone, PartialEq, Debug, dynamickit::Dynamic)]
    #[dynamic(shared f64 => { unit: UNIT, value: self })]
    #[dynamic(Ratio => { unit: Unit(0), value: .0 })]
    pub struct AnyRatio {
        unit: Unit,
        value: f64,
    }

    #[test]
    fn partners_by_value() {
        let r = AnyRatio::from(2.0);
        assert_eq!(Ratio::try_from(r), Ok(Ratio(2.0)));
        assert_eq!(f64::try_from(AnyRatio::from(Ratio(3.0))), Ok(3.0));
    }
}
