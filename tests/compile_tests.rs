#[test]
fn public_api_contracts() {
    let t = trybuild::TestCases::new();

    t.pass("tests/ui/pass/*.rs");
    t.compile_fail("tests/ui/fail/*.rs");
}