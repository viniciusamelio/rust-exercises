#[test]
fn test_user_new() {
    let user = implementations::User::new(
        "John Doe".to_string(),
        "john@example.com".to_string(),
        implementations::UserStatus::Active,
    );
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.email, "john@example.com");
    assert_eq!(user.status, implementations::UserStatus::Active);
}

#[test]
fn test_user_is_active() {
    let user = implementations::User::new(
        "John Doe".to_string(),
        "john@example.com".to_string(),
        implementations::UserStatus::Active,
    );
    assert!(user.is_active());
}

#[test]
fn test_user_set_status() {
    let mut user = implementations::User::new(
        "John Doe".to_string(),
        "john@example.com".to_string(),
        implementations::UserStatus::Active,
    );
    user.set_status(implementations::UserStatus::Inactive);
    assert!(!user.is_active());
}

#[test]
fn test_user_logout() {
    let user = implementations::User::new(
        "John Doe".to_string(),
        "john@example.com".to_string(),
        implementations::UserStatus::Active,
    );
    user.logout();
}
