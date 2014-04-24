#![crate_id = "argparse"]
#![crate_type = "lib"]

extern crate collections;

pub use parser::{ArgumentParser, cell, StoreTrue, StoreFalse};

mod parser;

mod test_bool;
mod test_parser;
