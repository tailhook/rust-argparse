use std::cell::RefCell;
use std::rc::Rc;

use super::action::Action;
use super::action::TypedAction;
use super::action::Flag;
use super::generic::StoreConstAction;
use super::generic::Store;

pub struct StoreTrue;
pub struct StoreFalse;
pub type StoreBool = Store<bool>;

impl TypedAction<bool> for StoreTrue {
    fn bind<'x>(&self, cell: Rc<RefCell<&'x mut bool>>) -> Action {
        return Flag(~StoreConstAction { cell: cell, value: true });
    }
}

impl TypedAction<bool> for StoreFalse {
    fn bind<'x>(&self, cell: Rc<RefCell<&'x mut bool>>) -> Action {
        return Flag(~StoreConstAction { cell: cell, value: false });
    }
}

