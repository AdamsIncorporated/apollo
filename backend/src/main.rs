use dotenv::dotenv;
mod requests;
use requests::extract_financial_data;
use serde_json::to_string_pretty;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    match extract_financial_data().await {
        Ok(json_data) => {
            let file_path = "financial_data.json";
            let mut file = File::create(file_path).await?;
            let json_string = to_string_pretty(&json_data)?;
            file.write_all(json_string.as_bytes()).await?;
            println!("JSON data saved to {}", file_path);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}