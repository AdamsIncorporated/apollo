use dotenv::dotenv;
mod requests;
use requests::extract_financial_data;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let url = "https://finance.yahoo.com/quote/TSLA/";
    let html_text = fetch_data(url).await?;
    let test: Option<String> = extract_financial_data()?;


    match test {
        Some(value) => {
            let _ = extract_financial_data("^GSPC");
        },
        None => println!("No financial html data was found!"),
    }

    Ok(())
}