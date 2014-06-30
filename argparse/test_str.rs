use parser::ArgumentParser;
use super::Store;
use test_parser::{check_ok,check_err};

#[test]
fn test_str() {
    let mut val: String = "".to_string();
    let mut ap = ArgumentParser::new();
    ap.refer(&mut val)
      .add_option(["-s", "--set"], box Store::<String>,
        "Set string value");
    check_ok(&ap, ["./argparse_test", "-s", "10"]);
    assert!(val == "10".to_string());
    check_ok(&ap, ["./argparse_test", "--set", "value"]);
    assert!(val == "value".to_string());
    check_err(&ap, ["./argparse_test", "--set"]);
}
