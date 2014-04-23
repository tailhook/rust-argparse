#![crate_id = "argparse"]
#![crate_type = "lib"]

use std::os;
use std::rc::Rc;
use std::cell::RefCell;
use std::slice::Items;

pub enum Action<'a> {
    StoreTrue(Rc<RefCell<&'a mut bool>>),
    StoreFalse(Rc<RefCell<&'a mut bool>>),
}

struct DashOption<'a> {
    options: ~[&'a str],
    help: &'a str,
    action: Action<'a>,
}

struct Argument<'a> {
    name: &'a str,
    help: &'a str,
    action: Action<'a>,
}

pub struct ArgumentParser<'a> {
    priv options: ~[DashOption<'a>],
    priv arguments: ~[Argument<'a>],
}

struct Context<'a, 'b> {
    //parser: &'a ArgumentParser<'b>,
    options: &'a [DashOption<'b>],
    arguments: &'a [Argument<'b>],
    iter: Items<'a, ~str>,
}

impl<'a, 'b> Context<'a, 'b> {
    fn parse_argument(&mut self, arg: &str) {
    }

    fn parse_option(&mut self, opt: &DashOption) {
        match opt.action {
            StoreTrue(ref cell) => {
                **cell.borrow_mut() = true;
            }
            StoreFalse(ref cell) => {
                **cell.borrow_mut() = false;
            }
        }
    }

    fn parse_long_option<'c>(&'c mut self, arg: &str) {
        for opt in self.options.iter() {
            for tname in opt.options.iter() {
                if arg.eq(tname) {
                    self.parse_option(opt);
                    return;
                }
            }
        }
    }

    fn new<'c, 'd>(parser: &'c ArgumentParser<'d>, args: &'c[~str])
        -> Context<'c, 'd>
    {
        return Context {
            options: parser.options,
            arguments: parser.arguments,
            iter: args.iter(),
        };
    }

    fn parse(&mut self) {
        loop {
            let next = self.iter.next();
            match next {
                Some(arg) => {
                    // positional args: ^$, ^-$, ^[^-].*
                    if arg.len() < 2 || arg[0] != ('-' as u8) {
                        self.parse_argument(*arg);
                    } else if arg[1] == ('-' as u8) {
                        self.parse_long_option(*arg);
                    }
                }
                None => {
                    break;
                }
            }
        }
    }
}


impl<'a> ArgumentParser<'a> {
    fn new() -> ArgumentParser {
        return ArgumentParser {
            arguments: ~[],
            options: ~[],
            };
    }
    fn add_option<'c>(&'c mut self, names: ~[&'a str],
        help: &'a str, action: Action<'a>) {
        self.options.push(DashOption {
            options: names,
            help: help,
            action: action,
            });
    }
    /*
    fn add_argument<'b>(&'b mut self, name: &'a str,
        help: &'a str, action: Action<'a>) {
        self.arguments.push(Argument {
            name: name,
            help: help,
            action: action,
        })
    }
    */

    fn parse_args(&self, args: ~[~str]) {
        Context::new(self, args).parse();
    }
}

fn cell<'a, T>(val: &'a mut T) -> Rc<RefCell<&'a mut T>> {
    return Rc::new(RefCell::new(val));
}

#[test]
fn test_no_arg() {
    let ap = ArgumentParser::new();
    ap.parse_args(os::args());
}


#[test]
fn test_store_true() {
    let mut verbose = false;
    let mut ap = ArgumentParser::new();
    ap.add_option(~["-t", "--true"],
        "Store true action",
        StoreTrue(cell(&mut verbose)));
    assert!(!verbose);
    ap.parse_args(~[~"./argparse_test"]);
    assert!(!verbose);
    ap.parse_args(~[~"./argparse_test", ~"--true"]);
    assert!(verbose);
}

#[test]
fn test_store_false() {
    let mut verbose = true;
    let mut ap = ArgumentParser::new();
    ap.add_option(~["-f", "--false"],
        "Store false action",
        StoreFalse(cell(&mut verbose)));
    assert!(verbose);
    ap.parse_args(~[~"./argparse_test"]);
    assert!(verbose);
    ap.parse_args(~[~"./argparse_test", ~"--false"]);
    assert!(!verbose);
}

/*
#[test]
fn test_verbose() {
    let mut opt = Options {
        verbose: false,
        debug: false,
        };
    {
        let mut ap = ArgumentParser::new();
        let vcell = cell(&mut opt.verbose);
        ap.add_option(~["-v", "--verbose"],
            "Turn on verbose mode",
            StoreTrue(vcell.clone()));
        ap.add_option(~["-q", "--quiet"],
            "Turn on quiet mode",
            StoreFalse(vcell.clone()));
        ap.add_option(~["-d", "--debug"],
            "Enable debugging",
            StoreTrue(cell(&mut opt.debug)));
        ap.parse_args(~[~"./argparse_test", ~"-v"]);
    }
    assert!(opt.verbose);
    assert!(opt.debug);
}

*/
