use statekit::StateError;


fn assert_std_error<T: std::error::Error>() {}

#[test]
fn verify_std_error_compliance() {
    assert_std_error::<StateError>();
}
