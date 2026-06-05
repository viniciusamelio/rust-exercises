pub mod types;
pub use types::Status;

pub struct User {
    pub age: u32,
}

pub fn can_buy_whisky(user: User) -> bool {
    match user {
        User { age } if age < 18 => false,
        _ => true,
    }
}

pub fn is_online(status: types::Status) -> bool {
    matches!(status, types::Status::Online)
}

pub fn get_status_label(status: types::Status) -> &'static str {
    match status {
        types::Status::Online => "Online",
        types::Status::Offline => "Offline",
    }
}
