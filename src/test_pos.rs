use parser::ArgumentParser;
use super::{Store, List};
use test_parser::{check_ok,check_err};

#[test]
fn test_argument() {
    let mut ap = ArgumentParser::new();
    let mut val = 0;
    ap.refer(&mut val).add_argument("value", box Store::<int>, "The value");
    check_ok(&ap, &["./argparse_test", "10"]);
    assert_eq!(val, 10);
    check_err(&ap, &["./argparse_test", "10", "20"]);
    check_err(&ap, &["./argparse_test", "test", "20"]);
    check_err(&ap, &["./argparse_test", "1.5"]);
}

#[test]
fn test_two() {
    let mut ap = ArgumentParser::new();
    let mut val1 = 1;
    let mut val2 = 2;
    ap.refer(&mut val1).add_argument("v1", box Store::<int>, "The value 1");
    ap.refer(&mut val2).add_argument("v2", box Store::<int>, "The value 2");
    check_ok(&ap, &["./argparse_test", "10"]);
    assert_eq!(val1, 10);
    check_ok(&ap, &["./argparse_test", "11", "21"]);
    assert_eq!(val1, 11);
    assert_eq!(val2, 21);
    check_err(&ap, &["./argparse_test", "10", "20", "30"]);
    check_err(&ap, &["./argparse_test", "test", "20"]);
    check_err(&ap, &["./argparse_test", "1.5"]);
}

#[test]
fn test_positional_optional() {
    let mut ap = ArgumentParser::new();
    let mut val1 = 1;
    let mut val2 = 2;
    ap.refer(&mut val1)
        .add_option(&["--v1"], box Store::<int>, "The value 1")
        .add_argument("v1", box Store::<int>, "The value 1");
    ap.refer(&mut val2).add_argument("v2", box Store::<int>, "The value 2");
    check_ok(&ap, &["./argparse_test", "10"]);
    assert_eq!(val1, 10);
    check_ok(&ap, &["./argparse_test", "11", "21"]);
    assert_eq!(val1, 11);
    assert_eq!(val2, 21);
    check_ok(&ap, &["./argparse_test", "--v1=7", "8"]);
    assert_eq!(val1, 7);
    assert_eq!(val2, 8);
    check_ok(&ap, &["./argparse_test", "10", "--v1=9"]);
    assert_eq!(val1, 9);
    assert_eq!(val2, 10);
    check_err(&ap, &["./argparse_test", "--v1=10", "20", "30"]);
}

#[test]
fn test_positional_required() {
    let mut ap = ArgumentParser::new();
    let mut val1 = 1;
    let mut val2 = 2;
    ap.refer(&mut val1)
        .add_option(&["--v1"], box Store::<int>, "The value 1")
        .add_argument("v1", box Store::<int>, "The value 1")
        .required();
    ap.refer(&mut val2).add_argument("v2", box Store::<int>, "The value 2");
    check_ok(&ap, &["./argparse_test", "10"]);
    assert_eq!(val1, 10);
    check_ok(&ap, &["./argparse_test", "11", "21"]);
    assert_eq!(val1, 11);
    assert_eq!(val2, 21);
    check_ok(&ap, &["./argparse_test", "--v1=7"]);
    assert_eq!(val1, 7);
    check_ok(&ap, &["./argparse_test", "10", "--v1=9"]);
    assert_eq!(val1, 9);
    assert_eq!(val2, 10);
    check_err(&ap, &["./argparse_test", "--v1=10", "20", "30"]);
    check_err(&ap, &["./argparse_test"]);
}

#[test]
fn test_positional_stop() {
    let mut ap = ArgumentParser::new();
    let mut val1 = 1;
    let mut val2 = Vec::new();
    ap.refer(&mut val1)
        .add_option(&["--v1"], box Store::<int>, "The value 1")
        .add_argument("v1", box Store::<int>, "The value 1")
        .required();
    ap.refer(&mut val2).add_argument("v2", box List::<String>, "The value 2");
    ap.stop_on_first_argument(true);
    check_ok(&ap, &["./argparse_test", "10"]);
    assert_eq!(val1, 10);
    check_ok(&ap, &["./argparse_test", "11", "21"]);
    assert_eq!(val1, 11);
    assert_eq!(val2, vec!("21".to_string()));
    check_ok(&ap, &["./argparse_test", "--v1=7"]);
    assert_eq!(val1, 7);
    check_ok(&ap, &["./argparse_test", "10", "--v1=9", "--whatever"]);
    assert_eq!(val1, 10);
    assert_eq!(val2, vec!("--v1=9".to_string(), "--whatever".to_string()));
    check_err(&ap, &["./argparse_test"]);
}
