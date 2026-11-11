//! Every operator a static family refuses, and the message it fails with:
//! from a crate that uses a system defined elsewhere (`ui/use`), and from the
//! crate that defines it (`ui/define`).

#[test]
fn refusals() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/use/*.rs");
    t.compile_fail("tests/ui/define/*.rs");
}
