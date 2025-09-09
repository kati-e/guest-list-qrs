use crate::error::log_error;
use qrcode::QrCode;
use qrcode::render::svg;
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(test)]
const IS_TEST: bool = true;

#[cfg(not(test))]
const IS_TEST: bool = false;

pub fn generate(url: &str, tokens_csv_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tokens: Vec<String> = extract_tokens(tokens_csv_path)?;

    if tokens.is_empty() {
        // Prefer returning an error over exiting in library code
        return Err("No tokens found in CSV. Add comma-separated values and try again.".into());
    }

    for token in tokens {
        if let Err(err) = validate_token(&token) {
            log_error(&format!("Invalid token '{}': {}", token, err))?;
            eprint!(
                "Skipped invalid token: {}\n",
                if token.is_empty() { " (empty)" } else { &token }
            );
            continue;
        }
        generate_qr_svg(url, &token)?;
    }

    Ok(())
}

fn extract_tokens(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    Ok(content
        .trim()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty()) // ignore accidental empty tokens
        .collect())
}

fn generate_qr_svg(url: &str, token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data = format!("{url}{token}");
    let code = QrCode::new(data.as_bytes())?;
    let svg = code
        .render::<svg::Color>()
        .min_dimensions(600, 600)
        .quiet_zone(true)
        .build();

    let out_dir = if IS_TEST { "tests/out" } else { "out" };
    let filename: PathBuf = Path::new(out_dir).join(format!("{}.svg", token.to_lowercase()));

    // Ensure the output directory exists
    if let Some(parent) = filename.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&filename, svg)?;
    Ok(())
}

fn validate_token(token: &str) -> Result<(), &str> {
    if token.is_empty() {
        Err("token is empty")
    } else if !token.chars().all(|c| c.is_alphanumeric()) {
        Err("token must be alphanumeric")
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn generate_succeeds_if_tokens_are_valid() {
        let token = "Testing999";
        let mock_csv = token;
        let path_for_csv_str = "tests/.testtokens.test.csv";
        let path_for_csv = Path::new(path_for_csv_str);

        if path_for_csv.exists() {
            fs::remove_file(path_for_csv).expect("Failed to delete test-generated csv file");
        }
        fs::write(path_for_csv, mock_csv).expect("Failed to write mock csv file");
        assert!(path_for_csv.exists());

        let result = generate("http://someurlhere.com/?param=", path_for_csv_str);
        assert!(result.is_ok());

        let path_for_svg = format!("tests/out/{}.svg", token.to_lowercase());
        let path_for_svg: &Path = Path::new(&path_for_svg);

        fs::remove_file(path_for_svg).expect("Failed to delete test-generated svg file");
        fs::remove_file(path_for_csv).expect("Failed to delete test-generated csv file");
    }

    mod extract_tokens_tests {
        use super::*;
        use std::path::Path;

        #[test]
        fn successfully_extracts_token_values() {
            let mock_csv = "testtoken1,testtoken2,testtoken3";
            let path_str = "tests/.moretesttokens.test.csv";
            let path = Path::new(path_str);

            if path.exists() {
                fs::remove_file(path).expect("Failed to delete test-generated csv file");
            }
            fs::write(path, mock_csv).expect("Failed to write mock csv file");
            assert!(path.exists());

            let result = extract_tokens(path_str);
            assert!(result.is_ok());

            let tokens = result.unwrap();
            assert_eq!(tokens.len(), 3);
            assert_eq!(tokens, vec!["testtoken1", "testtoken2", "testtoken3"]);

            fs::remove_file(path).expect("Failed to delete test-generated csv file");
        }
    }

    mod generate_qr_svg_tests {
        use super::*;
        use std::fs;
        use std::path::Path;

        #[test]
        fn successfully_generates_qr_code_svg_file() {
            let token = "someTOKEN123";
            let result = generate_qr_svg("http://someurlhere.com/?param=", token);
            assert!(result.is_ok());

            let expected_path = format!("tests/out/{}.svg", token.to_lowercase());
            let path = Path::new(&expected_path);
            assert!(path.exists());

            fs::remove_file(path).expect("Failed to delete test-generated svg file");
        }
    }

    mod validate_token_tests {
        use super::*;

        #[test]
        fn succeeds_if_token_is_valid() {
            let result = validate_token("Myfeelingsarevalid123");
            assert_eq!(result, Ok(()));
        }

        #[test]
        fn throws_error_for_empty() {
            let result = validate_token("");
            assert_eq!(result, Err("token is empty"));
        }

        #[test]
        fn throws_error_for_non_alphanumeric() {
            let result = validate_token("XYZ%$#");
            assert_eq!(result, Err("token must be alphanumeric"));
        }
    }
}
