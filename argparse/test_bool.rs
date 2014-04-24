use parser::{ArgumentParser, cell, StoreTrue, StoreFalse};

#[test]
fn test_store_true() {
    let mut verbose = false;
    let mut ap = ArgumentParser::new();
    ap.add_option(~["-t", "--true"],
        "Store true action",
        StoreTrue(cell(&mut verbose)));
    assert!(!verbose);
    ap.parse_args(~[~"./argparse_test"]);
    assert!(!verbose);
    ap.parse_args(~[~"./argparse_test", ~"--true"]);
    assert!(verbose);
}

#[test]
fn test_store_false() {
    let mut verbose = true;
    let mut ap = ArgumentParser::new();
    ap.add_option(~["-f", "--false"],
        "Store false action",
        StoreFalse(cell(&mut verbose)));
    assert!(verbose);
    ap.parse_args(~[~"./argparse_test"]);
    assert!(verbose);
    ap.parse_args(~[~"./argparse_test", ~"--false"]);
    assert!(!verbose);
}

#[test]
fn test_bool() {
    let mut verbose = false;
    let mut ap = ArgumentParser::new();
    let c = cell(&mut verbose);
    ap.add_option(~["-f", "--false"],
        "Store false action",
        StoreFalse(c.clone()));
    ap.add_option(~["-t", "--true"],
        "Store false action",
        StoreTrue(c.clone()));
    assert!(!verbose);
    ap.parse_args(~[~"./argparse_test"]);
    assert!(!verbose);
    ap.parse_args(~[~"./argparse_test", ~"-t"]);
    assert!(verbose);
    ap.parse_args(~[~"./argparse_test", ~"-f"]);
    assert!(!verbose);
    ap.parse_args(~[~"./argparse_test", ~"-fft"]);
    assert!(verbose);
    ap.parse_args(~[~"./argparse_test", ~"-fffft", ~"-f"]);
    assert!(!verbose);
    ap.parse_args(~[~"./argparse_test", ~"--false", ~"-fffft", ~"-f",
                    ~"--true"]);
    assert!(verbose);
}
