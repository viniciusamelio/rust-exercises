#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: String,
}

impl Post {
    pub fn new(id: String, title: String, content: String) -> Self {
        Self { id, title, content }
    }
}
