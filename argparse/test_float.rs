use parser::ArgumentParser;
use super::{IncrBy,DecrBy};
use super::Store;
use test_parser::{check_ok,check_err};

#[test]
fn test_incr_decr() {
    let mut val = 0.5;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut val)
          .add_option(["-d", "--decr"], box DecrBy(0.25),
            "Decrement value")
          .add_option(["-i", "--incr"], box IncrBy(0.5),
            "Increment value");
        assert!(val == 0.5);
        check_ok(&ap, ["./argparse_test",
            "-iiddd", "--incr", "-iii"]);
    }
    assert_eq!(val, 2.75);
}

#[test]
fn test_float() {
    let mut val = 0.1;
    let mut ap = ArgumentParser::new();
    ap.refer(&mut val)
      .add_option(["-s", "--set"], box Store::<f64>,
        "Set integer value");
    check_ok(&ap, ["./argparse_test", "-s", "15.125"]);
    assert_eq!(val, 15.125);
    check_err(&ap, ["./argparse_test", "-s", "test"]);
}
