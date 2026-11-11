#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
use yadc_tests::*;

fn main() {
    let mut altitude = SEA_LEVEL + 100.0 * METRE;
    let mut length = METRE;
    length += SECOND; // AddSameDimension
    altitude += altitude; // AddNotTwoPoints
    altitude -= altitude; // SubInPlace: the difference is a length
    length -= SECOND; // SubSameDimension
    altitude *= 2.0; // ScaleNotPoint
    altitude /= 2.0;
    length *= METRE; // MulInPlace: the product is an area
    length /= SECOND; // DivInPlace
    altitude *= ONE; // MulExists: a point has no products
    let _ = -altitude; // `-` of a point: rustc reports no reason for a unary operator
    let _: Altitude = [altitude].into_iter().sum(); // `Sum` of points: AddNotTwoPoints
}
