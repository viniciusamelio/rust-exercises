#[derive(Debug, PartialEq)]
pub enum UserStatus {
    Active,
    Inactive,
}

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub email: String,
    pub status: UserStatus,
}

impl User {
    pub fn new(name: String, email: String, status: UserStatus) -> Self {
        Self {
            name,
            email,
            status,
        }
    }

    pub fn is_active(&self) -> bool {
        self.status == UserStatus::Active
    }

    pub fn set_status(&mut self, status: UserStatus) {
        self.status = status;
    }

    pub fn logout(mut self) {
        self.set_status(UserStatus::Inactive);
    }

    pub fn login(&mut self) -> &mut Self {
        self.set_status(UserStatus::Active);
        self
    }
}
