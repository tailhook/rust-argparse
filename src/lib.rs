#![crate_name = "argparse"]
#![crate_type = "lib"]

pub use self::parser::{ArgumentParser, Ref};

pub mod action;
pub mod parser;
mod generic;
mod custom;
mod help;
mod print;

mod bool;
mod num;
mod from_cli;

pub trait FromCommandLine: Sized {
    fn from_argument(s: &str) -> Result<Self, String>;
}

// TODO(tailhook) make consts
pub struct StoreTrue;
pub struct StoreFalse;

pub struct StoreConst<T>(pub T);

pub struct PushConst<T>(pub T);

pub struct Store;
pub struct Parse;

pub struct StoreOption;
pub struct ParseOption;

pub struct List;
pub struct ParseList;

pub struct Collect;
pub struct ParseCollect;

/// Print string and exit with status 0
///
/// Particularly useful for `--version` option and similar
pub struct Print(pub String);

pub struct IncrBy<T>(pub T);

pub struct DecrBy<T>(pub T);


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
#[cfg(test)] mod test_const;
#[cfg(test)] mod test_path;
