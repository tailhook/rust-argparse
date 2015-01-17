use std::str::FromStr;

use parser::ArgumentParser;
use super::Store;
use test_parser::{check_ok,check_err};

use self::Greeting::{Hello, Hi, NoGreeting};


#[derive(PartialEq, Eq, Show)]
enum Greeting {
    Hello,
    Hi,
    NoGreeting,
}

impl FromStr for Greeting {
    fn from_str(src: &str) -> Option<Greeting> {
        return match src {
            "hello" => Some(Hello),
            "hi" => Some(Hi),
            _ => None,
        };
    }
}

fn parse_enum(args: &[&str]) -> Greeting {
    let mut val = NoGreeting;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut val)
          .add_option(&["-g"], box Store::<Greeting>,
            "Greeting");
        check_ok(&ap, args);
    }
    return val;
}

#[test]
fn test_parse_enum() {
    assert_eq!(parse_enum(&["./argparse_test"]), NoGreeting);
    assert_eq!(parse_enum(&["./argparse_test", "-ghello"]), Hello);
    assert_eq!(parse_enum(&["./argparse_test", "-ghi"]), Hi);
}

#[test]
#[should_fail]
fn test_parse_error() {
    parse_enum(&["./argparse_test", "-ghell"]);
}
