use std::cell::RefCell;
use parser::ArgumentParser;
use generic::Store;

#[test]
fn test_str() {
    let mut val: ~str = ~"";
    let mut ap = ArgumentParser::new();
    ap.refer(&RefCell::new(&mut val))
      .add_option(~["-s", "--set"],
        "Set string value",
        ~Store::<~str>);
    ap.parse_list(~[~"./argparse_test", ~"-s", ~"10"]);
    assert!(val.eq(&~"10"));
    ap.parse_list(~[~"./argparse_test", ~"--set", ~"value"]);
    assert!(val.eq(&~"value"));
}
