use agent_forge::http;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: u32,

    id: u32,

    title: String,

    completed: bool,
}

fn main() {}
