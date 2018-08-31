use std::env;
use std::io::{Write};
use std::io::Result as IoResult;
use std::io::{stdout, stderr};
use std::rc::Rc;
use std::cell::RefCell;
use std::iter::Peekable;
use std::slice::Iter;
use std::hash::Hash;
use std::hash::Hasher;
use std::str::FromStr;
use std::process::exit;

#[allow(unused_imports)] #[allow(deprecated)]
use std::ascii::AsciiExt;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::collections::HashSet;

use super::action::{Action, ParseResult};
use super::action::ParseResult::{Parsed, Help, Exit, Error};
use super::action::TypedAction;
use super::action::Action::{Flag, Single, Push, Many};
use super::action::IArgAction;
use super::generic::StoreAction;
use super::help::{HelpAction, wrap_text};
use action::IFlagAction;

use self::ArgumentKind::{Positional, ShortOption, LongOption, Delimiter};


static OPTION_WIDTH: usize = 24;
static TOTAL_WIDTH: usize = 79;


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

struct GenericArgument<'parser> {
    id: usize,
    varid: usize,
    name: &'parser str,
    help: &'parser str,
    action: Action<'parser>,
}

struct GenericOption<'parser> {
    id: usize,
    varid: Option<usize>,
    names: Vec<&'parser str>,
    help: &'parser str,
    action: Action<'parser>,
}

struct EnvVar<'parser> {
    varid: usize,
    name: &'parser str,
    action: Box<IArgAction + 'parser>,
}

impl<'a> Hash for GenericOption<'a> {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.id.hash(state);
    }
}

impl<'a> PartialEq for GenericOption<'a> {
    fn eq(&self, other: &GenericOption<'a>) -> bool {
        return self.id == other.id;
    }
}

impl<'a> Eq for GenericOption<'a> {}

impl<'a> Hash for GenericArgument<'a> {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.id.hash(state);
    }
}

impl<'a> PartialEq for GenericArgument<'a> {
    fn eq(&self, other: &GenericArgument<'a>) -> bool {
        return self.id == other.id;
    }
}

impl<'a> Eq for GenericArgument<'a> {}

pub struct Var {
    id: usize,
    metavar: String,
    required: bool,
}

impl Hash for Var {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.id.hash(state);
    }
}

impl PartialEq for Var {
    fn eq(&self, other: &Var) -> bool {
        return self.id == other.id;
    }
}

impl Eq for Var {}

