use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: u64,
    pub name: Option<String>,
    pub followers: Option<u64>,
    pub created_at: String,
    pub public_repos: Option<u64>,
}
