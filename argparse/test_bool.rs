use std::cell::RefCell;
use parser::ArgumentParser;
use bool::{StoreTrue, StoreFalse};

#[test]
fn test_store_true() {
    let mut verbose = false;
    let mut ap = ArgumentParser::new();
    ap.refer(&RefCell::new(&mut verbose))
      .add_option(~["-t", "--true"],
        "Store true action",
        ~StoreTrue);
    assert!(!verbose);
    ap.parse_list(~[~"./argparse_test"]);
    assert!(!verbose);
    ap.parse_list(~[~"./argparse_test", ~"--true"]);
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
    ap.parse_list(~[~"./argparse_test"]);
    assert!(verbose);
    ap.parse_list(~[~"./argparse_test", ~"--false"]);
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
    ap.parse_list(~[~"./argparse_test"]);
    assert!(!verbose);
    ap.parse_list(~[~"./argparse_test", ~"-t"]);
    assert!(verbose);
    ap.parse_list(~[~"./argparse_test", ~"-f"]);
    assert!(!verbose);
    ap.parse_list(~[~"./argparse_test", ~"-fft"]);
    assert!(verbose);
    ap.parse_list(~[~"./argparse_test", ~"-fffft", ~"-f"]);
    assert!(!verbose);
    ap.parse_list(~[~"./argparse_test", ~"--false", ~"-fffft", ~"-f",
                    ~"--true"]);
    assert!(verbose);
}
