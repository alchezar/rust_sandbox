use minigrep::{Config, run};

fn main() {
    println!("Hello, world!");
    let config = Config::build(std::env::args()).unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments: {}", e);
        std::process::exit(1);
    });

    println!("-- Searching for {}", config.query);
    println!("-- In file {}", config.path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
