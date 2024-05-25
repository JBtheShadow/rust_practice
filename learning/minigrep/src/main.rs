//cargo run --bin minigrep -- searchstring example-filename.txt
// -- here means arguments after it are for our program and not cargo

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // We need to annotate the type here for collect() to work
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
