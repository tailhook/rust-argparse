#![crate_id = "argparse"]
#![crate_type = "lib"]

extern crate collections;
extern crate arena;

pub use parser::{ArgumentParser, Ref};
pub use bool::{StoreTrue, StoreFalse, StoreBool};
pub use num::{IncrBy, DecrBy};
pub use generic::{Store, StoreOption, StoreConst, List, Collect};

mod generic;
mod parser;
mod action;
mod help;

mod bool;
mod num;

mod test_parser;
mod test_bool;
mod test_int;
mod test_float;
mod test_str;
mod test_enum;
mod test_pos;
mod test_many;
mod test_optional;
mod test_usage;
