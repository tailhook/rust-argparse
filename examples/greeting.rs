/* The extern crate crashes current rust compiler (0.10) for some reason but
 * directly including a module in program works */
//extern crate argparse;
extern crate collections;


use std::os;

use argparse::{ArgumentParser, StoreTrue, Store};

#[path="../argparse/mod.rs"]
mod argparse;

fn main() {
    let mut verbose = false;
    let mut name = "World".to_owned();

    let mut ap = ArgumentParser::new();
    ap.set_description("Greet somebody.");
    ap.refer(&mut verbose)
        .add_option(["-v", "--verbose"], box StoreTrue,
        "Be verbose");
    ap.refer(&mut name)
        .add_option(["--name"], box Store::<~str>,
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
