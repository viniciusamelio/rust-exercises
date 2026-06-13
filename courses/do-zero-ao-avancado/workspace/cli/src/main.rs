use core::create_user;

fn main() {
    let user = create_user(1, "John Doe".to_string(), "john@doe.com".to_string());
    println!("{:?}", user);
}
