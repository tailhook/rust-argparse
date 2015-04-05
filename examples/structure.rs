extern crate argparse;

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
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Be verbose");
        ap.refer(&mut options.name)
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

    if options.verbose {
        println!("name is {}", options.name);
    }
    println!("Hello {}!", options.name);
}
