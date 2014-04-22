#![crate_id = "argparse"]
#![crate_type = "lib"]
#![feature(struct_variant)]

use std::os;

pub enum Action<'a> {
    StoreTrue(&'a mut bool),
    StoreFalse(&'a mut bool),
}

pub enum Argument<'a> {
    DashOption {
        options: &'a[&'a str],
        help: &'a str,
        metavar: &'a str,
        action: Action<'a>,
    },
    Positional {
        name: &'a str,
        help: &'a str,
        action: Action<'a>,
    },
}

pub struct ArgumentParser<'a> {
    priv arguments: Vec<Argument<'a>>,
}


impl<'a> ArgumentParser<'a> {
    fn new() -> ArgumentParser {
        return ArgumentParser { arguments: Vec::new() };
    }
    fn add(&mut self, arg: Argument<'a>) {
        self.arguments.push(arg);
    }
    fn parse_args(&self, args: ~[~str]) {
    }
}

#[test]
fn test_no_arg() {
    let ap = ArgumentParser::new();
    ap.parse_args(os::args());
}
