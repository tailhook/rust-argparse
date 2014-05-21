use std::os;

use parser::ArgumentParser;
use super::Store;
use test_parser::{check_ok, check_err};


#[test]
fn test_required() {
    let mut ap = ArgumentParser::new();
    let mut val1 = 1;
    let mut val2 = 2;
    ap.refer(&mut val1)
        .add_option(["--v1"], box Store::<int>, "The value 1")
        .add_argument("v1", box Store::<int>, "The value 1")
        .envvar("TEST_ENV_REQUIRED_V1")
        .required();
    ap.refer(&mut val2).add_argument("v2", box Store::<int>, "The value 2");
    os::setenv("TEST_ENV_REQUIRED_V1", "some_crap");
    check_err(&ap, ["./argparse_test"]);
    check_ok(&ap, ["./argparse_test", "10"]);
    assert_eq!(val1, 10);
    check_ok(&ap, ["./argparse_test", "11", "21"]);
    assert_eq!(val1, 11);
    assert_eq!(val2, 21);
    check_ok(&ap, ["./argparse_test", "--v1=7"]);
    assert_eq!(val1, 7);
    os::setenv("TEST_ENV_REQUIRED_V1", "9");
    check_ok(&ap, ["./argparse_test", "10"]);
    assert_eq!(val1, 9);
    assert_eq!(val2, 10);
    check_ok(&ap, ["./argparse_test", "7", "--v1=15"]);
    assert_eq!(val1, 15);
    assert_eq!(val2, 7);
    os::unsetenv("TEST_ENV_REQUIRED_V1");
}
