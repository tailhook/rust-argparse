extern crate argparse;

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
                // We should set exit status, but rust 1.0.0-beta has
                // unstable set_exit_status, so we panic, temporarily
                panic!("Command line error. Error code {}", x);
                // set_exit_status(x);
                // return;
            }
        }
    }

    if verbose {
        println!("name is {}", name);
    }
    println!("Hello {}!", name);
}
