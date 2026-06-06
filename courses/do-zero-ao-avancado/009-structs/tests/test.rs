#[test]
fn test_new_user() {
    let mut user = structs::User::new(String::from("nickname"), String::from("email"));
    println!("Name is: {:?}", user);
    assert_eq!(user.nickname, String::from("nickname"));
    assert_eq!(user.email, String::from("email"));

    user.nickname = String::from("Vini");
    assert_eq!(user.nickname, String::from("Vini"));
}

#[test]
fn test_make_rect() {
    let rect = structs::make_rect(structs::Width(10), structs::Height(20));
    assert_eq!(rect, structs::Rect(structs::Width(10), structs::Height(20)));
}
