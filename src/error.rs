use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::OnceLock;

static FRESH_RUN: OnceLock<()> = OnceLock::new();

pub fn log_error(err_msg: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("error.log")?;

    FRESH_RUN.get_or_init(|| {
        let timestamp = Local::now().format("%d/%m/%Y %H:%M:%S").to_string();
        file.set_len(0).unwrap(); // clear the file contents
        writeln!(file, "----- {timestamp} -----").unwrap();
    });

    writeln!(file, "{err_msg}")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn logs_error() {
        let timestamp: String = Local::now().format("%d/%m/%Y %H:%M:%S").to_string();
        let result = log_error("Oh no, a wild error occured!");
        let log_contents = fs::read_to_string("error.log").unwrap();

        assert!(log_contents.contains(&format!("----- {timestamp} -----")));
        assert!(log_contents.contains(&format!("Oh no, a wild error occured!")));
        assert!(result.is_ok());
    }

    #[test]
    fn logs_multiple_errors_per_run() {
        let timestamp: String = Local::now().format("%d/%m/%Y %H:%M:%S").to_string();
        let result = log_error("Oh no, a wild error occured!");
        let result2 = log_error("Oh no, another a wild error occured!");
        let result3 = log_error("Oh no, more wild errors occured!");
        let log_contents = fs::read_to_string("error.log").unwrap();

        assert!(
            log_contents
                .matches(&format!("----- {timestamp} -----"))
                .count()
                == 1
        );
        assert!(log_contents.contains(&format!("Oh no, a wild error occured!")));
        assert!(log_contents.contains(&format!("Oh no, another a wild error occured!")));
        assert!(log_contents.contains(&format!("Oh no, more wild errors occured!")));
        assert!(result.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_ok());
    }
}
