use std::os;
use std::io::IoResult;
use std::rc::Rc;
use std::cell::RefCell;
use std::iter::Peekable;
use std::slice::Items;
use std::fmt::{Show, Formatter};
use std::hash::Hash;
use std::hash::sip::SipState;

use collections::hashmap::HashMap;

use action::Action;
use action::{ParseResult, Parsed, Exit, Error};
use action::TypedAction;
use action::{Flag, Single, Push, Many};
use action::IArgAction;

mod action;

enum ArgumentKind {
    Positional,
    ShortOption,
    LongOption,
    Delimiter, // Barely "--"
}

impl ArgumentKind {
    fn check(name: &str) -> ArgumentKind {
        let mut iter = name.chars();
        let char1 = iter.next();
        let char2 = iter.next();
        let char3 = iter.next();
        return match char1 {
            Some('-') => match char2 {
                Some('-') => match char3 {
                        Some(_) => LongOption, // --opt
                        None => Delimiter,  // just --
                },
                Some(_) => ShortOption,  // -opts
                None => Positional,  // single dash
            },
            Some(_) | None => Positional,
        }
    }
}

enum OptionName<'a> {
    Dash(~[&'a str]),
    Pos(&'a str),
}

impl<'a> Show for OptionName<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> IoResult<()> {
        return match *self {
            Dash(ref names) => names.fmt(fmt),
            Pos(ref name) => name.fmt(fmt),
        }
    }
}

struct GenericOption<'a> {
    id: uint,
    name: OptionName<'a>,
    help: &'a str,
    action: Action,
}

impl<'a> Hash for GenericOption<'a> {
    fn hash(&self, state: &mut SipState) {
        state.write_uint(self.id).unwrap();
    }
}
impl<'a> Eq for GenericOption<'a> {
    fn eq(&self, other: &GenericOption<'a>) -> bool {
        return self.id == other.id;
    }
}
impl<'a> TotalEq for GenericOption<'a> {}

pub struct Context<'a, 'b> {
    parser: &'a ArgumentParser<'b>,
    options: HashMap<Rc<GenericOption<'b>>, Vec<&'a str>>,
    arguments: Vec<&'a str>,
    iter: Peekable<&'a ~str, Items<'a, ~str>>,
}

impl<'a, 'b> Context<'a, 'b> {

    fn parse_option(&mut self, opt: Rc<GenericOption<'b>>,
        optarg: Option<&'a str>)
        -> ParseResult
    {
        let value = match optarg {
            Some(value) => value,
            None => match self.iter.next() {
                Some(value) => {
                    let argborrow: &'a str = *value;
                    argborrow
                }
                None => {
                    return match opt.action {
                        Many(_) => Parsed,
                        _ => Error(format!(
                            "Option {} requires an argument", opt.name)),
                    };
                }
            },
        };
        let vec = self.options.find_or_insert(opt.clone(), Vec::new());
        vec.push(value);
        match opt.action {
            Single(ref action) => {
                return action.parse_arg(value);
            }
            Push(_) => {
                return Parsed;
            }
            Many(_) => {
                match optarg {
                    Some(_) => return Parsed,
                    _ => {}
                }
                loop {
                    match self.iter.peek() {
                        None => { break; }
                        Some(arg) if arg.starts_with("-") => { break; }
                        Some(value) => {
                            let argborrow: &'a str = **value;
                            vec.push(argborrow);
                        }
                    }
                    self.iter.next();
                }
                return Parsed;
            }
            _ => fail!(),
        };
    }

    fn parse_long_option(&mut self, arg: &'a str) -> ParseResult {
        let mut equals_iter = arg.splitn('=', 1);
        let optname = equals_iter.next().unwrap();
        let valueref = equals_iter.next();
        let opt = self.parser.long_options.find(&optname.to_str());
        match opt {
            Some(opt) => {
                match opt.action {
                    Flag(ref action) => {
                        match valueref {
                            Some(_) => {
                                return Error(format!(
                                    "Option {} does not accept an argument",
                                    optname));
                            }
                            None => {
                                self.options.find_or_insert(opt.clone(),
                                    Vec::new()).push(arg);
                                return action.parse_flag();
                            }
                        }
                    }
                    Single(_) | Push(_) | Many(_) => {
                        return self.parse_option(opt.clone(), valueref);
                    }
                }
            }
            None => {
                return Error(format!("Unknown option {}", arg));
            }
        }
    }

    fn parse_short_options<'x>(&'x mut self, arg: &'a str) -> ParseResult {
        let mut iter = arg.char_indices();
        iter.next();
        for (idx, ch) in iter {
            let opt = match self.parser.short_options.find(&ch) {
                Some(opt) => { opt }
                None => {
                    return Error(format!("Unknown short option \"{}\"", ch));
                }
            };
            let res = match opt.action {
                Flag(ref action) => action.parse_flag(),
                Single(_) | Push(_) | Many(_) => {
                    let value;
                    if idx + 1 < arg.len() {
                        value = Some(arg.slice(idx+1, arg.len()));
                    } else {
                        value = None;
                    }
                    return self.parse_option(opt.clone(), value);
                }
            };
            match res {
                Parsed => { continue; }
                x => { return x; }
            }
        }
        return Parsed;
    }

    fn postpone_argument(&mut self, arg: &'a str) {
        self.arguments.push(arg);
    }

