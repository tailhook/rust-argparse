use parser::ArgumentParser;
use super::{IncrBy,DecrBy};
use super::Store;
use test_parser::{check_ok};

fn incr_int(args: &[&str]) -> usize {
    let mut val = 0;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut val)
          .add_option(&["-i", "--incr"], IncrBy(1usize),
            "Increment value");
        check_ok(&ap, args);
    }
    return val;
}

#[test]
fn test_incr_int() {
    assert_eq!(incr_int(&["./argparse_test"]), 0);
    assert_eq!(incr_int(&["./argparse_test", "--incr"]), 1);
    assert_eq!(incr_int(&["./argparse_test", "-iiiii"]), 5);
}

fn decr_int(args: &[&str]) -> isize {
    let mut val = 5;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut val)
          .add_option(&["-d", "--decr"], DecrBy(1isize),
            "Decrement value");
        check_ok(&ap, args);
    }
    return val;
}

#[test]
fn test_decr_int() {
    assert_eq!(decr_int(&["./argparse_test"]), 5);
    assert_eq!(decr_int(&["./argparse_test", "--decr"]), 4);
    assert_eq!(decr_int(&["./argparse_test", "-dddddd"]), -1);
}

#[test]
fn test_incr_decr() {
    let mut val = 0;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut val)
          .add_option(&["-d", "--decr"], DecrBy(1isize),
            "Decrement value")
          .add_option(&["-i", "--incr"], IncrBy(1isize),
            "Increment value");
        check_ok(&ap, &["./argparse_test",
            "-iiddd", "--incr", "-iii"]);
    }
    assert_eq!(val, 3);
}

fn set_int(args: &[&str]) -> isize {
    let mut val = 0;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut val)
          .add_option(&["-s", "--set"], Store,
            "Set integer value");
        check_ok(&ap, args);
    }
    return val;
}

#[test]
fn test_set_int() {
    assert_eq!(set_int(&["./argparse_test", "-s", "10"]), 10);
    assert_eq!(set_int(&["./argparse_test", "--set", "99"]), 99);
    assert_eq!(set_int(&["./argparse_test", "-s", "7", "-s77"]), 77);
    assert_eq!(set_int(&["./argparse_test", "-s333", "--set=123"]), 123);
}

#[test]
#[should_panic(expected="Bad value 1.5")]
fn test_set_int_bad() {
    set_int(&["./argparse_test", "-s1.5"]);
}

fn set_i16(args: &[&str]) -> i16 {
    let mut val = 0;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut val)
          .add_option(&["-s", "--set"], Store,
            "Set integer value");
        check_ok(&ap, args);
    }
    return val;
}

#[test]
fn test_i16() {
    assert_eq!(set_i16(&["./argparse_test", "-s", "124"]), 124);
}

#[test]
#[should_panic(expected="Bad value 1000000")]
fn test_i16_big() {
    set_i16(&["./argparse_test", "-s", "1000000"]);
}
