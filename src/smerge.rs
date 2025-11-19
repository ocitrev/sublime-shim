fn main() {
    const SMERGE: &str = "C:\\Program Files\\Sublime Merge\\smerge.exe";

    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = std::process::Command::new(SMERGE).args(&args).status();

    match result {
        Ok(status) => {
            let exit_code = status.code().unwrap_or(1);
            std::process::exit(exit_code);
        }
        Err(e) => {
            eprintln!("Failed to launch '{}'. Error: {}", SMERGE, e);
            std::process::exit(1);
        }
    }
}
