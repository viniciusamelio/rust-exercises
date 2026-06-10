use rand::RngExt;

#[test]
fn test_create_user_ok() {
    let result = result::create_user(1);
    assert!(result.is_ok());
    let user = result.ok().unwrap();
    assert_eq!(user.id, "1");
    assert_eq!(user.name, "XPTO");
}

#[test]
fn test_create_user_err() {
    let result = result::create_user(2);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.message, "error message");
    assert_eq!(err.data, 2);
}

#[test]
fn test_random() {
    let input: u64 = rand::rng().random();
    match result::create_user(input) {
        Ok(user) => {
            assert_eq!(user.id, format!("{}", input));
            assert_eq!(user.name, "XPTO");
        }
        Err(err) => {
            assert_eq!(err.message, "error message");
            assert_eq!(err.data, input);
        }
    }
}
