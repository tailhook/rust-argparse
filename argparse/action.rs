use std::cell::RefCell;

use parser::Res;

pub enum Action {
    Flag(~IFlagAction),
    Single(~IArgAction),
    Many(~IArgsAction),
}

pub trait TypedAction<T> {
    fn bind<'x>(&self, &'x RefCell<&'x mut T>) -> Action;
}

pub trait IFlagAction {
    fn parse_flag(&self) -> Res;
}

pub trait IArgAction {
    fn parse_arg(&self, arg: &str) -> Res;
}

pub trait IArgsAction {
    fn parse_args(&self, args: ~[&str]) -> Res;
}
