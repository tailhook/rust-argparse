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
    Collect(~ICollectAction),
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

pub trait ICollectAction {
    fn parse_arg(&self, arg: &str) -> ParseResult;
}
