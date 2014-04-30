use parser::ArgumentParser;
use generic::Store;
use test_parser::{check_ok,check_err};

#[test]
fn test_str() {
    let mut val: ~str = ~"";
    let mut ap = ArgumentParser::new();
    ap.refer(&mut val)
      .add_option(~["-s", "--set"],
        "Set string value",
        ~Store::<~str>);
    check_ok(ap.parse_list(~[~"./argparse_test", ~"-s", ~"10"]));
    assert!(val.eq(&~"10"));
    check_ok(ap.parse_list(~[~"./argparse_test", ~"--set", ~"value"]));
    assert!(val.eq(&~"value"));
    check_err(ap.parse_list(~[~"./argparse_test", ~"--set"]));
}
