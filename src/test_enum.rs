use std::str::FromStr;

use parser::ArgumentParser;
use super::Store;
use test_parser::{check_ok};

use self::Greeting::{Hello, Hi, NoGreeting};


#[derive(PartialEq, Eq, Debug)]
enum Greeting {
    Hello,
    Hi,
    NoGreeting,
}

impl FromStr for Greeting {
    type Err = ();
    fn from_str(src: &str) -> Result<Greeting, ()> {
        return match src {
            "hello" => Ok(Hello),
            "hi" => Ok(Hi),
            _ => Err(()),
        };
    }
}

fn parse_enum(args: &[&str]) -> Greeting {
    let mut val = NoGreeting;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut val)
          .add_option(&["-g"], Store,
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
#[should_panic]
fn test_parse_error() {
    parse_enum(&["./argparse_test", "-ghell"]);
}
