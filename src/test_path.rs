use std::path::PathBuf;
use parser::ArgumentParser;
use super::Parse;
use test_parser::{check_ok};

fn parse_str(args: &[&str]) -> PathBuf {
    let mut val: PathBuf = From::from("");
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut val)
          .add_option(&["-s", "--set"], Parse,
            "Set path value");
        check_ok(&ap, args);
    }
    return val;
}

#[test]
fn test_path() {
    assert_eq!(parse_str(&["./argparse_test", "-s", "/hello"]),
               PathBuf::from("/hello"));
    assert_eq!(parse_str(&["./argparse_test", "--set", "a///b/../c"]),
               PathBuf::from("a/b/../c"));
}

#[test]
#[should_panic]
fn test_err() {
    parse_str(&["./argparse_test", "--set"]);
}
