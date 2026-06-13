mod user;

pub use user::User;

pub fn create_user(id: u64, name: String, email: String) -> user::User {
    user::User::new(id, name, email)
}
