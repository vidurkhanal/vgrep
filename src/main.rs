use std::{env, process};
use vgrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match Config::build(&args) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Problem parsing provided parameters : {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1)
    }
}
