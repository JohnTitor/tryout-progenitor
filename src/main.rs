pub use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    progenitor::generate_api!("./docs/openapi.json");

    let client = Client::new("localhost:8080");
    let foo = client.api_version();
    println!("{foo}");
    Ok(())
}
