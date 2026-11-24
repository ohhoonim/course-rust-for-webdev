use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub body: String,
}

pub async fn fetch_posts() -> Result<Vec<Post>, String> {
    Request::get("https://jsonplaceholder.typicode.com/posts?_limit=5")
        .send()
        .await
        .map_err(|error| error.to_string())?
        .json::<Vec<Post>>()
        .await
        .map_err(|error| error.to_string())
}

pub async fn fetch_maybe_bad(use_bad_url: bool) -> Result<String, String> {
    let url = if use_bad_url {
        "https://jsonplaceholder.typicode.invalid/posts"
    } else {
        "https://jsonplaceholder.typicode.com/posts/1"
    };

    Request::get(url)
        .send()
        .await
        .map_err(|error| error.to_string())?
        .text()
        .await
        .map_err(|error| error.to_string())
}
