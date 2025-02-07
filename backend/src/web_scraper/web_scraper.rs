use scraper::{Html, Selector};

pub fn extract_financial_data(html_text: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let document = Html::parse_document(&html_text);
    let selector = Selector::parse("fin-streamer[data-symbol]").unwrap();

    if let Some(element) = document.select(&selector).next() {
        let inner_html = element.html();
        println!("{}", inner_html);
        Ok(Some(inner_html))
    } else {
        Ok(None)
    }
}