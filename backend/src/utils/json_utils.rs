use std::io::Write;

pub async fn save_json_to_file(json: &serde_json::Value, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = std::fs::File::create(filename)?;
    let json_string = serde_json::to_string_pretty(json)?;
    file.write(json_string.as_bytes())?;
    print!("Output saved to json.");
    Ok(())
}
