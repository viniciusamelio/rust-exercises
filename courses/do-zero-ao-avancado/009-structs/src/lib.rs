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
