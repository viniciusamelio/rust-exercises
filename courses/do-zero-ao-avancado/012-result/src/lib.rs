pub struct User {
    pub id: String,
    pub name: String,
}

pub struct Failure<T> {
    pub message: String,
    pub data: T,
}

pub fn check_seed(seed: u64) -> Result<(), Failure<u64>> {
    if seed % 2 == 0 {
        return Err(Failure {
            message: "error message".to_string(),
            data: seed,
        });
    }
    return Ok(());
}

pub fn create_user(seed: u64) -> Result<User, Failure<u64>> {
    check_seed(seed)?;
    return Ok(User {
        id: format!("{}", seed),
        name: "XPTO".to_string(),
    });
}
