fn main() {
    const SUBL: &str = "C:\\Program Files\\Sublime Text\\subl.exe";

    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let test_argument = args.join(" ");

    if test_argument.len() > 1 && std::path::Path::new(&test_argument).exists() {
        args = vec![test_argument];
    }

    let result = std::process::Command::new(SUBL).args(&args).status();

    match result {
        Ok(status) => {
            let exit_code = status.code().unwrap_or(1);
            std::process::exit(exit_code);
        }
        Err(e) => {
            eprintln!("Failed to launch '{}'. Error: {}", SUBL, e);
            std::process::exit(1);
        }
    }
}
