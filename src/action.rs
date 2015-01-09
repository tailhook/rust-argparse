use std::cell::RefCell;
use std::rc::Rc;

pub enum ParseResult {
    Parsed,
    Help,
    Exit,
    Error(String),
}


pub enum Action<'a> {
    Flag(Box<IFlagAction + 'a>),
    Single(Box<IArgAction + 'a>),
    Push(Box<IArgsAction + 'a>),
    Many(Box<IArgsAction + 'a>),
}

pub trait TypedAction<T> {
    fn bind<'x>(&self, Rc<RefCell<&'x mut T>>) -> Action<'x>;
}

pub trait IFlagAction {
    fn parse_flag(&self) -> ParseResult;
}

pub trait IArgAction {
    fn parse_arg(&self, arg: &str) -> ParseResult;
}

pub trait IArgsAction {
    fn parse_args(&self, args: &[&str]) -> ParseResult;
}