    fn parse(parser: &ArgumentParser, args: &[~str])
        -> ParseResult
    {
        let mut ctx = Context {
            parser: parser,
            iter: args.iter().peekable(),
            options: HashMap::new(),
            arguments: Vec::new(),
        };
        ctx.iter.next();  // Command name
        loop {
            let next = ctx.iter.next();
            let arg = match next {
                Some(arg) => { arg }
                None => { break; }
            };
            let res = match ArgumentKind::check(*arg) {
                Positional => {
                    ctx.postpone_argument(*arg);
                    continue;
                }
                LongOption => ctx.parse_long_option(*arg),
                ShortOption => ctx.parse_short_options(*arg),
                Delimiter => break,
            };
            match res {
                Parsed => continue,
                _ => return res,
            }
        }

        loop {
            match ctx.iter.next() {
                None => break,
                Some(arg) => ctx.postpone_argument(*arg),
            }
        }

        let mut pargs = ctx.parser.arguments.iter();
        for arg in ctx.arguments.iter() {
            let opt = match pargs.next() {
                Some(opt) => opt,
                None => match ctx.parser.catchall_argument {
                    Some(ref opt) => opt,
                    None => return Error(format!(
                        "Unexpected argument {}", arg)),
                }
            };
            let res = match opt.action {
                Single(ref act) => match ctx.options.find(opt) {
                    Some(_) => continue,  // Option is already with --opt
                    None => {
                        ctx.options.insert(opt.clone(), vec!(*arg));
                        act.parse_arg(*arg)
                    }
                },
                Many(_) | Push(_) => {
                    ctx.options.find_or_insert(
                        opt.clone(), Vec::new()).push(*arg);
                    Parsed
                },
                _ => fail!("Value {:?} / {:?}", opt, opt.action),
            };
            match res {
                Parsed => continue,
                _ => return res,
            }
        }
        for opt in ctx.parser.options.iter() {
            let res;
            match opt.action {
                Many(ref action) | Push(ref action) => {
                    res = match ctx.options.find(opt) {
                        Some(lst) => action.parse_args(lst.as_slice()),
                        None => action.parse_args(&[]),
                    };
                }
                _ => continue, // No postprocessing needed
            };
            match res {
                Parsed => continue,
                _ => return res,
            }
        }
        return Parsed;
    }
}

pub struct Ref<'a, 'b, T> {
    priv cell: Rc<RefCell<&'a mut T>>,
    priv parser: &'a mut ArgumentParser<'b>,
}

impl<'a, 'b, T> Ref<'a, 'b, T> {

    pub fn add_option<'x>(&'x mut self, names: ~[&'b str],
        action: ~TypedAction<T>, help: &'b str)
        -> &'x mut Ref<'a, 'b, T>
    {
        let opt = Rc::new(GenericOption {
            id: self.parser.options.len(),
            name: Dash(names.clone()),
            help: help,
            action: action.bind(self.cell.clone()),
            });

        for nameptr in names.iter() {
            let name = *nameptr;
            match ArgumentKind::check(name) {
                Positional|Delimiter => {
                    fail!("Bad argument name {}", name);
                }
                LongOption => {
                    self.parser.long_options.insert(
                        name.to_str(), opt.clone());
                }
                ShortOption => {
                    if name.len() > 2 {
                        fail!("Bad short argument {}", name);
                    }
                    self.parser.short_options.insert(
                        name[1] as char, opt.clone());
                }
            }
        }
        self.parser.options.push(opt);
        return self;
    }

    pub fn add_argument<'x>(&'x mut self, name: &'b str,
        action: ~TypedAction<T>, help: &'b str)
    {
        let act = action.bind(self.cell.clone());
        let opt = Rc::new(GenericOption {
            id: self.parser.options.len(),
            name: Pos(name),
            help: help,
            action: act,
            });
        self.parser.options.push(opt.clone());
        match opt.action {
            Flag(_) => fail!("Flag arguments can't be positional"),
            Many(_) | Push(_) => {
                match self.parser.catchall_argument {
                    Some(ref y) => fail!(format!(
                        "Option {} conflicts with option {}",
                        name, y.name)),
                    None => {},
                }
                self.parser.catchall_argument = Some(opt);
            }
            Single(_) => {
                self.parser.arguments.push(opt);
            }
        }
    }
}

pub struct ArgumentParser<'a> {
    priv options: ~[Rc<GenericOption<'a>>],
    priv arguments: ~[Rc<GenericOption<'a>>],
    priv catchall_argument: Option<Rc<GenericOption<'a>>>,
    priv short_options: HashMap<char, Rc<GenericOption<'a>>>,
    priv long_options: HashMap<~str, Rc<GenericOption<'a>>>,
}



impl<'a> ArgumentParser<'a> {

    pub fn new() -> ArgumentParser {
        return ArgumentParser {
            arguments: ~[],
            catchall_argument: None,
            options: ~[],
            short_options: HashMap::new(),
            long_options: HashMap::new(),
            };
    }

    pub fn refer<'x, T>(&'x mut self, val: &'x mut T)
        -> ~Ref<'x, 'a, T>
    {
        return ~Ref {
            cell: Rc::new(RefCell::new(val)),
            parser: self,
        };
    }

    pub fn parse_list(&self, args: ~[~str]) -> Result<(), int> {
        match Context::parse(self, args) {
            Parsed => return Ok(()),
            Exit => return Err(0),
            Error(val) => {
                self.error(args[0], val);
                return Err(2);
            }
        }
    }

    fn error(&self, command: &str, message: &str) {
        println!("{}: {}", command, message);
    }

    pub fn parse_args(&self) -> Result<(), int> {
        return self.parse_list(os::args());
    }
}

