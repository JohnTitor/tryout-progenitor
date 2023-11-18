pub use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    progenitor::generate_api!("./docs/openapi.json");

    let client = Client::new("http://localhost:8080");
    let api_version = client.api_version();
    let todo_list = client.get_todos().await?;
    println!("api version: {api_version}\ntodo list: {todo_list:?}");
    Ok(())
}
