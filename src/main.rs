use actix_todo::{types, ClientTodoExt, Client};

const API_KEY: &str = "utoipa-rocks";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("todo_apikey", API_KEY.parse().unwrap());
    let default_client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()?;
    let client = Client::new_with_client("http://localhost:8080", default_client);

    let todo1 = client
        .create_todo()
        .body(types::Todo {
            id: 10,
            value: "Write a blog post".to_string(),
            checked: false,
        })
        .send()
        .await?;
    println!("todo1: {todo1:?}");

    let todo2 = client
        .create_todo()
        .body(&types::Todo {
            id: 20,
            value: "Attend a daily standup".to_string(),
            checked: false,
        })
        .send()
        .await?;
    println!("todo1: {todo2:?}");

    let todo_search = client
        .search_todos()
        .send()
        .await?
        .into_inner();
    println!("todo_search: {todo_search:?}");

    let todo_list = client.get_todos().send().await?;
    println!("todo list: {todo_list:?}");
    for todo in todo_list.into_inner() {
        client.delete_todo().id(todo.id).send().await?;
    }

    let todo_list = client.get_todos().send().await?;
    if todo_list.into_inner().is_empty() {
        println!("All todos deleted successfully");
    } else {
        println!("Failed to delete all todos");
    }

    Ok(())
}
