pub struct User {
    pub name: String,
    pub age: u32,
}

pub fn copy_with(user: &User) -> User {
    User {
        name: user.name.clone(),
        age: user.age,
    }
}

pub fn update_name(user: &mut User, new_name: String) {
    user.name = new_name;
}
