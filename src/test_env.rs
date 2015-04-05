use std::env;

use parser::ArgumentParser;
use super::Store;
use test_parser::{check_ok};


fn required(args: &[&str]) -> (isize, isize) {
    let mut val1 = 1isize;
    let mut val2 = 2isize;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut val1)
            .add_option(&["--v1"], Store, "The value 1")
            .add_argument("v1", Store, "The value 1")
            .envvar("TEST_ENV_REQUIRED_V1")
            .required();
        ap.refer(&mut val2)
            .add_argument("v2", Store, "The value 2");
        check_ok(&ap, args);
    }
    return (val1, val2)
}

#[test]
#[should_panic]
fn test_required() {
    env::set_var("TEST_ENV_REQUIRED_V1", "some_crap");
    required(&["./argparse_test"]);
    env::remove_var("TEST_ENV_REQUIRED_V1");
}

#[test]
fn test_req() {
    env::set_var("TEST_ENV_REQUIRED_V1", "some_crap");
    assert_eq!(required(&["./argparse_test", "10"]), (10, 2));
    assert_eq!(required(&["./argparse_test", "11", "21"]), (11, 21));
    assert_eq!(required(&["./argparse_test", "--v1=7"]), (7, 2));
    env::set_var("TEST_ENV_REQUIRED_V1", "9");
    assert_eq!(required(&["./argparse_test", "10"]), (9, 10));
    assert_eq!(required(&["./argparse_test", "7", "--v1=15"]), (15, 7));
    env::remove_var("TEST_ENV_REQUIRED_V1");
}
