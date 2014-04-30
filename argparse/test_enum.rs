use std::cell::RefCell;
use std::from_str::FromStr;

use parser::ArgumentParser;
use generic::Store;

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

#[test]
fn test_parse_enum() {
    let mut val = NoGreeting;
    let mut ap = ArgumentParser::new();
    ap.refer(&RefCell::new(&mut val))
      .add_option(~["-g"],
        "Greeting",
        ~Store::<Greeting>);
    ap.parse_list(~[~"./argparse_test"]);
    assert!(match val { NoGreeting => true, _ => false });
    ap.parse_list(~[~"./argparse_test", ~"-ghello"]);
    assert!(match val { Hello => true, _ => false });
    ap.parse_list(~[~"./argparse_test", ~"-ghi"]);
    assert!(match val { Hi => true, _ => false });
}
