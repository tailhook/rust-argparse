use std::cell::RefCell;
use std::from_str::FromStr;
use std::rc::Rc;

use action::Action;
use action::{TypedAction, IFlagAction, IArgAction};
use action::{ParseResult, Parsed, Error};
use action::{Flag, Single};

mod action;

pub struct StoreConst<T>(T);

pub struct Store<T>;

pub struct StoreConstAction<'a, T> {
    value: T,
    cell: Rc<RefCell<&'a mut T>>,
}

pub struct StoreAction<'a, T> {
    cell: Rc<RefCell<&'a mut T>>,
}

impl<T: 'static + Copy> TypedAction<T> for StoreConst<T> {
    fn bind<'x>(&self, cell: Rc<RefCell<&'x mut T>>) -> Action {
        let StoreConst(val) = *self;
        return Flag(~StoreConstAction { cell: cell, value: val });
    }
}

impl<T: 'static + FromStr> TypedAction<T> for Store<T> {
    fn bind<'x>(&self, cell: Rc<RefCell<&'x mut T>>) -> Action {
        return Single(~StoreAction { cell: cell });
    }
}

impl<'a, T: Copy> IFlagAction for StoreConstAction<'a, T> {
    fn parse_flag(&self) -> ParseResult {
        let mut targ = self.cell.borrow_mut();
        **targ = self.value;
        return Parsed;
    }
}

impl<'a, T: FromStr> IArgAction for StoreAction<'a, T> {
    fn parse_arg(&self, arg: &str) -> ParseResult {
        match FromStr::from_str(arg) {
            Some(x) => {
                **self.cell.borrow_mut() = x;
                return Parsed;
            }
            None => {
                return Error(format!("Bad value {}", arg));
            }
        }
    }
}
