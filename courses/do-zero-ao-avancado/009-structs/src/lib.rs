#[derive(Debug, PartialEq)]
pub struct User {
    pub nickname: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    pub fn new(nickname: String, email: String) -> Self {
        Self {
            nickname,
            email,
            created_at: chrono::Utc::now(),
        }
    }
}

impl Default for User {
    fn default() -> Self {
        Self::new(String::new(), String::new())
    }
}

// tuple struct

#[derive(Debug, PartialEq)]
pub struct Width(pub u32);

#[derive(Debug, PartialEq)]
pub struct Height(pub u32);

#[derive(Debug, PartialEq)]
pub struct Rect(pub Width, pub Height);

pub fn make_rect(width: Width, height: Height) -> Rect {
    Rect(width, height)
}
