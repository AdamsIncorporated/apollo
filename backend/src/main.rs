mod requests;
use requests::auth::client::create_yahoo_finance_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match create_yahoo_finance_client().await {
        Ok(client) => {
            client;
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}