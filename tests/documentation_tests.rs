#[test]
fn documentation_examples_compile() {
    let t = trybuild::TestCases::new();

    t.pass("tests/ui/pass/docs/*.rs");
    t.compile_fail("tests/ui/fail/docs/*.rs");
}
