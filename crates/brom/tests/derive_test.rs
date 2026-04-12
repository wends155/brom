#[test]
fn compile_tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/fixtures/pass_*.rs");
    t.compile_fail("tests/fixtures/fail_*.rs");
}
