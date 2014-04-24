#![crate_id = "argparse"]
#![crate_type = "lib"]

extern crate collections;

use std::os;
use std::rc::Rc;
use std::cell::RefCell;
use std::slice::Items;
use collections::hashmap::HashMap;

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
    priv options: ~[Rc<DashOption<'a>>],
    priv arguments: ~[Rc<Argument<'a>>],
    priv short_options: HashMap<char, Rc<DashOption<'a>>>,
    priv long_options: HashMap<&'a str, Rc<DashOption<'a>>>,
}

struct Context<'a, 'b> {
    parser: &'a ArgumentParser<'b>,
    iter: Items<'a, ~str>,
}

impl<'a> Action<'a> {
    fn has_arg(&self) -> bool {
        return match *self {
            StoreTrue(_) => false,
            StoreFalse(_) => false,
        }
    }
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
        for (name, opt) in self.parser.long_options.iter() {
            if arg.eq(name) {
                self.parse_option(&**opt);
                return;
            }
        }
    }

    fn parse_short_options<'c>(&'c mut self, arg: &str) {
        let mut iter = arg.chars();
        iter.next();
        for ch in iter {
            match self.parser.short_options.find(&ch) {
                Some(opt) => {
                    self.parse_option(&**opt);
                }
                None => {
                    fail!("Unknown short option \"{}\"", ch);
                }
            }
        }
    }

    fn new<'c, 'd>(parser: &'c ArgumentParser<'d>, args: &'c[~str])
        -> Context<'c, 'd>
    {
        return Context {
            parser: parser,
            iter: args.iter(),
        };
    }

    fn parse(&mut self) {
        loop {
            let next = self.iter.next();
            match next {
                Some(arg) => {
                    if is_argument(*arg) {
                        self.parse_argument(*arg);
                    } else if arg[1] == ('-' as u8) {
                        self.parse_long_option(*arg);
                    } else {
                        self.parse_short_options(*arg);
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
            short_options: HashMap::new(),
            long_options: HashMap::new(),
            };
    }
    fn add_option<'c>(&'c mut self, names: ~[&'a str],
        help: &'a str, action: Action<'a>) {
        let opt = Rc::new(DashOption {
            options: names,
            help: help,
            action: action,
            });
        self.options.push(opt.clone());
        for _n in opt.options.iter() {
            let name = *_n;
            if is_argument(name) {
                fail!("Bad argument name {}", name);
            } else if name[1] == ('-' as u8) {
                self.long_options.insert(name, opt.clone());
            } else if name.len() > 2 {
                fail!("Bad short option {}", name);
            } else {
                self.short_options.insert(name[1] as char, opt.clone());
            }
        }
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

fn is_argument(name: &str) -> bool {
    return name.len() < 2 || name[0] != ('-' as u8);
}

pub fn cell<'a, T>(val: &'a mut T) -> Rc<RefCell<&'a mut T>> {
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

#[test]
fn test_bool() {
    let mut verbose = false;
    let mut ap = ArgumentParser::new();
    let c = cell(&mut verbose);
    ap.add_option(~["-f", "--false"],
        "Store false action",
        StoreFalse(c.clone()));
    ap.add_option(~["-t", "--true"],
        "Store false action",
        StoreTrue(c.clone()));
    assert!(!verbose);
    ap.parse_args(~[~"./argparse_test"]);
    assert!(!verbose);
    ap.parse_args(~[~"./argparse_test", ~"-t"]);
    assert!(verbose);
    ap.parse_args(~[~"./argparse_test", ~"-f"]);
    assert!(!verbose);
    ap.parse_args(~[~"./argparse_test", ~"-fft"]);
    assert!(verbose);
    ap.parse_args(~[~"./argparse_test", ~"-fffft", ~"-f"]);
    assert!(!verbose);
    ap.parse_args(~[~"./argparse_test", ~"--false", ~"-fffft", ~"-f",
                    ~"--true"]);
    assert!(verbose);
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
