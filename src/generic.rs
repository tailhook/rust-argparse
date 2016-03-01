use std::cell::RefCell;
use std::str::FromStr;
use std::rc::Rc;

use super::{StoreConst, Store, StoreOption, List, Collect, PushConst};
use super::action::Action;
use super::action::{TypedAction, IFlagAction, IArgAction, IArgsAction};
use super::action::ParseResult;
use super::action::ParseResult::{Parsed, Error};
use super::action::Action::{Flag, Single, Push, Many};

pub struct StoreConstAction<'a, T: 'a> {
    pub value: T,
    pub cell: Rc<RefCell<&'a mut T>>,
}

pub struct PushConstAction<'a, T: 'a> {
    pub value: T,
    pub cell: Rc<RefCell<&'a mut Vec<T>>>,
}

pub struct StoreAction<'a, T: 'a> {
    pub cell: Rc<RefCell<&'a mut T>>,
}

pub struct StoreOptionAction<'a, T: 'a> {
    cell: Rc<RefCell<&'a mut Option<T>>>,
}

pub struct ListAction<'a, T: 'a> {
    cell: Rc<RefCell<&'a mut Vec<T>>>,
}

impl<T: 'static + Clone> TypedAction<T> for StoreConst<T> {
    fn bind<'x>(&self, cell: Rc<RefCell<&'x mut T>>) -> Action<'x> {
        let StoreConst(ref val) = *self;
        return Flag(Box::new(StoreConstAction { cell: cell, value: val.clone() }));
    }
}

impl<T: 'static + Clone> TypedAction<Vec<T>> for PushConst<T> {
    fn bind<'x>(&self, cell: Rc<RefCell<&'x mut Vec<T>>>) -> Action<'x> {
        let PushConst(ref val) = *self;
        return Flag(Box::new(PushConstAction { cell: cell, value: val.clone() }));
    }
}

impl<T: 'static + FromStr> TypedAction<T> for Store {
    fn bind<'x>(&self, cell: Rc<RefCell<&'x mut T>>) -> Action<'x> {
        return Single(Box::new(StoreAction { cell: cell }));
    }
}

impl<T: 'static + FromStr> TypedAction<Option<T>> for StoreOption {
    fn bind<'x>(&self, cell: Rc<RefCell<&'x mut Option<T>>>) -> Action<'x> {
        return Single(Box::new(StoreOptionAction { cell: cell }));
    }
}

impl<T: 'static + FromStr + Clone> TypedAction<Vec<T>> for List {
    fn bind<'x>(&self, cell: Rc<RefCell<&'x mut Vec<T>>>) -> Action<'x> {
        return Many(Box::new(ListAction { cell: cell }));
    }
}

impl<T: 'static + FromStr + Clone> TypedAction<Vec<T>> for Collect {
    fn bind<'x>(&self, cell: Rc<RefCell<&'x mut Vec<T>>>) -> Action<'x> {
        return Push(Box::new(ListAction { cell: cell }));
    }
}

impl<'a, T: Clone> IFlagAction for StoreConstAction<'a, T> {
    fn parse_flag(&self) -> ParseResult {
        let mut targ = self.cell.borrow_mut();
        **targ = self.value.clone();
        return Parsed;
    }
}

impl<'a, T: Clone> IFlagAction for PushConstAction<'a, T> {
    fn parse_flag(&self) -> ParseResult {
        let mut targ = self.cell.borrow_mut();
        targ.push(self.value.clone());
        return Parsed;
    }
}

impl<'a, T: FromStr> IArgAction for StoreAction<'a, T> {
    fn parse_arg(&self, arg: &str) -> ParseResult {
        match FromStr::from_str(arg) {
            Ok(x) => {
                **self.cell.borrow_mut() = x;
                return Parsed;
            }
            Err(_) => {
                return Error(format!("Bad value {}", arg));
            }
        }
    }
}

impl<'a, T: FromStr> IArgAction for StoreOptionAction<'a, T> {
    fn parse_arg(&self, arg: &str) -> ParseResult {
        match FromStr::from_str(arg) {
            Ok(x) => {
                **self.cell.borrow_mut() = Some(x);
                return Parsed;
            }
            Err(_) => {
                return Error(format!("Bad value {}", arg));
            }
        }
    }
}

impl<'a, T: FromStr + Clone> IArgsAction for ListAction<'a, T> {
    fn parse_args(&self, args: &[&str]) -> ParseResult {
        let mut result = vec!();
        for arg in args.iter() {
            match FromStr::from_str(*arg) {
                Ok(x) => {
                    result.push(x);
                }
                Err(_) => {
                    return Error(format!("Bad value {}", arg));
                }
            }
        }
        **self.cell.borrow_mut() = result;
        return Parsed;
    }
}

