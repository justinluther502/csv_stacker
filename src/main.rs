use csv_stacker::{run, Config};
use std::process;

fn main() {
    let config = Config::build().unwrap_or_else(|err| {
        eprintln!("Couldn't configure the csv stacker: {err}");
        process::exit(1);
    });

    run(config).unwrap_or_else(|err| {
        eprintln!("Application error: {err}");
        process::exit(1);
    });
}
