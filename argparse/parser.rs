use std::os;
use std::io::IoResult;
use std::io::stdio::{stdout, stderr};
use std::rc::Rc;
use std::cell::RefCell;
use std::iter::Peekable;
use std::slice::Items;
use std::fmt::{Show, Formatter};
use std::hash::Hash;
use std::hash::sip::SipState;
use std::ascii::StrAsciiExt;

use collections::hashmap::HashMap;
use collections::hashmap::HashSet;

use action::Action;
use action::{ParseResult, Parsed, Exit, Error};
use action::TypedAction;
use action::{Flag, Single, Push, Many};
use action::IArgAction;
use help::wrap_text;


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
    varid: uint,
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

pub struct Var<'parser> {
    priv id: uint,
    priv metavar: &'parser str,
    priv required: bool,
}

impl<'parser> Hash for Var<'parser> {
    fn hash(&self, state: &mut SipState) {
        state.write_uint(self.id).unwrap();
    }
}

impl<'parser> Eq for Var<'parser> {
    fn eq(&self, other: &Var<'parser>) -> bool {
        return self.id == other.id;
    }
}

impl<'a> TotalEq for Var<'a> {}

pub struct Context<'ctx, 'parser> {
    parser: &'ctx ArgumentParser<'parser>,
    set_vars: HashSet<Rc<Var<'parser>>>,
    list_options: HashMap<Rc<GenericOption<'parser>>, Vec<&'ctx str>>,
    arguments: Vec<&'ctx str>,
    iter: Peekable<&'ctx ~str, Items<'ctx, ~str>>,
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
        self.set_vars.insert(self.parser.vars[opt.varid].clone());
        match opt.action {
            Single(ref action) => {
                return action.parse_arg(value);
            }
            Push(_) => {
                let vec = self.list_options.find_or_insert(
                    opt.clone(), Vec::new());
                vec.push(value);
                return Parsed;
            }
            Many(_) => {
                let vec = self.list_options.find_or_insert(
                    opt.clone(), Vec::new());
                vec.push(value);
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
                                self.set_vars.insert(
                                    self.parser.vars[opt.varid].clone());
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
                Flag(ref action) => {
                    self.set_vars.insert(
                        self.parser.vars[opt.varid].clone());
                    action.parse_flag()
                }
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
            set_vars: HashSet::new(),
            list_options: HashMap::new(),
            arguments: Vec::new(),
        };

        // Parsing options, postponing positional arguments
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

        // Parse positional arguments
        let mut pargs = ctx.parser.arguments.iter();
        for arg in ctx.arguments.iter() {
            let mut opt;
            loop {
                match pargs.next() {
                    Some(option) => {
                        if ctx.set_vars.contains(&ctx.parser.vars[option.varid]) {
                            continue;
                        }
                        opt = option;
                        break;
                    }
                    None => match ctx.parser.catchall_argument {
                        Some(ref option) => {
                            opt = option;
                            break;
                        }
                        None => return Error(format!(
                            "Unexpected argument {}", arg)),
                    }
                };
            }
            let res = match opt.action {
                Single(ref act) => {
                    ctx.set_vars.insert(
                        ctx.parser.vars[opt.varid].clone());
                    act.parse_arg(*arg)
                },
                Many(_) | Push(_) => {
                    ctx.list_options.find_or_insert(
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

        // Parse list_arguments, which were collected before
        for (opt, lst) in ctx.list_options.iter() {
            match opt.action {
                Push(ref act) | Many(ref act) => {
                    let res = act.parse_args(lst.as_slice());
                    match res {
                        Parsed => continue,
                        _ => return res,
                    }
                }
                _ => fail!(),
            }
        }
        return Parsed;
    }
}

pub struct Ref<'refer, 'parser, T> {
    priv cell: Rc<RefCell<&'refer mut T>>,
    priv varid: uint,
    priv parser: &'refer mut ArgumentParser<'parser>,
}

impl<'a, 'b, T> Ref<'a, 'b, T> {

    pub fn add_option<'x>(&'x mut self, names: ~[&'b str],
        action: ~TypedAction<T>, help: &'b str)
        -> &'x mut Ref<'a, 'b, T>
    {
        let opt = Rc::new(GenericOption {
            id: self.parser.options.len(),
            varid: self.varid,
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
            varid: self.varid,
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
    priv description: &'a str,
    priv vars: ~[Rc<Var<'a>>],
    priv options: ~[Rc<GenericOption<'a>>],
    priv arguments: ~[Rc<GenericOption<'a>>],
    priv catchall_argument: Option<Rc<GenericOption<'a>>>,
    priv short_options: HashMap<char, Rc<GenericOption<'a>>>,
    priv long_options: HashMap<~str, Rc<GenericOption<'a>>>,
}



impl<'parser> ArgumentParser<'parser> {

    pub fn new() -> ArgumentParser {
        return ArgumentParser {
            description: "",
            vars: ~[],
            arguments: ~[],
            catchall_argument: None,
            options: ~[],
            short_options: HashMap::new(),
            long_options: HashMap::new(),
            };
    }

    pub fn refer<'x, T>(&'x mut self, val: &'x mut T)
        -> ~Ref<'x, 'parser, T>
    {
        let cell = Rc::new(RefCell::new(val));
        let id = self.vars.len();
        self.vars.push(Rc::new(Var {
                id: id,
                required: false,
                metavar: "",
                }));
        return ~Ref {
            cell: cell.clone(),
            varid: id,
            parser: self,
        };
    }

    pub fn set_description(&mut self, descr: &'parser str) {
        self.description = descr;
    }

    pub fn print_help(&self, name: &str, writer: &mut Writer) -> IoResult<()> {
        return HelpFormatter::print_help(self, name, writer);
    }

    pub fn print_usage(&self, name: &str, writer: &mut Writer) -> IoResult<()>
    {
        return HelpFormatter::print_usage(self, name, writer);
    }

    pub fn parse(&self, args: ~[~str],
        stdout: &mut Writer, stderr: &mut Writer)
        -> Result<(), int>
    {
        match Context::parse(self, args) {
            Parsed => return Ok(()),
            Exit => return Err(0),
            Error(message) => {
                self.error(args[0], message, stderr);
                return Err(2);
            }
        }
    }

    pub fn error(&self, command: &str, message: &str, writer: &mut Writer) {
        self.print_usage(command, writer).unwrap();
        writer.write_str(format!("{}: {}\n", command, message)).unwrap();
    }

    pub fn parse_args(&self) -> Result<(), int> {
        return self.parse(os::args(), &mut stdout(), &mut stderr());
    }
}

pub struct HelpFormatter<'a, 'b> {
    name: &'a str,
    parser: &'a ArgumentParser<'b>,
    buf: &'a mut Writer,
}

impl<'a, 'b> HelpFormatter<'a, 'b> {
    pub fn print_usage(parser: &ArgumentParser, name: &str, writer: &mut Writer)
        -> IoResult<()>
    {
        return HelpFormatter { parser: parser, name: name, buf: writer }
            .write_usage();
    }

    pub fn print_help(parser: &ArgumentParser, name: &str, writer: &mut Writer)
        -> IoResult<()>
    {
        return HelpFormatter { parser: parser, name: name, buf: writer }
            .write_help();
    }

    fn write_help(&mut self) -> IoResult<()> {
        try!(self.write_usage());
        try!(self.buf.write_char('\n'));
        if self.parser.description.len() > 0 {
            try!(wrap_text(self.buf, self.parser.description, 79, 0, 0));
            try!(self.buf.write_char('\n'));
        }
        return Ok(());
    }

    fn write_usage(&mut self) -> IoResult<()> {
        try!(self.buf.write_str("Usage:\n    "));
        try!(self.buf.write(self.name.as_bytes()));
        if self.parser.options.len() != 0 {
            if self.parser.short_options.len() > 0
                || self.parser.long_options.len() > 0
            {
                try!(self.buf.write_str(" [options]"));
            }
            for opt in self.parser.arguments.iter() {
                match opt.name {
                    Pos(name) => {
                        try!(self.buf.write_str(" ["));
                        try!(self.buf.write_str(name.to_ascii_upper()));
                        try!(self.buf.write_char(']'));
                    }
                    _ => {}
                }
            }
            match self.parser.catchall_argument {
                Some(ref opt) => {
                    match opt.name {
                        Pos(name) => {
                            try!(self.buf.write_str(" ["));
                            try!(self.buf.write_str(name.to_ascii_upper()));
                            try!(self.buf.write_str(" ...]"));
                        }
                        _ => {}
                    }
                }
                None => {}
            }
        }
        try!(self.buf.write_char('\n'));
        return Ok(());
    }

}
