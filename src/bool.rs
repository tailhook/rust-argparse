use std::cell::RefCell;
use std::rc::Rc;

use super::action::Action;
use super::action::TypedAction;
use super::action::Action::Flag;
use super::generic::StoreConstAction;
use super::{StoreTrue, StoreFalse};


impl TypedAction<bool> for StoreTrue {
    fn bind<'x>(&self, cell: Rc<RefCell<&'x mut bool>>) -> Action<'x> {
        return Flag(Box::new(StoreConstAction { cell: cell, value: true }));
    }
}

impl TypedAction<bool> for StoreFalse {
    fn bind<'x>(&self, cell: Rc<RefCell<&'x mut bool>>) -> Action<'x> {
        return Flag(Box::new(StoreConstAction { cell: cell, value: false }));
    }
}

