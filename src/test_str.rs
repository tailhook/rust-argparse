use parser::ArgumentParser;
use super::Store;
use test_parser::{check_ok};

fn parse_str(args: &[&str]) -> String {
    let mut val: String = "".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut val)
          .add_option(&["-s", "--set"], Store,
            "Set string value");
        check_ok(&ap, args);
    }
    return val;
}

#[test]
fn test_str() {
    assert_eq!(parse_str(&["./argparse_test", "-s", "10"]), "10".to_string());
    assert_eq!(parse_str(&["./argparse_test", "--set", "value"]),
               "value".to_string());
}

#[test]
#[should_panic]
fn test_err() {
    parse_str(&["./argparse_test", "--set"]);
}
