use std::cell::RefCell;

use parser::ArgumentParser;
use num::{IncrBy,DecrBy};
use generic::Store;
use test_parser::{check_ok,check_err};

#[test]
fn incr_int() {
    let mut val = 0;
    let mut ap = ArgumentParser::new();
    ap.refer(&RefCell::new(&mut val))
      .add_option(~["-i", "--incr"],
        "Increment value",
        ~IncrBy(1));
    assert!(val == 0);
    check_ok(ap.parse_list(~[~"./argparse_test"]));
    assert!(val == 0);
    check_ok(ap.parse_list(~[~"./argparse_test", ~"--incr"]));
    assert!(val == 1);
    check_ok(ap.parse_list(~[~"./argparse_test", ~"-iiiii"]));
    assert!(val == 6);
}

#[test]
fn test_decr_int() {
    let mut val = 5;
    let mut ap = ArgumentParser::new();
    ap.refer(&RefCell::new(&mut val))
      .add_option(~["-d", "--decr"],
        "Decrement value",
        ~DecrBy(1));
    assert!(val == 5);
    check_ok(ap.parse_list(~[~"./argparse_test"]));
    assert!(val == 5);
    check_ok(ap.parse_list(~[~"./argparse_test", ~"--decr"]));
    assert!(val == 4);
    check_ok(ap.parse_list(~[~"./argparse_test", ~"-ddddd"]));
    assert!(val == -1);
}

#[test]
fn test_incr_decr() {
    let mut val = 0;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&RefCell::new(&mut val))
          .add_option(~["-d", "--decr"],
            "Decrement value",
            ~DecrBy(1))
          .add_option(~["-i", "--incr"],
            "Increment value",
            ~IncrBy(1));
        assert!(val == 0);
        check_ok(ap.parse_list(~[~"./argparse_test",
            ~"-iiddd", ~"--incr", ~"-iii"]));
    }
    assert_eq!(val, 3);
}

#[test]
fn test_int() {
    let mut val = 0;
    let mut ap = ArgumentParser::new();
    ap.refer(&RefCell::new(&mut val))
      .add_option(~["-s", "--set"],
        "Set integer value",
        ~Store::<int>);
    check_ok(ap.parse_list(~[~"./argparse_test", ~"-s", ~"10"]));
    assert!(val == 10);
    check_ok(ap.parse_list(~[~"./argparse_test", ~"--set", ~"99"]));
    assert!(val == 99);
    check_ok(ap.parse_list(~[~"./argparse_test", ~"-s", ~"7", ~"-s77"]));
    assert!(val == 77);
    check_ok(ap.parse_list(~[~"./argparse_test", ~"-s333", ~"--set=123"]));
    assert!(val == 123);
    check_err(ap.parse_list(~[~"./argparse_test", ~"-s1.5"]));
}

#[test]
fn test_i16() {
    let mut val = 0;
    let mut ap = ArgumentParser::new();
    ap.refer(&RefCell::new(&mut val))
      .add_option(~["-s", "--set"],
        "Set integer value",
        ~Store::<i16>);
    check_ok(ap.parse_list(~[~"./argparse_test", ~"-s", ~"124"]));
    assert_eq!(val, 124);
    check_err(ap.parse_list(~[~"./argparse_test", ~"-s", ~"1000000"]));
}
