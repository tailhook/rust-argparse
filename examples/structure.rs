#![feature(box_syntax)]

extern crate argparse;


use std::os;

use argparse::{ArgumentParser, StoreTrue, Store};


struct Options {
    verbose: bool,
    name: String,
}


fn main() {
    let mut options = Options {
        verbose: false,
        name: "World".to_string(),
    };
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Greet somebody.");
        ap.refer(&mut options.verbose)
            .add_option(&["-v", "--verbose"], box StoreTrue,
            "Be verbose");
        ap.refer(&mut options.name)
            .add_option(&["--name"], box Store,
            "Name for the greeting");
        match ap.parse_args() {
            Ok(()) => {}
            Err(x) => {
                os::set_exit_status(x);
                return;
            }
        }
    }

    if options.verbose {
        println!("name is {}", options.name);
    }
    println!("Hello {}!", options.name);
}
