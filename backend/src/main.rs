use dotenv::dotenv;
mod requests;
use requests::extract_financial_data;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let body = extract_financial_data("^GSPC");
    Ok(())
}