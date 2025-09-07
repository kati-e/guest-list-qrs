pub mod error {
    use chrono::Local;
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::sync::OnceLock;

    // timestamp before the logs only once per run, keeps logs cleaner since they are just simple and dumped into a log file
    static TIMESTAMP_WRITTEN: OnceLock<()> = OnceLock::new();

    pub fn log_error(err_msg: &str) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("error.log")?;

        TIMESTAMP_WRITTEN.get_or_init(|| {
            let timestamp = Local::now().format("%d/%m/%Y %H:%M:%S").to_string();
            writeln!(file, "----- {timestamp} -----").unwrap();
        });

        writeln!(file, "{err_msg}")?;
        Ok(())
    }
}
