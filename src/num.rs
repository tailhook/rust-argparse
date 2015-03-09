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

impl<T: 'static + Add<Output = T> + Copy> TypedAction<T> for IncrBy<T> {
    fn bind<'x>(&self, cell: Rc<RefCell<&'x mut T>>) -> Action<'x> {
        let IncrBy(delta) = *self;
        return Flag(Box::new(IncrByAction { cell: cell, delta: delta }));
    }
}

impl<T: 'static + Sub<Output = T> + Copy> TypedAction<T> for DecrBy<T> {
    fn bind<'x>(&self, cell: Rc<RefCell<&'x mut T>>) -> Action<'x> {
        let DecrBy(delta) = *self;
        return Flag(Box::new(DecrByAction { cell: cell, delta: delta }));
    }
}

impl<'a, T: Add<Output = T> + Copy> IFlagAction for IncrByAction<'a, T> {
    fn parse_flag(&self) -> ParseResult {
        let mut targ = self.cell.borrow_mut();
        let oldval = **targ;
        **targ = oldval + self.delta;
        return Parsed;
    }
}

impl<'a, T: Sub<Output = T> + Copy> IFlagAction for DecrByAction<'a, T> {
    fn parse_flag(&self) -> ParseResult {
        let mut targ = self.cell.borrow_mut();
        let oldval = **targ;
        **targ = oldval - self.delta;
        return Parsed;
    }
}

