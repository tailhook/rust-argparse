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
        ap.parse_args_or_exit();
    }

    if verbose {
        println!("name is {}", name);
    }
    println!("Hello {}!", name);
}
