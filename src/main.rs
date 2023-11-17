pub use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    progenitor::generate_api!("./docs/keeper.json");

    let client = Client::new("localhost");
    let foo = client.ping(None);
    if let Ok(o) = foo.await {
        println!("Ping: {:?}", o);
    }
    Ok(())
}
