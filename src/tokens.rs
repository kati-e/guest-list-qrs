pub mod tokens {
    use crate::error::error::log_error;
    use qrcode::QrCode;
    use qrcode::render::svg;
    use std::fs;

    pub fn generate(url: &str, tokens_csv_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let tokens: Vec<String> = extract_tokens(&tokens_csv_path)?;

        if tokens.is_empty() {
            eprintln!("No tokens found in CSV. Add comma-separated values and try again.");
            std::process::exit(1); // exit early if no tokens were found
        }

        for token in tokens {
            if let Err(err) = validate_token(&token) {
                log_error(&format!("Invalid token '{}': {}", token, err))?;
                eprint!(
                    // print to terminal so user knows something was skipped
                    "Skipped invalid token: {}\n",
                    if token.is_empty() { " (empty)" } else { &token }
                );
                continue;
            }
            generate_qr_svg(&url, &token)?;
        }

        Ok(())
    }

    fn extract_tokens(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let content: String = fs::read_to_string(path)?;
        Ok(content
            .trim()
            .split(',')
            .map(|s: &str| s.trim().to_string())
            .filter(|s: &String| !s.is_empty()) // ignore accidental empty tokens
            .collect())
    }

    fn generate_qr_svg(url: &str, token: &str) -> Result<(), Box<dyn std::error::Error>> {
        let data: String = format!("{url}{token}");
        let code: QrCode = QrCode::new(data.as_bytes())?;
        let svg: String = code
            .render::<svg::Color>()
            .min_dimensions(512, 512)
            .quiet_zone(true)
            .build();
        let filename: String = format!("out/{}.svg", token.to_lowercase());
        fs::write(filename, svg)?;
        Ok(())
    }

    fn validate_token(token: &str) -> Result<(), &str> {
        if token.is_empty() {
            Err("token is empty")
        } else if !token.chars().all(|c: char| c.is_alphanumeric()) {
            Err("token must be alphanumeric")
        } else {
            Ok(())
        }
    }
}
