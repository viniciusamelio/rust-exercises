mod user;
use user::User;

async fn fetch_user(username: &str) -> Result<User, reqwest::Error> {
    let url: String = format!("https://api.github.com/users/{}", username);
    let user = reqwest::Client::new()
        .get(&url)
        .header("user-agent", "rust")
        .header("content-type", "application/json")
        .send()
        .await?
        .json::<User>()
        .await?;
    Ok(user)
}
#[tokio::main]
async fn main() {
    let maybe_user = fetch_user("viniciusamelio").await;
    match maybe_user {
        Ok(user) => println!(
            "id: {:?}\nname: {:?}\nfollowers: {:?}\ncreated_at: {}\npublic_repos: {:?}",
            user.id, user.name, user.followers, user.created_at, user.public_repos
        ),
        Err(e) => println!("Error: {}", e),
    }
}
