pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

pub fn get_user(id: &str) -> Option<User> {
    if id.is_empty() {
        return None;
    }

    let users = vec![
        User {
            id: "1".to_string(),
            name: "John Doe".to_string(),
            email: "john@doe.com".to_string(),
        },
        User {
            id: "2".to_string(),
            name: "Jane Doe".to_string(),
            email: "jane@doe.com".to_string(),
        },
    ];

    users.into_iter().find(|user| user.id == id)
}
