use std::io::MemWriter;
use std::str::from_utf8;

use parser::ArgumentParser;

#[cfg(test)]
pub fn check_ok(ap: &ArgumentParser, args: &[&str]) {
    let mut stdout = MemWriter::new();
    let mut stderr = MemWriter::new();
    let mut owned_args = Vec::new();
    for x in args.iter() {
        owned_args.push(x.to_string());
    }
    let res = ap.parse(owned_args, &mut stdout, &mut stderr);
    match res {
        Ok(()) => return,
        Err(x) => panic!(
            from_utf8(stderr.into_inner().as_slice()).unwrap().to_string() +
            format!("Expected ok, but found Exit({})", x).as_slice()),
    }
}

#[cfg(test)]
pub fn check_exit(ap: &ArgumentParser, args: &[&str]) {
    let mut stdout = MemWriter::new();
    let mut stderr = MemWriter::new();
    let mut owned_args = Vec::new();
    for x in args.iter() {
        owned_args.push(x.to_string());
    }
    let res = ap.parse(owned_args, &mut stdout, &mut stderr);
    match res {
        Err(0) => return,
        Err(x) => panic!(format!("Expected code {} got {}", 0us, x)),
        Ok(()) => panic!(format!("Expected failure, got success")),
    }
}

#[cfg(test)]
pub fn check_err(ap: &ArgumentParser, args: &[&str]) {
    let mut stdout = MemWriter::new();
    let mut stderr = MemWriter::new();
    let mut owned_args = Vec::new();
    for x in args.iter() {
        owned_args.push(x.to_string());
    }
    let res = ap.parse(owned_args, &mut stdout, &mut stderr);
    match res {
        Err(2) => return,
        Err(x) => panic!(format!("Expected code {} got {}", 2us, x)),
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

