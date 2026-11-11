//! What the derives refuse, and how: each file in `ui/` must fail to compile
//! with the message beside it.

#[test]
fn refusals() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
