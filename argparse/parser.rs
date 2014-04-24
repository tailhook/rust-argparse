use std::rc::Rc;
use std::cell::RefCell;
use std::slice::Items;
use collections::hashmap::HashMap;


pub enum Action<'a> {
    StoreTrue(Rc<RefCell<&'a mut bool>>),
    StoreFalse(Rc<RefCell<&'a mut bool>>),
    IncrInt(Rc<RefCell<&'a mut int>>),
    DecrInt(Rc<RefCell<&'a mut int>>),
    /*
    SetInt(Rc<RefCell<&'a mut int>>),
    */
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
            //SetInt(_) => true,
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
            IncrInt(ref cell) => {
                **cell.borrow_mut() += 1;
            }
            DecrInt(ref cell) => {
                **cell.borrow_mut() -= 1;
            }
            /*
            SetInt(ref cell) => {
                **cell.borrow_mut() = ;
            }
            */
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

    pub fn parse_args(&self, args: ~[~str]) {
        Context::new(self, args).parse();
    }
}

fn is_argument(name: &str) -> bool {
    return name.len() < 2 || name[0] != ('-' as u8);
}

pub fn cell<'a, T>(val: &'a mut T) -> Rc<RefCell<&'a mut T>> {
    return Rc::new(RefCell::new(val));
}
