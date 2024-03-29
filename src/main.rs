use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config =  Config::build(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("path: - {}", config.file_path);
    println!("query: - {}", config.query);
    
    if let Err(e) = minigrep::run(config) {
        println!("application error: - {e}");
        process::exit(1);
    }

    
}

