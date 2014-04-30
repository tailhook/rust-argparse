use std::cell::RefCell;
use std::num::strconv::{from_str_bytes_common, ExpDec};

use parser::Res;
use action::{TypedAction, Action};
use action::IFlagAction;
use action::Flag;

pub struct IncrBy<T>(T);

pub struct DecrBy<T>(T);

pub struct IncrByAction<'a, T> {
    delta: T,
    cell: &'a RefCell<&'a mut T>,
}

pub struct DecrByAction<'a, T> {
    delta: T,
    cell: &'a RefCell<&'a mut T>,
}

impl<T: 'static + Add<T, T> + Copy> TypedAction<T> for IncrBy<T> {
    fn bind<'x>(&self, cell: &'x RefCell<&'x mut T>) -> Action {
        let IncrBy(delta) = *self;
        return Flag(~IncrByAction { cell: cell, delta: delta });
    }
}

impl<T: 'static + Sub<T, T> + Copy> TypedAction<T> for DecrBy<T> {
    fn bind<'x>(&self, cell: &'x RefCell<&'x mut T>) -> Action {
        let DecrBy(delta) = *self;
        return Flag(~DecrByAction { cell: cell, delta: delta });
    }
}

impl<'a, T: Add<T, T> + Copy> IFlagAction for IncrByAction<'a, T> {
    fn parse_flag(&self) -> Res {
        let mut targ = self.cell.borrow_mut();
        let oldval = **targ;
        **targ = oldval + self.delta;
        return Ok(());
    }
}

impl<'a, T: Sub<T, T> + Copy> IFlagAction for DecrByAction<'a, T> {
    fn parse_flag(&self) -> Res {
        let mut targ = self.cell.borrow_mut();
        let oldval = **targ;
        **targ = oldval - self.delta;
        return Ok(());
    }
}

