use parser::ArgumentParser;
use super::{Store, List};
use test_parser::{check_ok};

fn parse_pos(args: &[&str]) -> isize {
    let mut val = 0;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut val)
            .add_argument("value", Store, "The value");
        check_ok(&ap, args);
    }
    return val;
}


#[test]
fn test_argument() {
    assert_eq!(parse_pos(&["./argparse_test", "10"]), 10);
}

#[test]
#[should_panic]
fn too_much_args() {
    parse_pos(&["./argparse_test", "10", "20"]);
}

#[test]
#[should_panic]
fn wrong_value() {
    parse_pos(&["./argparse_test", "test", "20"]);
}

#[test]
#[should_panic]
fn float_value() {
    parse_pos(&["./argparse_test", "1.5"]);
}

fn parse_two(args: &[&str]) -> (isize, isize) {
    let mut val1 = 1;
    let mut val2 = 2;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut val1)
            .add_argument("v1", Store, "The value 1");
        ap.refer(&mut val2)
            .add_argument("v2", Store, "The value 2");
        check_ok(&ap, args);
    }
    return (val1, val2);
}

#[test]
fn test_two() {
    assert_eq!(parse_two(&["./argparse_test", "10"]), (10, 2));
    assert_eq!(parse_two(&["./argparse_test", "11", "21"]), (11, 21));
}

#[test]
#[should_panic]
fn test_two_fail_many() {
    parse_two(&["./argparse_test", "10", "20", "30"]);
}

#[test]
#[should_panic]
fn test_two_fail_value() {
    parse_two(&["./argparse_test", "test", "20"]);
}

#[test]
#[should_panic]
fn test_two_fail_float() {
    parse_two(&["./argparse_test", "1.5"]);
}

fn parse_pos_opt(args: &[&str]) -> (isize, isize) {
    let mut val1 = 1;
    let mut val2 = 2;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut val1)
            .add_option(&["--v1"], Store, "The value 1")
            .add_argument("v1", Store, "The value 1");
        ap.refer(&mut val2)
            .add_argument("v2", Store, "The value 2");
        check_ok(&ap, args);
    }
    return (val1, val2);
}

#[test]
fn test_positional_optional() {
    assert_eq!(parse_pos_opt(&["./argparse_test", "10"]), (10, 2));
    assert_eq!(parse_pos_opt(&["./argparse_test", "11", "21"]), (11, 21));
    assert_eq!(parse_pos_opt(&["./argparse_test", "--v1=7", "8"]), (7, 8));
    assert_eq!(parse_pos_opt(&["./argparse_test", "10", "--v1=9"]), (9, 10));
}

#[test]
#[should_panic]
fn test_pos_opt_err() {
    parse_pos_opt(&["./argparse_test", "--v1=10", "20", "30"]);
}

fn parse_pos_req(args: &[&str]) -> (isize, isize) {
    let mut val1 = 1;
    let mut val2 = 2;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut val1)
            .add_option(&["--v1"], Store, "The value 1")
            .add_argument("v1", Store, "The value 1")
            .required();
        ap.refer(&mut val2)
            .add_argument("v2", Store, "The value 2");
        check_ok(&ap, args);
    }
    return (val1, val2);
}

#[test]
fn test_positional_required() {
    assert_eq!(parse_pos_req(&["./argparse_test", "10"]), (10, 2));
    assert_eq!(parse_pos_req(&["./argparse_test", "11", "21"]), (11, 21));
    assert_eq!(parse_pos_req(&["./argparse_test", "--v1=7"]), (7, 2));
    assert_eq!(parse_pos_req(&["./argparse_test", "10", "--v1=9"]), (9, 10));
}

#[test]
#[should_panic]
fn test_pos_extra() {
    parse_pos_req(&["./argparse_test", "--v1=10", "20", "30"]);
}

#[test]
#[should_panic]
fn test_pos_no_req() {
    parse_pos_req(&["./argparse_test"]);
}

fn pos_stop(args: &[&str]) -> (isize, Vec<String>) {
    let mut val1 = 1;
    let mut val2 = Vec::new();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut val1)
            .add_option(&["--v1"], Store, "The value 1")
            .add_argument("v1", Store, "The value 1")
            .required();
        ap.refer(&mut val2)
            .add_argument("v2", List, "The value 2");
        ap.stop_on_first_argument(true);
        check_ok(&ap, args);
    }
    return (val1, val2);
}

#[test]
fn test_pos_stop() {
    assert_eq!(pos_stop(&["./argparse_test", "10"]), (10, vec!()));
    assert_eq!(pos_stop(&["./argparse_test", "11", "21"]),
        (11, vec!("21".to_string())));
    assert_eq!(pos_stop(&["./argparse_test", "--v1=7"]), (7, vec!()));
    assert_eq!(pos_stop(&["./argparse_test", "10", "--v1=9", "--whatever"]),
        (10, vec!("--v1=9".to_string(), "--whatever".to_string())));
}

#[test]
#[should_panic]
fn test_test() {
    pos_stop(&["./argparse_test"]);
}

fn pos_dash(args: &[&str], dash: bool) -> Vec<String> {
    let mut val = Vec::new();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut val)
            .add_argument("v1", List, "The value");
        ap.silence_double_dash(dash);
        check_ok(&ap, args);
    }
    return val;
}

#[test]
fn test_pos_dash() {
    assert_eq!(pos_dash(&["./argparse_test", "1"], true),
        vec!("1".to_string()));
    assert_eq!(pos_dash(&["./argparse_test", "--", "1"], true),
        vec!("1".to_string()));
    assert_eq!(pos_dash(&["./argparse_test", "--", "1"], false),
        vec!("--".to_string(), "1".to_string()));
}
