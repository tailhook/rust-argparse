#![crate_id = "argparse"]
#![crate_type = "lib"]

extern crate collections;
extern crate arena;

pub use self::parser::{ArgumentParser, Ref};
pub use self::bool::{StoreTrue, StoreFalse, StoreBool};
pub use self::num::{IncrBy, DecrBy};
pub use self::generic::{Store, StoreOption, StoreConst, List, Collect};

pub mod action;
mod generic;
mod parser;
mod help;

mod bool;
mod num;

#[cfg(test)] mod test_parser;
#[cfg(test)] mod test_bool;
#[cfg(test)] mod test_int;
#[cfg(test)] mod test_float;
#[cfg(test)] mod test_str;
#[cfg(test)] mod test_enum;
#[cfg(test)] mod test_pos;
#[cfg(test)] mod test_many;
#[cfg(test)] mod test_optional;
#[cfg(test)] mod test_usage;
#[cfg(test)] mod test_help;
#[cfg(test)] mod test_env;
