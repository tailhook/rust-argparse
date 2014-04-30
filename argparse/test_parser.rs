use parser::ArgumentParser;
use generic::Store;

#[cfg(test)]
pub fn check_ok(res: Result<(), int>) {
    match res {
        Ok(()) => return,
        Err(x) => fail!(format!("Expected ok, but found Exit({})", x)),
    }
}

#[cfg(test)]
pub fn check_exit(res: Result<(), int>) {
    match res {
        Err(0) => return,
        Err(x) => fail!(format!("Expected code {} got {}", 0, x)),
        Ok(()) => fail!(format!("Expected failure, got success")),
    }
}

#[cfg(test)]
pub fn check_err(res: Result<(), int>) {
    match res {
        Err(2) => return,
        Err(x) => fail!(format!("Expected code {} got {}", 2, x)),
        Ok(()) => fail!(format!("Expected failure, got success")),
    }
}

#[test]
fn test_no_arg() {
    let ap = ArgumentParser::new();
    assert!(match ap.parse_list(~[~"./argparse_test"]) {
        Ok(()) => true, _ => false });
}

#[test]
fn test_argument() {
    let mut ap = ArgumentParser::new();
    let mut val = 0;
    ap.refer(&mut val).add_argument("value", "The value", ~Store::<int>);
    check_ok(ap.parse_list(~[~"./argparse_test", ~"10"]));
    assert_eq!(val, 10);
    check_err(ap.parse_list(~[~"./argparse_test", ~"10", ~"20"]));
    check_err(ap.parse_list(~[~"./argparse_test", ~"test", ~"20"]));
    check_err(ap.parse_list(~[~"./argparse_test", ~"1.5"]));
}
