use dotenv::dotenv;
mod api;
use api::fetch_data;
mod web_scraper;
use web_scraper::extract_financial_data;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let url = "https://finance.yahoo.com/quote/TSLA/";
    let html_text = fetch_data(url).await?;
    let test: Option<String> = extract_financial_data(&html_text)?;


    match test {
        Some(value) => {
            let _ = extract_financial_data(&value);
        },
        None => println!("No financial html data was found!"),
    }

    Ok(())
}