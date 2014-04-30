use std::cell::RefCell;

use parser::ArgumentParser;
use generic::Store;
use bool::{StoreTrue, StoreFalse};
use test_parser::{check_ok,check_err};

#[test]
fn test_store_true() {
    let mut verbose = false;
    let mut ap = ArgumentParser::new();
    ap.refer(&RefCell::new(&mut verbose))
      .add_option(~["-t", "--true"],
        "Store true action",
        ~StoreTrue);
    assert!(!verbose);
    check_ok(ap.parse_list(~[~"./argparse_test"]));
    assert!(!verbose);
    check_ok(ap.parse_list(~[~"./argparse_test", ~"--true"]));
    assert!(verbose);
}

#[test]
fn test_store_false() {
    let mut verbose = true;
    let mut ap = ArgumentParser::new();
    ap.refer(&RefCell::new(&mut verbose))
      .add_option(~["-f", "--false"],
        "Store false action",
        ~StoreFalse);
    assert!(verbose);
    check_ok(ap.parse_list(~[~"./argparse_test"]));
    assert!(verbose);
    check_ok(ap.parse_list(~[~"./argparse_test", ~"--false"]));
    assert!(!verbose);
}

#[test]
fn test_bool() {
    let mut verbose = false;
    let mut ap = ArgumentParser::new();
    ap.refer(&RefCell::new(&mut verbose))
      .add_option(~["-f", "--false"],
        "Store false action",
        ~StoreFalse)
      .add_option(~["-t", "--true"],
        "Store false action",
        ~StoreTrue);
    assert!(!verbose);
    check_ok(ap.parse_list(~[~"./argparse_test"]));
    assert!(!verbose);
    check_ok(ap.parse_list(~[~"./argparse_test", ~"-t"]));
    assert!(verbose);
    check_ok(ap.parse_list(~[~"./argparse_test", ~"-f"]));
    assert!(!verbose);
    check_ok(ap.parse_list(~[~"./argparse_test", ~"-fft"]));
    assert!(verbose);
    check_ok(ap.parse_list(~[~"./argparse_test", ~"-fffft", ~"-f"]));
    assert!(!verbose);
    check_ok(ap.parse_list(~[~"./argparse_test", ~"--false", ~"-fffft", ~"-f",
                    ~"--true"]));
    assert!(verbose);
}

#[test]
fn test_set_bool() {
    let mut verbose = false;
    let mut ap = ArgumentParser::new();
    ap.refer(&RefCell::new(&mut verbose))
      .add_option(~["-s", "--set"],
        "Set boolean value",
        ~Store::<bool>);
    assert!(!verbose);
    check_ok(ap.parse_list(~[~"./argparse_test"]));
    assert!(!verbose);
    check_ok(ap.parse_list(~[~"./argparse_test", ~"-strue"]));
    assert!(verbose);
    check_ok(ap.parse_list(~[~"./argparse_test", ~"-sfalse"]));
    assert!(!verbose);

    // Unfortunately other values do not work
    check_err(ap.parse_list(~[~"./argparse_test", ~"-syes"]));
    assert!(!verbose);
    check_err(ap.parse_list(~[~"./argparse_test", ~"-sno"]));
    assert!(!verbose);
}
