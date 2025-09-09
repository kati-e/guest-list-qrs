mod error;
mod tokens;

use dotenvy::dotenv;
use std::env;
use std::fs;

use crate::tokens::generate;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let url: String = get_env("URL")?;
    let tokens_csv_path: String = get_env("TOKENS_CSV_PATH")?;

    fs::create_dir_all("out")?;

    println!("Generating QR codes from tokens in {tokens_csv_path}...");
    generate(&url, &tokens_csv_path)?;

    println!("Successfully generated QR codes. Files are in the /out directory. Enjoy!");
    Ok(())
}

fn get_env(key: &str) -> Result<String, String> {
    env::var(key).map_err(|_| format!("Missing required env var: {}", key))
}
