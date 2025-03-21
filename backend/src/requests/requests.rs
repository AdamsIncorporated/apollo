use reqwest::header::{HeaderMap, HeaderValue};
use std::error::Error;
use serde_json::Value;
use urlencoding::encode;

pub async fn extract_financial_data() -> Result<Value, Box<dyn Error>> {
    let metrics = "annualTaxEffectOfUnusualItems,trailingTaxEffectOfUnusualItems,..."; // Your long metric string
    let encoded_metrics = encode(metrics);
    let url = format!("https://query1.finance.yahoo.com/ws/fundamentals-timeseries/v1/finance/timeseries/TSLA?merge=false&padTimeSeries=true&period1=493590046&period2=1742522399&type={}&lang=en-US&region=US", encoded_metrics);
    let mut headers = HeaderMap::new();
    headers.insert(
        "User-Agent",
        HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:136.0) Gecko/20100101 Firefox/136.0",
        ),
    );
    headers.insert("Accept", HeaderValue::from_static("*/*"));
    headers.insert(
        "Accept-Language",
        HeaderValue::from_static("en-US,en;q=0.5"),
    );
    headers.insert(
        "Accept-Encoding",
        HeaderValue::from_static("gzip, deflate, br, zstd"),
    );
    headers.insert(
        "Referer",
        HeaderValue::from_static("https://finance.yahoo.com/quote/TSLA/financials/"),
    );
    headers.insert(
        "Origin",
        HeaderValue::from_static("https://finance.yahoo.com"),
    );
    headers.insert("Cookie", HeaderValue::from_static("A1=d=AQABBMrC3GcCEG1advixeFJ6gO0WSCgSJrsFEgEBAQEU3mfmZ9xS0iMA_eMAAA&S=AQAAAryz87R9M7VfcKIO2byGWkc; A3=d=AQABBMrC3GcCEG1advixeFJ6gO0WSCgSJrsFEgEBAQEU3mfmZ9xS0iMA_eMAAA&S=AQAAAryz87R9M7VfcKIO2byGWkc; A1S=d=AQABBMrC3GcCEG1advixeFJ6gO0WSCgSJrsFEgEBAQEU3mfmZ9xS0iMA_eMAAA&S=AQAAAryz87R9M7VfcKIO2byGWkc; cmp=t=1742521038&j=0&u=1YNN; gpp=DBABLA~BVRqAAAAAmA.QA; gpp_sid=7; PRF=t%3DTSLA; axids=gam=y-IyVlGZJE2uIO7VKbDBQLLu6bpwIwa3Di~A&dv360=eS15S3VlV1BaRTJ1RlVqMHJLbXFsMUowTjBYdVVGdkdqWn5B&ydsp=y-T427AYpE2uLyzokLl5x_gi6Uoa_1ThJl~A&tbla=y-IjX12NxE2uIWM_W1LnjXqrRONoJDtmiQ~A; tbla_id=aa5355ae-dfd3-4e22-8178-d1dde6a773fd-tucted6494d; _cb=CJTG38C_DbGTDFhT0D; _chartbeat2=.1742521039188.1742522144078.1.Ns8mDBXXDMECEjY1vCjnpWzCoVH4r.2; _cb_svref=https%3A%2F%2Fhelp.yahoo.com%2Fkb%2FSLN2311.html; _chartbeat5=; _SUPERFLY_lockout=1"));
    headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("empty"));
    headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("cors"));
    headers.insert("Sec-Fetch-Site", HeaderValue::from_static("same-site"));

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let json: Value = response.json().await?;
        Ok(json)
    } else {
        Err(format!("HTTP request failed with status: {}", response.status()).into())
    }
}
