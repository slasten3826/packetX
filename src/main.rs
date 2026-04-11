mod cli;
mod cli_app;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = match cli::parse_args(&args) {
        Ok(command) => command,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(2);
        }
    };

    if let Err(err) = cli_app::run(command) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
