use std::cell::RefCell;
use std::rc::Rc;
use std::ops::{Add, Sub};

use super::{IncrBy, DecrBy};
use super::action::{TypedAction, Action, ParseResult};
use super::action::ParseResult::Parsed;
use super::action::IFlagAction;
use super::action::Action::Flag;

pub struct IncrByAction<'a, T: 'a> {
    delta: T,
    cell: Rc<RefCell<&'a mut T>>,
}

pub struct DecrByAction<'a, T: 'a> {
    delta: T,
    cell: Rc<RefCell<&'a mut T>>,
}

impl<T: 'static + Add<Output = T> + Clone> TypedAction<T> for IncrBy<T> {
    fn bind<'x>(&self, cell: Rc<RefCell<&'x mut T>>) -> Action<'x> {
        let IncrBy(ref delta) = *self;
        return Flag(Box::new(IncrByAction { cell: cell, delta: delta.clone() }));
    }
}

impl<T: 'static + Sub<Output = T> + Clone> TypedAction<T> for DecrBy<T> {
    fn bind<'x>(&self, cell: Rc<RefCell<&'x mut T>>) -> Action<'x> {
        let DecrBy(ref delta) = *self;
        return Flag(Box::new(DecrByAction { cell: cell, delta: delta.clone() }));
    }
}

impl<'a, T: Add<Output = T> + Clone> IFlagAction for IncrByAction<'a, T> {
    fn parse_flag(&self) -> ParseResult {
        let oldval = {
            let targ = self.cell.borrow();
            targ.clone()
        };
        let mut targ = self.cell.borrow_mut();
        **targ = oldval + self.delta.clone();
        return Parsed;
    }
}

impl<'a, T: Sub<Output = T> + Clone> IFlagAction for DecrByAction<'a, T> {
    fn parse_flag(&self) -> ParseResult {
        let oldval = {
            let targ = self.cell.borrow();
            targ.clone()
        };
        let mut targ = self.cell.borrow_mut();
        **targ = oldval - self.delta.clone();
        return Parsed;
    }
}

