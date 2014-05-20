use parser::ArgumentParser;
use generic::{List, Store, Collect};
use test_parser::{check_ok,check_err};

#[test]
fn test_pos_list() {
    let mut ap = ArgumentParser::new();
    let mut val1 = 1;
    let mut val2 = ~[];
    ap.refer(&mut val1).add_argument("v1", box Store::<int>, "The value 1");
    ap.refer(&mut val2).add_argument("v2", box List::<int>, "The list of vals");
    check_ok(&ap, ["./argparse_test", "10"]);
    assert_eq!(val1, 10);
    check_ok(&ap, ["./argparse_test", "11", "21"]);
    assert_eq!(val1, 11);
    assert_eq!(val2, ~[21]);
    check_ok(&ap, ["./argparse_test", "10", "20", "30"]);
    assert_eq!(val1, 10);
    assert_eq!(val2, ~[20, 30]);
}

#[test]
fn test_pos_collect() {
    let mut ap = ArgumentParser::new();
    let mut lst = ~[];
    ap.refer(&mut lst).add_argument("v", box Collect::<int>, "The list of vals");
    check_ok(&ap, ["./argparse_test", "10"]);
    assert_eq!(lst, ~[10]);
    check_ok(&ap, ["./argparse_test", "11", "21"]);
    assert_eq!(lst, ~[11, 21]);
    check_ok(&ap, ["./argparse_test", "10", "20", "30"]);
    assert_eq!(lst, ~[10, 20, 30]);
    check_err(&ap, ["./argparse_test", "10", "20", "test"]);
}

#[test]
fn test_collect() {
    let mut ap = ArgumentParser::new();
    let mut lst = ~[];
    ap.refer(&mut lst).add_option(["-a", "--add"], box Collect::<int>,
        "The list of vals");
    check_ok(&ap, ["./argparse_test", "-a10"]);
    assert_eq!(lst, ~[10]);
    check_ok(&ap, ["./argparse_test", "--add=11", "-a", "21"]);
    assert_eq!(lst, ~[11, 21]);
    check_ok(&ap, ["./argparse_test",
        "-a", "10", "--add=20", "--add", "30"]);
    assert_eq!(lst, ~[10, 20, 30]);
    check_err(&ap, ["./argparse_test",
        "-a", "10", "20", "30"]);
}

#[test]
fn test_list() {
    let mut ap = ArgumentParser::new();
    let mut vec = ~[];
    ap.refer(&mut vec).add_option(["-a", "--add"], box List::<int>,
        "The list of vals");
    check_ok(&ap, ["./argparse_test", "-a10"]);
    assert_eq!(vec.as_slice(), &[10]);
    check_ok(&ap, ["./argparse_test", "--add", "11", "21"]);
    assert_eq!(vec.as_slice(), &[11, 21]);
    check_ok(&ap, ["./argparse_test", "-a", "10", "20", "30"]);
    assert_eq!(vec.as_slice(), &[10, 20, 30]);
    check_err(&ap, ["./argparse_test", "10", "20", "30"]);
}
