use parser::ArgumentParser;

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

