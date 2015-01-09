use parser::ArgumentParser;
use super::StoreOption;
use test_parser::{check_ok,check_err};

#[test]
fn test_opt() {
    let mut val = None;
    let mut ap = ArgumentParser::new();
    ap.refer(&mut val)
      .add_option(&["-s", "--set"], box StoreOption::<int>,
        "Set int value");
    assert_eq!(val, None);
    check_ok(&ap, &["./argparse_test"]);
    assert_eq!(val, None);
    check_ok(&ap, &["./argparse_test", "-s", "10"]);
    assert_eq!(val, Some(10i));
    check_ok(&ap, &["./argparse_test", "--set", "11"]);
    assert_eq!(val, Some(11));
    check_err(&ap, &["./argparse_test", "--set"]);
}

#[test]
fn test_str() {
    let mut val = None;
    let mut ap = ArgumentParser::new();
    ap.refer(&mut val)
      .add_option(&["-s", "--set"], box StoreOption::<String>,
        "Set string value");
    assert_eq!(val, None);
    check_ok(&ap, &["./argparse_test"]);
    assert_eq!(val, None);
    check_ok(&ap, &["./argparse_test", "-s", "10"]);
    assert_eq!(val, Some("10".to_string()));
    check_ok(&ap, &["./argparse_test", "--set", "11"]);
    assert_eq!(val, Some("11".to_string()));
    check_err(&ap, &["./argparse_test", "--set"]);
}
