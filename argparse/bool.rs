use std::cell::RefCell;
use std::rc::Rc;

use action::Action;
use action::TypedAction;
use action::Flag;
use generic::StoreConstAction;
use generic::Store;

mod generic;

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

