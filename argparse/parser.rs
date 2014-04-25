use std::rc::Rc;
use std::cell::RefCell;
use std::slice::Items;
use std::os;
use std::int;
use collections::hashmap::HashMap;



pub enum Action<'a> {
    StoreTrue(Rc<RefCell<&'a mut bool>>),
    StoreFalse(Rc<RefCell<&'a mut bool>>),
    IncrInt(Rc<RefCell<&'a mut int>>),
    DecrInt(Rc<RefCell<&'a mut int>>),
    SetInt(Rc<RefCell<&'a mut int>>),
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
            IncrInt(_) => false,
            DecrInt(_) => false,
            SetInt(_) => true,
        }
    }
}

impl<'a, 'b> Context<'a, 'b> {
    fn parse_argument(&mut self, arg: &str) {
    }

    fn parse_flag(&mut self, opt: &DashOption) {
        match opt.action {
            StoreTrue(ref cell) => {
                **cell.borrow_mut() = true;
            }
            StoreFalse(ref cell) => {
                **cell.borrow_mut() = false;
            }
            IncrInt(ref cell) => {
                **cell.borrow_mut() += 1;
            }
            DecrInt(ref cell) => {
                **cell.borrow_mut() -= 1;
            }
            _ => { fail!("Unexpected flag action"); }
        }
    }

    fn parse_option(&mut self, opt: &DashOption, arg: &str) {
        match opt.action {
            SetInt(ref cell) => {
                let val = match int::parse_bytes(arg.as_bytes(), 10) {
                    Some(val) => { val }
                    None => {
                        self.error(format!("Invalid integer \"{}\"", arg));
                    }
                };
                **cell.borrow_mut() = val;
            }
            _ => { fail!("Unexpected flag action"); }
        };
    }

    fn error(&self, message: ~str) -> ! {
        os::set_exit_status(2);
        fail!(message);
    }

    fn get_next_option(&mut self, opt: &DashOption, name: &str) {
        let value = self.iter.next();
        match value {
            Some(arg) => {
                self.parse_option(opt, *arg);
            }
            None => {
                self.error(format!("Option {} requires value", name));
            }
        }
    }

    fn parse_long_option<'c>(&'c mut self, arg: &str) {
        let mut equals_iter = arg.splitn('=', 1);
        let realarg = match equals_iter.next() {
            Some(value) => { value }
            None => { fail!() }
        };
        for (name, opt) in self.parser.long_options.iter() {
            if realarg.eq(name) {
                if !opt.action.has_arg() {
                    match equals_iter.next() {
                        Some(arg) => {
                            self.error(format!(
                                "Option {} does not accept an argument",
                                name));
                        }
                        None => {}
                    }
                    self.parse_flag(&**opt);
                } else {
                    match equals_iter.next() {
                        Some(arg) => {
                            self.parse_option(&**opt, arg);
                        }
                        None => {
                            self.get_next_option(&**opt, arg);
                        }
                    }
                }
                return;
            }
        }
    }

    fn parse_short_options<'c>(&'c mut self, arg: &str) {
        let mut iter = arg.char_indices();
        iter.next();
        for (idx, ch) in iter {
            let opt = match self.parser.short_options.find(&ch) {
                Some(opt) => { opt }
                None => { fail!("Unknown short option \"{}\"", ch); }
            };
            if opt.action.has_arg() {
                if idx + 1 < arg.len() {
                    self.parse_option(&**opt, arg.slice(idx+1, arg.len()));
                } else {
                    self.get_next_option(&**opt, arg);
                }
                break;
            } else {
                self.parse_flag(&**opt);
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
            let arg = match next {
                Some(arg) => { arg }
                None => { break; }
            };
            if is_argument(*arg) {
                self.parse_argument(*arg);
            } else if arg[1] == ('-' as u8) {
                self.parse_long_option(*arg);
            } else {
                self.parse_short_options(*arg);
            }
        }
    }
}


impl<'a> ArgumentParser<'a> {
    pub fn new() -> ArgumentParser {
        return ArgumentParser {
            arguments: ~[],
            options: ~[],
            short_options: HashMap::new(),
            long_options: HashMap::new(),
            };
    }
    pub fn add_option<'c>(&'c mut self, names: ~[&'a str],
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
    pub fn parse_list(&self, args: ~[~str]) {
        Context::new(self, args).parse();
    }

    pub fn parse_args(&self) {
        self.parse_list(os::args());
    }
}

fn is_argument(name: &str) -> bool {
    return name.len() < 2 || name[0] != ('-' as u8);
}

pub fn cell<'a, T>(val: &'a mut T) -> Rc<RefCell<&'a mut T>> {
    return Rc::new(RefCell::new(val));
}
