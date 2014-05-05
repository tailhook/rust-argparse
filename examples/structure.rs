//extern crate argparse;
extern crate collections;


use std::os;

use argparse::{ArgumentParser, StoreTrue, Store};

#[path="../argparse/mod.rs"]
mod argparse;

struct Options {
    verbose: bool,
    name: ~str,
}


fn main() {
    let mut options = Options {
        verbose: false,
        name: "World".to_owned(),
    };
    let mut ap = ArgumentParser::new();
    ap.set_description("Greet somebody.");
    ap.refer(&mut options.verbose)
        .add_option(["-v", "--verbose"], ~StoreTrue,
        "Be verbose");
    ap.refer(&mut options.name)
        .add_option(["--name"], ~Store::<~str>,
        "Name for the greeting");
    match ap.parse_args() {
        Ok(()) => {}
        Err(x) => {
            os::set_exit_status(x);
            return;
        }
    }

    if options.verbose {
        println!("name is {}", options.name);
    }
    println!("Hello {}!", options.name);
}
