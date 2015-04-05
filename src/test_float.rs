use parser::ArgumentParser;
use super::{IncrBy,DecrBy};
use super::Store;
use test_parser::{check_ok};

fn incr_decr(args: &[&str]) -> f32 {
    let mut val = 0f32;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut val)
          .add_option(&["-d", "--decr"], DecrBy(0.25f32),
            "Decrement value")
          .add_option(&["-i", "--incr"], IncrBy(0.5f32),
            "Increment value");
        check_ok(&ap, args);
    }
    return val;
}

#[test]
fn test_incr_decr() {
    assert_eq!(incr_decr(&["./argparse_test",
        "--incr", "-iii"]), 2.0);
    assert_eq!(incr_decr(&["./argparse_test",
        "-iiddd", "--incr", "-iii"]), 2.25);
}

#[test]
fn test_float() {
    let mut val = 0.1;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut val)
          .add_option(&["-s", "--set"], Store,
            "Set float value");
        check_ok(&ap, &["./argparse_test", "-s", "15.125"]);
    }
    assert_eq!(val, 15.125);
}

#[test]
#[should_panic]
fn test_fail() {
    let mut val = 0.1;
    let mut ap = ArgumentParser::new();
    ap.refer(&mut val)
      .add_option(&["-s", "--set"], Store,
        "Set float value");
    check_ok(&ap, &["./argparse_test", "-s", "test"]);
}
