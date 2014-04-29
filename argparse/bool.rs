use std::cell::RefCell;
use std::ascii::StrAsciiExt;

use parser::Res;
use action::Action;
use action::TypedAction;
use action::Flag;
use generic::StoreConstAction;
use generic::{Store, ParseArgument};

mod generic;

pub struct StoreTrue;
pub struct StoreFalse;
pub type StoreBool = Store<bool>;

impl TypedAction<bool> for StoreTrue {
    fn bind<'x>(&self, cell: &'x RefCell<&'x mut bool>) -> Action {
        return Flag(~StoreConstAction { cell: cell, value: true });
    }
}

impl TypedAction<bool> for StoreFalse {
    fn bind<'x>(&self, cell: &'x RefCell<&'x mut bool>) -> Action {
        return Flag(~StoreConstAction { cell: cell, value: false });
    }
}

impl ParseArgument for bool {
    fn parse(cell: &RefCell<&mut bool>, arg: &str) -> Res
    {
        let mut targ = cell.borrow_mut();
        let lower = arg.to_ascii_lower();
        let borrow: &str = lower;
        match borrow {
            "true" | "yes" | "y" | "t" | "enable" | "enabled" => {
                **targ = true;
            }
            "false" | "no" | "n" | "f" | "disable" | "disabled" | "" => {
                **targ = false;
            }
            _ => {
                return Err(format!("Wrong boolean value \"{}\"", arg));
            }
        }
        return Ok(());
    }
}
