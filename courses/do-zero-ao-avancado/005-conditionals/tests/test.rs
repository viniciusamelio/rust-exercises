use conditionals::Status;

#[test]
fn age_greater_than_18() {
    let user = conditionals::User { age: 18 };
    assert!(conditionals::can_buy_whisky(user));
}

#[test]
fn age_less_than_18() {
    let user = conditionals::User { age: 17 };
    assert!(!conditionals::can_buy_whisky(user));
}

#[test]
fn user_online() {
    assert!(conditionals::is_online(Status::Online));
}

#[test]
fn user_offline() {
    assert!(!conditionals::is_online(Status::Offline));
}
