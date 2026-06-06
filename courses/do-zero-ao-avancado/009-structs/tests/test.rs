#[test]
fn test_new_user() {
    let mut user = structs::User::new(String::from("nickname"), String::from("email"));
    println!("Name is: {:?}", user);
    assert_eq!(user.nickname, String::from("nickname"));
    assert_eq!(user.email, String::from("email"));

    user.nickname = String::from("Vini");
    assert_eq!(user.nickname, String::from("Vini"));
}
