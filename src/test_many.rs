use parser::ArgumentParser;
use super::{List, Store, Collect};
use test_parser::{check_ok};

fn pos_list(args: &[&str]) -> (isize, Vec<isize>) {
    let mut val1 = 1;
    let mut val2 = Vec::new();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut val1).add_argument("v1", Store, "The value 1");
        ap.refer(&mut val2).add_argument("v2", List, "The list of vals");
        check_ok(&ap, args);
    }
    return (val1, val2);
}

#[test]
fn test_pos_list() {
    assert_eq!(pos_list(&["./argparse_test", "10"]), (10, vec!()));
    assert_eq!(pos_list(&["./argparse_test", "11", "21"]), (11, vec!(21)));
    assert_eq!(pos_list(&["./argparse_test", "10", "20", "30"]),
        (10, vec!(20, 30)));
}

fn pos_collect(args: &[&str]) -> Vec<isize> {
    let mut lst = Vec::new();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut lst)
            .add_argument("v", Collect, "The list of vals");
        check_ok(&ap, args);
    }
    return lst;
}

#[test]
fn test_pos_collect() {
    assert_eq!(pos_collect(&["./argparse_test", "10"]), vec!(10));
    assert_eq!(pos_collect(&["./argparse_test", "11", "21"]), vec!(11, 21));
    assert_eq!(pos_collect(&["./argparse_test", "10", "20", "30"]),
        vec!(10, 20, 30));
}

#[test]
#[should_panic]
fn wrong_type() {
    pos_collect(&["./argparse_test", "10", "20", "test"]);
}

fn collect(args: &[&str]) -> Vec<isize> {
    let mut lst = Vec::new();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut lst).add_option(&["-a", "--add"], Collect,
            "The list of vals");
        check_ok(&ap, args);
    }
    return lst;
}

#[test]
fn test_collect() {
    assert_eq!(collect(&["./argparse_test", "-a10"]), vec!(10));
    assert_eq!(collect(&["./argparse_test", "--add=11", "-a", "21"]),
        vec!(11, 21));
    assert_eq!(collect(&["./argparse_test",
        "-a", "10", "--add=20", "--add", "30"]), vec!(10, 20, 30));
}

#[test]
#[should_panic]
fn test_extra() {
    collect(&["./argparse_test", "-a", "10", "20", "30"]);
}

fn list(args: &[&str]) -> Vec<isize> {
    let mut vec = Vec::new();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut vec).add_option(&["-a", "--add"], List,
            "The list of vals");
        check_ok(&ap, args);
    }
    return vec;
}

#[test]
#[should_panic]
fn test_list() {
    assert_eq!(list(&["./argparse_test", "-a10"]), vec!(10));
    assert_eq!(list(&["./argparse_test", "--add", "11", "21"]), vec!(11, 21));
    assert_eq!(list(&["./argparse_test", "-a", "10", "20", "30"]),
        vec!(10, 20, 30));
    assert_eq!(list(&["./argparse_test", "10", "20", "30"]), vec!(10, 20, 30));
}
