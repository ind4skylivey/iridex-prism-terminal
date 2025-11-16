use clap::Parser;
use prism::cli::PrismCli;
use prism::error::PrismResult;

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run() -> PrismResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let cli = PrismCli::parse();
    prism::cli::run(cli)
}