struct Context<'ctx, 'parser: 'ctx> {
    parser: &'ctx ArgumentParser<'parser>,
    set_vars: HashSet<usize>,
    list_options: HashMap<Rc<GenericOption<'parser>>, Vec<&'ctx str>>,
    list_arguments: HashMap<Rc<GenericArgument<'parser>>, Vec<&'ctx str>>,
    arguments: Vec<&'ctx str>,
    iter: Peekable<Iter<'ctx, String>>,
    stderr: &'ctx mut (Write + 'ctx),
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
                    &value[..]
                }
                None => {
                    return match opt.action {
                        Many(_) => Parsed,
                        _ => Error(format!(
                            // TODO(tailhook) is {:?} ok?
                            "Option {:?} requires an argument", opt.names)),
                    };
                }
            },
        };
        match opt.varid {
            Some(varid) => { self.set_vars.insert(varid); }
            None => {}
        }
        match opt.action {
            Single(ref action) => {
                return action.parse_arg(value);
            }
            Push(_) => {
                (match self.list_options.entry(opt.clone()) {
                    Entry::Occupied(occ) => occ.into_mut(),
                    Entry::Vacant(vac) => vac.insert(Vec::new()),
                }).push(value);
                return Parsed;
            }
            Many(_) => {
                let vec = match self.list_options.entry(opt.clone()) {
                    Entry::Occupied(occ) => occ.into_mut(),
                    Entry::Vacant(vac) => vac.insert(Vec::new()),
                };
                vec.push(value);
                match optarg {
                    Some(_) => return Parsed,
                    _ => {}
                }
                loop {
                    match self.iter.peek() {
                        None => { break; }
                        Some(arg) if arg.starts_with("-") => {
                            break;
                        }
                        Some(value) => {
                            vec.push(&value[..]);
                        }
                    }
                    self.iter.next();
                }
                return Parsed;
            }
            _ => panic!(),
        };
    }

    fn parse_long_option(&mut self, arg: &'a str) -> ParseResult {
        let mut equals_iter = arg.splitn(2, '=');
        let optname = equals_iter.next().unwrap();
        let valueref = equals_iter.next();
        let opt = self.parser.long_options.get(&optname.to_string());
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
                                match opt.varid {
                                    Some(varid) => {
                                        self.set_vars.insert(varid);
                                    }
                                    None => {}
                                }
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
            let opt = match self.parser.short_options.get(&ch) {
                Some(opt) => { opt }
                None => {
                    return Error(format!("Unknown short option \"{}\"", ch));
                }
            };
            let res = match opt.action {
                Flag(ref action) => {
                    match opt.varid {
                        Some(varid) => { self.set_vars.insert(varid); }
                        None => {}
                    }
                    action.parse_flag()
                }
                Single(_) | Push(_) | Many(_) => {
                    let value;
                    if idx + 1 < arg.len() {
                        value = Some(&arg[idx+1..arg.len()]);
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

    fn parse_options(&mut self) -> ParseResult {
        self.iter.next();  // Command name
        loop {
            let next = self.iter.next();
            let arg = match next {
                Some(arg) => { arg }
                None => { break; }
            };
            let res = match ArgumentKind::check(&arg[..]) {
                Positional => {
                    self.postpone_argument(&arg[..]);
                    if self.parser.stop_on_first_argument {
                        break;
                    }
                    continue;
                }
                LongOption => self.parse_long_option(&arg[..]),
                ShortOption => self.parse_short_options(&arg[..]),
                Delimiter => {
                    if !self.parser.silence_double_dash {
                        self.postpone_argument("--");
                    }
                    break;
                }
            };
            match res {
                Parsed => continue,
                _ => return res,
            }
        }

        loop {
            match self.iter.next() {
                None => break,
                Some(arg) => self.postpone_argument(&arg[..]),
            }
        }
        return Parsed;
    }

    fn parse_arguments(&mut self) -> ParseResult {
        let mut pargs = self.parser.arguments.iter();
        for arg in self.arguments.iter() {
            let opt;
            loop {
                match pargs.next() {
                    Some(option) => {
                        if self.set_vars.contains(&option.varid) {
                            continue;
                        }
                        opt = option;
                        break;
                    }
                    None => match self.parser.catchall_argument {
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
                    self.set_vars.insert(opt.varid);
                    act.parse_arg(*arg)
                },
                Many(_) | Push(_) => {
                    (match self.list_arguments.entry(opt.clone()) {
                        Entry::Occupied(occ) => occ.into_mut(),
                        Entry::Vacant(vac) => vac.insert(Vec::new()),
                    }).push(*arg);
                    Parsed
                },
                _ => unreachable!(),
            };
            match res {
                Parsed => continue,
                _ => return res,
            }
        }
        return Parsed;
    }

    fn parse_list_vars(&mut self) -> ParseResult {
        for (opt, lst) in self.list_options.iter() {
            match opt.action {
                Push(ref act) | Many(ref act) => {
                    let res = act.parse_args(&lst[..]);
                    match res {
                        Parsed => continue,
                        _ => return res,
                    }
                }
                _ => panic!(),
            }
        }
        for (opt, lst) in self.list_arguments.iter() {
            match opt.action {
                Push(ref act) | Many(ref act) => {
                    let res = act.parse_args(&lst[..]);
                    match res {
                        Parsed => continue,
                        _ => return res,
                    }
                }
                _ => panic!(),
            }
        }
        return Parsed;
    }

    fn parse_env_vars(&mut self) -> ParseResult {
        for evar in self.parser.env_vars.iter() {
            match env::var(evar.name) {
                Ok(val) => {
                    match evar.action.parse_arg(&val[..]) {
                        Parsed => {
                            self.set_vars.insert(evar.varid);
                            continue;
                        }
                        Error(err) => {
                            write!(self.stderr,
                                "WARNING: Environment variable {}: {}\n",
                                evar.name, err).ok();
                        }
                        _ => unreachable!(),
                    }
                }
                Err(_) => {}
            }
        }
        return Parsed;
    }

    fn check_required(&mut self) -> ParseResult {
        // Check for required arguments
        for var in self.parser.vars.iter() {
            if var.required && !self.set_vars.contains(&var.id) {
                // First try positional arguments
                for opt in self.parser.arguments.iter() {
                    if opt.varid == var.id {
                        return Error(format!(
                            "Argument {} is required", opt.name));
                    }
                }
                // Then options
                let mut all_options = vec!();
                for opt in self.parser.options.iter() {
                    match opt.varid {
                        Some(varid) if varid == var.id => {}
                        _ => { continue }
                    }
                    all_options.extend(opt.names.clone().into_iter());
                }
                if all_options.len() > 1 {
                    return Error(format!(
                        "One of the options {:?} is required", all_options));
                } else if all_options.len() == 1 {
                    return Error(format!(
                        "Option {:?} is required", all_options));
                }
                // Then envvars
                for envvar in self.parser.env_vars.iter() {
                    if envvar.varid == var.id {
                        return Error(format!(
                            "Environment var {} is required", envvar.name));
                    }
                }
            }
        }
        return Parsed;
    }

    fn parse(parser: &ArgumentParser, args: &Vec<String>, stderr: &mut Write)
        -> ParseResult
    {
        let mut ctx = Context {
            parser: parser,
            iter: args.iter().peekable(),
            set_vars: HashSet::new(),
            list_options: HashMap::new(),
            list_arguments: HashMap::new(),
            arguments: Vec::new(),
            stderr: stderr,
        };

        match ctx.parse_env_vars() {
            Parsed => {}
            x => { return x; }
        }

        match ctx.parse_options() {
            Parsed => {}
            x => { return x; }
        }

        match ctx.parse_arguments() {
            Parsed => {}
            x => { return x; }
        }

        match ctx.parse_list_vars() {
            Parsed => {}
            x => { return x; }
        }

        match ctx.check_required() {
            Parsed => {}
            x => { return x; }
        }

        return Parsed;
    }
}

pub struct Ref<'parser:'refer, 'refer, T: 'parser> {
    cell: Rc<RefCell<&'parser mut T>>,
    varid: usize,
    parser: &'refer mut ArgumentParser<'parser>,
}

impl<'parser, 'refer, T> Ref<'parser, 'refer, T> {

    pub fn add_option<'x, A: TypedAction<T>>(&'x mut self,
        names: &[&'parser str], action: A, help: &'parser str)
        -> &'x mut Ref<'parser, 'refer, T>
    {
        {
            let var = &mut self.parser.vars[self.varid];
            if var.metavar.len() == 0 {
                let mut longest_name = names[0];
                let mut llen = longest_name.len();
                for name in names.iter() {
                    if name.len() > llen {
                        longest_name = *name;
                        llen = longest_name.len();
                    }
                }
                if llen > 2 {
                    var.metavar = longest_name[2..llen]
                        .to_ascii_uppercase().replace("-", "_");
                }
            }
        }
        self.parser.add_option_for(Some(self.varid), names,
            action.bind(self.cell.clone()),
            help);
        return self;
    }

    pub fn add_argument<'x, A: TypedAction<T>>(&'x mut self,
        name: &'parser str, action: A, help: &'parser str)
        -> &'x mut Ref<'parser, 'refer, T>
    {
        let act = action.bind(self.cell.clone());
        let opt = Rc::new(GenericArgument {
            id: self.parser.arguments.len(),
            varid: self.varid,
            name: name,
            help: help,
            action: act,
            });
        match opt.action {
            Flag(_) => panic!("Flag arguments can't be positional"),
            Many(_) | Push(_) => {
                match self.parser.catchall_argument {
                    Some(ref y) => panic!(format!(
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
        {
            let var = &mut self.parser.vars[self.varid];
            if var.metavar.len() == 0 {
                var.metavar = name.to_string();
            }
        }
        return self;
    }

    pub fn metavar<'x>(&'x mut self, name: &str)
        -> &'x mut Ref<'parser, 'refer, T>
    {
        {
            let var = &mut self.parser.vars[self.varid];
            var.metavar = name.to_string();
        }
        return self;
    }

    pub fn required<'x>(&'x mut self)
        -> &'x mut Ref<'parser, 'refer, T>
    {
        {
            let var = &mut self.parser.vars[self.varid];
            var.required = true;
        }
        return self;
    }
}

impl<'parser, 'refer, T: 'static + FromStr> Ref<'parser, 'refer, T> {
    pub fn envvar<'x>(&'x mut self, varname: &'parser str)
        -> &'x mut Ref<'parser, 'refer, T>
    {
        self.parser.env_vars.push(Rc::new(EnvVar {
            varid: self.varid,
            name: varname,
            action: Box::new(StoreAction { cell: self.cell.clone() }),
            }));
        return self;
    }
}

/// The main argument parser class
pub struct ArgumentParser<'parser> {
    description: &'parser str,
    vars: Vec<Box<Var>>,
    options: Vec<Rc<GenericOption<'parser>>>,
    arguments: Vec<Rc<GenericArgument<'parser>>>,
    env_vars: Vec<Rc<EnvVar<'parser>>>,
    catchall_argument: Option<Rc<GenericArgument<'parser>>>,
    short_options: HashMap<char, Rc<GenericOption<'parser>>>,
    long_options: HashMap<String, Rc<GenericOption<'parser>>>,
    stop_on_first_argument: bool,
    silence_double_dash: bool,
}



impl<'parser> ArgumentParser<'parser> {

    /// Create an empty argument parser
    pub fn new() -> ArgumentParser<'parser> {

        let mut ap = ArgumentParser {
            description: "",
            vars: Vec::new(),
            env_vars: Vec::new(),
            arguments: Vec::new(),
            catchall_argument: None,
            options: Vec::new(),
            short_options: HashMap::new(),
            long_options: HashMap::new(),
            stop_on_first_argument: false,
            silence_double_dash: true,
            };
        ap.add_option_for(None, &["-h", "--help"], Flag(Box::new(HelpAction)),
            "Show this help message and exit");
        return ap;
    }

    /// Borrow mutable variable for an argument
    ///
    /// This returns `Ref` object which should be used configure the option
    pub fn refer<'x, T>(&'x mut self, val: &'parser mut T)
        -> Box<Ref<'parser, 'x, T>>
    {
        let cell = Rc::new(RefCell::new(val));
        let id = self.vars.len();
        self.vars.push(Box::new(Var {
                id: id,
                required: false,
                metavar: "".to_string(),
                }));
        return Box::new(Ref {
            cell: cell.clone(),
            varid: id,
            parser: self,
        });
    }

    /// Add option to argument parser
    ///
    /// This is only useful for options that don't store value. For
    /// example `Print(...)`
    pub fn add_option<F:IFlagAction + 'parser>(&mut self,
        names: &[&'parser str], action: F, help: &'parser str)
    {
        self.add_option_for(None, names, Flag(Box::new(action)), help);
    }

    /// Set description of the command
    pub fn set_description(&mut self, descr: &'parser str) {
        self.description = descr;
    }

    fn add_option_for(&mut self, var: Option<usize>,
        names: &[&'parser str],
        action: Action<'parser>, help: &'parser str)
    {
        let opt = Rc::new(GenericOption {
            id: self.options.len(),
            varid: var,
            names: names.to_vec(),
            help: help,
            action: action,
            });

        if names.len() < 1 {
            panic!("At least one name for option must be specified");
        }
        for nameptr in names.iter() {
            let name = *nameptr;
            match ArgumentKind::check(name) {
                Positional|Delimiter => {
                    panic!("Bad argument name {}", name);
                }
                LongOption => {
                    self.long_options.insert(
                        name.to_string(), opt.clone());
                }
                ShortOption => {
                    if name.len() > 2 {
                        panic!("Bad short argument {}", name);
                    }
                    self.short_options.insert(
                        name.as_bytes()[1] as char, opt.clone());
                }
            }
        }
        self.options.push(opt);
    }

    /// Print help
    ///
    /// Usually command-line option is used for printing help,
    /// this is here for any awkward cases
    pub fn print_help(&self, name: &str, writer: &mut Write) -> IoResult<()> {
        return HelpFormatter::print_help(self, name, writer);
    }

    /// Print usage
    ///
    /// Usually printed into stderr on error of command-line parsing
    pub fn print_usage(&self, name: &str, writer: &mut Write) -> IoResult<()>
    {
        return HelpFormatter::print_usage(self, name, writer);
    }

    /// Parse arguments
    ///
    /// This is most powerful method. Usually you need `parse_args`
    /// or `parse_args_or_exit` instead
    pub fn parse(&self, args: Vec<String>,
        stdout: &mut Write, stderr: &mut Write)
        -> Result<(), i32>
    {
        let name = if args.len() > 0 { &args[0][..] } else { "unknown" };
        match Context::parse(self, &args, stderr) {
            Parsed => return Ok(()),
            Exit => return Err(0),
            Help => {
                self.print_help(name, stdout).unwrap();
                return Err(0);
            }
            Error(message) => {
                self.error(&name[..], &message[..], stderr);
                return Err(2);
            }
        }
    }

    /// Write an error similar to one produced by the library itself
    ///
    /// Only needed if you like to do some argument validation that is out
    /// of scope of the argparse
    pub fn error(&self, command: &str, message: &str, writer: &mut Write) {
        self.print_usage(command, writer).unwrap();
        write!(writer, "{}: {}\n", command, message).ok();
    }

    /// Configure parser to ignore options when first non-option argument is
    /// encountered.
    ///
    /// Useful for commands that want to pass following options to the
    /// subcommand or subprocess, but need some options to be set before
    /// command is specified.
    pub fn stop_on_first_argument(&mut self, want_stop: bool) {
        self.stop_on_first_argument = want_stop;
    }

    /// Do not put double-dash (bare `--`) into argument
    ///
    /// The double-dash is used to stop parsing options and treat all the
    /// following tokens as the arguments regardless of whether they start
    /// with dash (minus) or not.
    ///
    /// The method only useful for `List` arguments. On by default. The method
    /// allows to set option to `false` so that `cmd xx -- yy` will get
    /// ``xx -- yy`` as arguments instead of ``xx yy`` by default. This is
    /// useful if your ``--`` argument is meaningful. Only first double-dash
    /// is ignored by default.
    pub fn silence_double_dash(&mut self, silence: bool) {
        self.silence_double_dash = silence;
    }

    /// Convenience method to parse arguments
    ///
    /// On error returns error code that is supposed to be returned by
    /// an application. (i.e. zero on `--help` and `2` on argument error)
    pub fn parse_args(&self) -> Result<(), i32> {
        // TODO(tailhook) can we get rid of collect?
        return self.parse(env::args().collect(),
            &mut stdout(), &mut stderr());
    }

    /// The simplest conveninece method
    ///
    /// The method returns only in case of successful parsing or exits with
    /// appropriate code (including successful on `--help`) otherwise.
    pub fn parse_args_or_exit(&self) {
        // TODO(tailhook) can we get rid of collect?
        self.parse(env::args().collect(), &mut stdout(), &mut stderr())
            .map_err(|c| exit(c))
            .ok();
    }
}

struct HelpFormatter<'a, 'b: 'a> {
    name: &'a str,
    parser: &'a ArgumentParser<'b>,
    buf: &'a mut (Write + 'a),
}

impl<'a, 'b> HelpFormatter<'a, 'b> {
    pub fn print_usage(parser: &ArgumentParser, name: &str, writer: &mut Write)
        -> IoResult<()>
    {
        return HelpFormatter { parser: parser, name: name, buf: writer }
            .write_usage();
    }

    pub fn print_help(parser: &ArgumentParser, name: &str, writer: &mut Write)
        -> IoResult<()>
    {
        return HelpFormatter { parser: parser, name: name, buf: writer }
            .write_help();
    }

    pub fn print_argument(&mut self, arg: &GenericArgument<'b>)
        -> IoResult<()>
    {
        let mut num = 2;
        try!(write!(self.buf, "  {}", arg.name));
        num += arg.name.len();
        if num >= OPTION_WIDTH {
            try!(write!(self.buf, "\n"));
            for _ in 0..OPTION_WIDTH {
                try!(write!(self.buf, " "));
            }
        } else {
            for _ in num..OPTION_WIDTH {
                try!(write!(self.buf, " "));
            }
        }
        try!(wrap_text(self.buf, arg.help, TOTAL_WIDTH, OPTION_WIDTH));
        try!(write!(self.buf, "\n"));
        return Ok(());
    }

    pub fn print_option(&mut self, opt: &GenericOption<'b>) -> IoResult<()> {
        let mut num = 2;
        try!(write!(self.buf, "  "));
        let mut niter = opt.names.iter();
        let name = niter.next().unwrap();
        try!(write!(self.buf, "{}", name));
        num += name.len();
        for name in niter {
            try!(write!(self.buf, ","));
            try!(write!(self.buf, "{}", name));
            num += name.len() + 1;
        }
        match opt.action {
            Flag(_) => {}
            Single(_) | Push(_) | Many(_) => {
                try!(write!(self.buf, " "));
                let var = &self.parser.vars[opt.varid.unwrap()];
                try!(write!(self.buf, "{}", &var.metavar[..]));
                num += var.metavar.len() + 1;
            }
        }
        if num >= OPTION_WIDTH {
            try!(write!(self.buf, "\n"));
            for _ in 0..OPTION_WIDTH {
                try!(write!(self.buf, " "));
            }
        } else {
            for _ in num..OPTION_WIDTH {
                try!(write!(self.buf, " "));
            }
        }
        try!(wrap_text(self.buf, opt.help, TOTAL_WIDTH, OPTION_WIDTH));
        try!(write!(self.buf, "\n"));
        return Ok(());
    }

    fn write_help(&mut self) -> IoResult<()> {
        try!(self.write_usage());
        try!(write!(self.buf, "\n"));
        if self.parser.description.len() > 0 {
            try!(wrap_text(self.buf, self.parser.description,TOTAL_WIDTH, 0));
            try!(write!(self.buf, "\n"));
        }
        if self.parser.arguments.len() > 0
            || self.parser.catchall_argument.is_some()
        {
            try!(write!(self.buf, "\nPositional arguments:\n"));
            for arg in self.parser.arguments.iter() {
                try!(self.print_argument(&**arg));
            }
            match self.parser.catchall_argument {
                Some(ref opt) => {
                    try!(self.print_argument(&**opt));
                }
                None => {}
            }
        }
        if self.parser.short_options.len() > 0
            || self.parser.long_options.len() > 0
        {
            try!(write!(self.buf, "\nOptional arguments:\n"));
            for opt in self.parser.options.iter() {
                try!(self.print_option(&**opt));
            }
        }
        return Ok(());
    }

    fn write_usage(&mut self) -> IoResult<()> {
        try!(write!(self.buf, "Usage:\n  "));
        try!(write!(self.buf, "{}", self.name));
        if self.parser.options.len() != 0 {
            if self.parser.short_options.len() > 1
                || self.parser.long_options.len() > 1
            {
                try!(write!(self.buf, " [OPTIONS]"));
            }
            for opt in self.parser.arguments.iter() {
                let var = &self.parser.vars[opt.varid];
                try!(write!(self.buf, " "));
                if !var.required {
                    try!(write!(self.buf, "["));
                }
                try!(write!(self.buf, "{}",
                    &opt.name.to_ascii_uppercase()[..]));
                if !var.required {
                    try!(write!(self.buf, "]"));
                }
            }
            match self.parser.catchall_argument {
                Some(ref opt) => {
                    let var = &self.parser.vars[opt.varid];
                    try!(write!(self.buf, " "));
                    if !var.required {
                        try!(write!(self.buf, "["));
                    }
                    try!(write!(self.buf, "{}",
                        &opt.name.to_ascii_uppercase()[..]));
                    if !var.required {
                        try!(write!(self.buf, " ...]"));
                    } else {
                        try!(write!(self.buf, " [...]"));
                    }
                }
                None => {}
            }
        }
        try!(write!(self.buf, "\n"));
        return Ok(());
    }

}
