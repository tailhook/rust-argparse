#![crate_name = "argparse"]
#![crate_type = "lib"]
#![feature(box_syntax)]

extern crate collections;
extern crate arena;

pub use self::parser::{ArgumentParser, Ref};

pub mod action;
pub mod parser;
mod generic;
mod help;

mod bool;
mod num;

pub struct StoreTrue;

pub struct StoreFalse;

pub struct StoreConst<T>(pub T);

pub struct Store<T>;

pub struct StoreOption<T>;

pub struct List<T>;

pub struct Collect<T>;

pub struct IncrBy<T>(pub T);

pub struct DecrBy<T>(pub T);

pub type StoreBool = Store<bool>;


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
