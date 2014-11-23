use std::cell::RefCell;
use std::str::FromStr;
use std::rc::Rc;

use super::{StoreConst, Store, StoreOption, List, Collect};
use super::action::Action;
use super::action::{TypedAction, IFlagAction, IArgAction, IArgsAction};
use super::action::{ParseResult, Parsed, Error};
use super::action::{Flag, Single, Push, Many};

pub struct StoreConstAction<'a, T: 'a> {
    pub value: T,
    pub cell: Rc<RefCell<&'a mut T>>,
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

impl<T: 'static + Copy> TypedAction<T> for StoreConst<T> {
    fn bind<'x>(&self, cell: Rc<RefCell<&'x mut T>>) -> Action<'x> {
        let StoreConst(val) = *self;
        return Flag(box StoreConstAction { cell: cell, value: val });
    }
}

impl<T: 'static + FromStr> TypedAction<T> for Store<T> {
    fn bind<'x>(&self, cell: Rc<RefCell<&'x mut T>>) -> Action<'x> {
        return Single(box StoreAction { cell: cell });
    }
}

impl<T: 'static + FromStr> TypedAction<Option<T>> for StoreOption<T> {
    fn bind<'x>(&self, cell: Rc<RefCell<&'x mut Option<T>>>) -> Action<'x> {
        return Single(box StoreOptionAction { cell: cell });
    }
}

impl<T: 'static + FromStr + Clone> TypedAction<Vec<T>> for List<T> {
    fn bind<'x>(&self, cell: Rc<RefCell<&'x mut Vec<T>>>) -> Action<'x> {
        return Many(box ListAction { cell: cell });
    }
}

impl<T: 'static + FromStr + Clone> TypedAction<Vec<T>> for Collect<T> {
    fn bind<'x>(&self, cell: Rc<RefCell<&'x mut Vec<T>>>) -> Action<'x> {
        return Push(box ListAction { cell: cell });
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

impl<'a, T: FromStr> IArgAction for StoreOptionAction<'a, T> {
    fn parse_arg(&self, arg: &str) -> ParseResult {
        match FromStr::from_str(arg) {
            Some(x) => {
                **self.cell.borrow_mut() = Some(x);
                return Parsed;
            }
            None => {
                return Error(format!("Bad value {}", arg));
            }
        }
    }
}

impl<'a, T: FromStr + Clone> IArgsAction for ListAction<'a, T> {
    fn parse_args(&self, args: &[&str]) -> ParseResult {
        let mut result = box Vec::new();
        for arg in args.iter() {
            match FromStr::from_str(*arg) {
                Some(x) => {
                    result.push(x);
                }
                None => {
                    return Error(format!("Bad value {}", arg));
                }
            }
        }
        **self.cell.borrow_mut() = result.as_slice().to_vec();
        return Parsed;
    }
}

