extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store, Print};

fn main() {
    let mut verbose = false;
    let mut name = "World".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Greet somebody.");
        ap.add_option(&["-V", "--version"],
            Print(env!("CARGO_PKG_VERSION").to_string()), "Show version");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Be verbose");
        ap.refer(&mut name)
            .add_option(&["--name"], Store,
            "Name for the greeting");
        ap.parse_args_or_exit();
    }

    if verbose {
        println!("name is {}", name);
    }
    println!("Hello {}!", name);
}
