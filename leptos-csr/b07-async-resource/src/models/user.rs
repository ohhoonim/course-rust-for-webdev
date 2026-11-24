use gloo_net::http::Request;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

pub async fn fetch_users(query: String) -> Result<Vec<User>, String> {
    let mut users = Request::get("https://jsonplaceholder.typicode.com/users")
        .send()
        .await
        .map_err(|error| error.to_string())?
        .json::<Vec<User>>()
        .await
        .map_err(|error| error.to_string())?;

    let query = query.trim().to_lowercase();
    if !query.is_empty() {
        users.retain(|user| {
            user.name.to_lowercase().contains(&query) || user.email.to_lowercase().contains(&query)
        });
    }
    Ok(users)
}
