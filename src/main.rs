use agent_forge::http::{AsyncHttpClient, HttpClientConfig};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: u32,

    id: u32,

    title: String,

    completed: bool,
}

#[derive(Debug, Serialize)]
struct CreateTodo {
    #[serde(rename = "userId")]
    user_id: u32,

    title: String,

    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = AsyncHttpClient::new(HttpClientConfig::default())?;

    let todo: Todo = client
        .get_json("https://jsonplaceholder.typicode.com/todos/1")
        .await?;

    println!("GET Result: {todo:#?}");

    println!("Todo ID: {}, User ID: {}", todo.id, todo.user_id);

    let request = CreateTodo {
        user_id: 2,
        title: "Build Rust Agent Runtime".to_string(),
        completed: false,
    };

    let created: Todo = client
        .post_json("https://jsonplaceholder.typicode.com/todos", &request)
        .await?;

    println!("POST Result: {created:#?}");

    let text = client.get_text("https://httpbin.org/robots.txt").await?;

    println!("Text: {text}");

    Ok(())
}
