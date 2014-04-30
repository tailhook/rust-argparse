use std::cell::RefCell;

pub enum ParseResult {
    Parsed,
    Exit,
    Error(~str),
}


pub enum Action {
    Flag(~IFlagAction),
    Single(~IArgAction),
    Many(~IArgsAction),
}

pub trait TypedAction<T> {
    fn bind<'x>(&self, &'x RefCell<&'x mut T>) -> Action;
}

pub trait IFlagAction {
    fn parse_flag(&self) -> ParseResult;
}

pub trait IArgAction {
    fn parse_arg(&self, arg: &str) -> ParseResult;
}

pub trait IArgsAction {
    fn parse_args(&self, args: ~[&str]) -> ParseResult;
}
