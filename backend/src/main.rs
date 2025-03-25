mod requests;
mod utils;
use requests::markets::markets::fetch_market_data;
use utils::save_json_to_file;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = fetch_market_data().await?;
    let _ = save_json_to_file(&json, "output.json").await;
    Ok(())
}
