use std::cell::RefCell;

use parser::Res;
use action::Action;
use action::{TypedAction, IFlagAction, IArgAction};
use action::{Flag, Single};

mod action;

pub trait ParseArgument {
    fn parse(cell: &RefCell<&mut Self>, arg: &str) -> Res;
}

pub struct StoreConst<T>(T);

pub struct Store<T>;

pub struct StoreConstAction<'a, T> {
    value: T,
    cell: &'a RefCell<&'a mut T>,
}

pub struct StoreAction<'a, T> {
    cell: &'a RefCell<&'a mut T>,
}

impl<T: 'static + Copy> TypedAction<T> for StoreConst<T> {
    fn bind<'x>(&self, cell: &'x RefCell<&'x mut T>) -> Action {
        let StoreConst(val) = *self;
        return Flag(~StoreConstAction { cell: cell, value: val });
    }
}

impl<T: 'static + ParseArgument> TypedAction<T> for Store<T> {
    fn bind<'x>(&self, cell: &'x RefCell<&'x mut T>) -> Action {
        return Single(~StoreAction { cell: cell });
    }
}

impl<'a, T: Copy> IFlagAction for StoreConstAction<'a, T> {
    fn parse_flag(&self) -> Res {
        let mut targ = self.cell.borrow_mut();
        **targ = self.value;
        return Ok(());
    }
}

impl<'a, T: ParseArgument> IArgAction for StoreAction<'a, T> {
    fn parse_arg(&self, arg: &str) -> Res {
        return ParseArgument::parse(self.cell, arg);
    }
}
