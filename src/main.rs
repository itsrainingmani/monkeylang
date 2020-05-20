use monkey::repl::Repl;
use std::process;

fn main() {
    let repl = Repl::new(String::from(">>"));

    if let Err(e) = repl.run() {
        eprint!("Error in REPL: {}", e);
        process::exit(1);
    }
}
