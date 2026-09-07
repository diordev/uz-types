#[test]
fn public_api_contracts() {
    let tests = trybuild::TestCases::new();

    tests.compile_fail("tests/ui/gender_requires_wildcard.rs");
    tests.compile_fail("tests/ui/secret_no_display.rs");

    #[cfg(not(feature = "serialize-secrets"))]
    tests.compile_fail("tests/ui/secret_no_serialize.rs");
}
