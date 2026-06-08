#[test]
fn test_get_valid_user() {
    let user = options::get_user("1");
    match user {
        Some(user) => assert_eq!(user.id, "1"),
        None => panic!("Expected Some"),
    }
}

#[test]
fn test_get_empty_user_id() {
    let user = options::get_user("");

    if let Some(_) = user {
        panic!("Expected None");
    }
    assert!(user.is_none());
}

#[test]
fn test_get_invalid_user() {
    let maybe_user = options::get_user("99");
    assert!(maybe_user.is_none());
}
