use parser::ArgumentParser;
use super::Store;
use super::{StoreTrue, StoreFalse};
use test_parser::{check_ok};

fn store_true(args: &[&str]) -> bool {
    let mut verbose = false;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut verbose)
          .add_option(&["-t", "--true"], StoreTrue,
            "Store true action");
        check_ok(&ap,  args);
    }
    return verbose;
}

#[test]
fn test_store_true() {
    assert!(!store_true(&["./argparse_test"]));
    assert!(store_true(&["./argparse_test", "--true"]));
}

fn store_false(args: &[&str]) -> bool {
    let mut verbose = true;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut verbose)
          .add_option(&["-f", "--false"], StoreFalse,
            "Store false action");
        check_ok(&ap,  args);
    }
    return verbose;
}
#[test]
fn test_store_false() {
    assert!(store_false(&["./argparse_test"]));
    assert!(!store_false(&["./argparse_test", "--false"]));
}

fn store_bool(args: &[&str]) -> bool {
    let mut verbose = false;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut verbose)
          .add_option(&["-f", "--false"], StoreFalse,
            "Store false action")
          .add_option(&["-t", "--true"], StoreTrue,
            "Store false action");
        check_ok(&ap,  args);
    }
    return verbose;
}

#[test]
fn test_bool() {
    assert!(!store_bool(&["./argparse_test"]));
    assert!(store_bool(&["./argparse_test", "-t"]));
    assert!(!store_bool(&["./argparse_test", "-f"]));
    assert!(store_bool(&["./argparse_test", "-fft"]));
    assert!(!store_bool(&["./argparse_test", "-fffft", "-f"]));
    assert!(store_bool(&["./argparse_test", "--false", "-fffft", "-f",
                   "--true"]));
}

fn set_bool(args: &[&str]) -> bool {
    let mut verbose = false;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut verbose)
          .add_option(&["-s", "--set"], Store,
            "Set boolean value");
        check_ok(&ap,  args);
    }
    return verbose;
}


#[test]
fn test_set_bool() {
    assert!(!set_bool(&["./argparse_test"]));
    assert!(set_bool(&["./argparse_test", "-strue"]));
    assert!(!set_bool(&["./argparse_test", "-sfalse"]));

    // Unfortunately other values do not work
}

#[test]
#[should_panic(expected="Bad value yes")]
fn test_bad_bools1() {
    assert!(!set_bool(&["./argparse_test", "-syes"]));
}

#[test]
#[should_panic(expected="Bad value no")]
fn test_bad_bools2() {
    assert!(!set_bool(&["./argparse_test", "-sno"]));
}
