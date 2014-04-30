use parser::ArgumentParser;
use num::{IncrBy,DecrBy};
use generic::Store;
use test_parser::{check_ok,check_err};

#[test]
fn test_incr_decr() {
    let mut val = 0.5;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut val)
          .add_option(~["-d", "--decr"],
            "Decrement value",
            ~DecrBy(0.25))
          .add_option(~["-i", "--incr"],
            "Increment value",
            ~IncrBy(0.5));
        assert!(val == 0.5);
        check_ok(ap.parse_list(~[~"./argparse_test",
            ~"-iiddd", ~"--incr", ~"-iii"]));
    }
    assert_eq!(val, 2.75);
}

#[test]
fn test_float() {
    let mut val = 0.1;
    let mut ap = ArgumentParser::new();
    ap.refer(&mut val)
      .add_option(~["-s", "--set"],
        "Set integer value",
        ~Store::<f64>);
    check_ok(ap.parse_list(~[~"./argparse_test", ~"-s", ~"15.125"]));
    assert_eq!(val, 15.125);
    check_err(ap.parse_list(~[~"./argparse_test", ~"-s", ~"test"]));
}
