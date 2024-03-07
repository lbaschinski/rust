use std::env;
use std::process;

use minigrep::Config;
// Execute `cargo run --bin minigrep -- "the" "poem.txt"` for example
// Execute `cargo doc --open` to view the documentation!
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
