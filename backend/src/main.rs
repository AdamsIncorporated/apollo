use dotenv::dotenv;
mod api;
use api::fetch_data;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let url = "https://finance.yahoo.com/quote/TSLA/";
    fetch_data(url).await?;

    Ok(())
}
