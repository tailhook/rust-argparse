use std::cell::RefCell;
use std::rc::Rc;

pub enum ParseResult {
    Parsed,
    Exit,
    Error(~str),
}


pub enum Action {
    Flag(~IFlagAction),
    Single(~IArgAction),
    Push(~IArgsAction),
    Many(~IArgsAction),
}

pub trait TypedAction<T> {
    fn bind<'x>(&self, Rc<RefCell<&'x mut T>>) -> Action;
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
