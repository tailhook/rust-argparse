use std::cell::RefCell;
use std::rc::Rc;

use super::action::Action;
use super::action::TypedAction;
use super::action::Flag;
use super::generic::StoreConstAction;
use super::{StoreTrue, StoreFalse};


impl TypedAction<bool> for StoreTrue {
    fn bind<'x>(&'x self, cell: Rc<RefCell<&'x mut bool>>) -> Action {
        return Flag(box StoreConstAction { cell: cell, value: true });
    }
}

impl TypedAction<bool> for StoreFalse {
    fn bind<'x>(&'x self, cell: Rc<RefCell<&'x mut bool>>) -> Action {
        return Flag(box StoreConstAction { cell: cell, value: false });
    }
}

