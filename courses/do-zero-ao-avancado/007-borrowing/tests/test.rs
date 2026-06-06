#[test]
fn test_copy_with() {
    let user = borrowing::User {
        name: String::from("John"),
        age: 30,
    };

    let copied_user = borrowing::copy_with(&user);

    assert_eq!(copied_user.name, user.name);
    assert_eq!(copied_user.age, user.age);
}

#[test]
fn test_update_name() {
    let mut user = borrowing::User {
        name: String::from("John"),
        age: 30,
    };
    assert_eq!(user.name, String::from("John"));

    borrowing::update_name(&mut user, String::from("Jane"));

    assert_eq!(user.name, String::from("Jane"));
}
