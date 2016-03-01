
use parser::ArgumentParser;

pub fn check_ok(ap: &ArgumentParser, args: &[&str]) {
    let mut stdout = Vec::<u8>::new();
    let mut stderr = Vec::<u8>::new();
    let mut owned_args = Vec::new();
    for x in args.iter() {
        owned_args.push(x.to_string());
    }
    let res = ap.parse(owned_args, &mut stdout, &mut stderr);
    match res {
        Ok(()) => return,
        Err(x) => panic!(
            String::from_utf8(stderr).unwrap() +
            &format!("Expected ok, but found Exit({})", x)[..]),
    }
}

pub fn check_exit(ap: &ArgumentParser, args: &[&str]) {
    let mut stdout = Vec::<u8>::new();
    let mut stderr = Vec::<u8>::new();
    let mut owned_args = Vec::new();
    for x in args.iter() {
        owned_args.push(x.to_string());
    }
    let res = ap.parse(owned_args, &mut stdout, &mut stderr);
    match res {
        Err(0) => return,
        Err(x) => panic!(format!("Expected code {} got {}", 0usize, x)),
        Ok(()) => panic!(format!("Expected failure, got success")),
    }
}

pub fn check_err(ap: &ArgumentParser, args: &[&str]) {
    let mut stdout = Vec::<u8>::new();
    let mut stderr = Vec::<u8>::new();
    let mut owned_args = Vec::new();
    for x in args.iter() {
        owned_args.push(x.to_string());
    }
    let res = ap.parse(owned_args, &mut stdout, &mut stderr);
    match res {
        Err(2) => return,
        Err(x) => panic!(format!("Expected code {} got {}", 2usize, x)),
        Ok(()) => panic!(format!("Expected failure, got success")),
    }
}

#[test]
fn test_no_arg() {
    let ap = ArgumentParser::new();
    check_ok(&ap, &["./argparse_test"]);
    check_err(&ap, &["./argparse_test", "a"]);
    check_err(&ap, &["./argparse_test", "-a"]);
    check_err(&ap, &["./argparse_test", "--an-option"]);
}

#[test]
fn test_help() {
    let ap = ArgumentParser::new();
    check_ok(&ap, &["./argparse_test"]);
    check_exit(&ap, &["./argparse_test", "--help"]);
}
