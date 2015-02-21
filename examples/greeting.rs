#![feature(env)]

extern crate argparse;

use std::env::set_exit_status;

use argparse::{ArgumentParser, StoreTrue, Store};

fn main() {
    let mut verbose = false;
    let mut name = "World".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Greet somebody.");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Be verbose");
        ap.refer(&mut name)
            .add_option(&["--name"], Store,
            "Name for the greeting");
        match ap.parse_args() {
            Ok(()) => {}
            Err(x) => {
                set_exit_status(x);
                return;
            }
        }
    }

    if verbose {
        println!("name is {}", name);
    }
    println!("Hello {}!", name);
}
