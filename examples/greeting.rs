extern crate argparse;

use std::os;

use argparse::{ArgumentParser, StoreTrue, Store};

fn main() {
    let mut verbose = false;
    let mut name = "World".to_owned();

    let mut ap = ArgumentParser::new();
    ap.set_description("Greet somebody.");
    ap.refer(&mut verbose)
        .add_option(["-v", "--verbose"], ~StoreTrue,
        "Be verbose");
    ap.refer(&mut name)
        .add_option(["--name"], ~Store::<~str>,
        "Name for the greeting");
    match ap.parse_args() {
        Ok(()) => {}
        Err(x) => {
            os::set_exit_status(x);
            return;
        }
    }

    if verbose {
        println!("name is {}", name);
    }
    println!("Hello {}!", name);
}
