use parser::ArgumentParser;
use super::Store;
use super::{StoreTrue, StoreFalse};
use test_parser::{check_ok,check_err};

#[test]
fn test_store_true() {
    let mut verbose = false;
    let mut ap = ArgumentParser::new();
    ap.refer(&mut verbose)
      .add_option(["-t", "--true"], ~StoreTrue,
        "Store true action");
    assert!(!verbose);
    check_ok(&ap, ["./argparse_test"]);
    assert!(!verbose);
    check_ok(&ap, ["./argparse_test", "--true"]);
    assert!(verbose);
}

#[test]
fn test_store_false() {
    let mut verbose = true;
    let mut ap = ArgumentParser::new();
    ap.refer(&mut verbose)
      .add_option(["-f", "--false"], ~StoreFalse,
        "Store false action");
    assert!(verbose);
    check_ok(&ap, ["./argparse_test"]);
    assert!(verbose);
    check_ok(&ap, ["./argparse_test", "--false"]);
    assert!(!verbose);
}

#[test]
fn test_bool() {
    let mut verbose = false;
    let mut ap = ArgumentParser::new();
    ap.refer(&mut verbose)
      .add_option(["-f", "--false"], ~StoreFalse,
        "Store false action")
      .add_option(["-t", "--true"], ~StoreTrue,
        "Store false action");
    assert!(!verbose);
    check_ok(&ap, ["./argparse_test"]);
    assert!(!verbose);
    check_ok(&ap, ["./argparse_test", "-t"]);
    assert!(verbose);
    check_ok(&ap, ["./argparse_test", "-f"]);
    assert!(!verbose);
    check_ok(&ap, ["./argparse_test", "-fft"]);
    assert!(verbose);
    check_ok(&ap, ["./argparse_test", "-fffft", "-f"]);
    assert!(!verbose);
    check_ok(&ap, ["./argparse_test", "--false", "-fffft", "-f",
                   "--true"]);
    assert!(verbose);
}

#[test]
fn test_set_bool() {
    let mut verbose = false;
    let mut ap = ArgumentParser::new();
    ap.refer(&mut verbose)
      .add_option(["-s", "--set"], ~Store::<bool>,
        "Set boolean value");
    assert!(!verbose);
    check_ok(&ap, ["./argparse_test"]);
    assert!(!verbose);
    check_ok(&ap, ["./argparse_test", "-strue"]);
    assert!(verbose);
    check_ok(&ap, ["./argparse_test", "-sfalse"]);
    assert!(!verbose);

    // Unfortunately other values do not work
    check_err(&ap, ["./argparse_test", "-syes"]);
    assert!(!verbose);
    check_err(&ap, ["./argparse_test", "-sno"]);
    assert!(!verbose);
}
