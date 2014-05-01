use std::io::MemWriter;
use std::str::from_utf8;

use parser::ArgumentParser;

#[cfg(test)]
pub fn check_ok(ap: &ArgumentParser, args: ~[~str]) {
    let mut stdout = MemWriter::new();
    let mut stderr = MemWriter::new();
    let res = ap.parse(args, &mut stdout, &mut stderr);
    match res {
        Ok(()) => return,
        Err(x) => fail!(
            from_utf8(stderr.unwrap()).unwrap() +
            format!("Expected ok, but found Exit({})", x)),
    }
}

#[cfg(test)]
pub fn check_exit(ap: &ArgumentParser, args: ~[~str]) {
    let mut stdout = MemWriter::new();
    let mut stderr = MemWriter::new();
    let res = ap.parse(args, &mut stdout, &mut stderr);
    match res {
        Err(0) => return,
        Err(x) => fail!(format!("Expected code {} got {}", 0, x)),
        Ok(()) => fail!(format!("Expected failure, got success")),
    }
}

#[cfg(test)]
pub fn check_err(ap: &ArgumentParser, args: ~[~str]) {
    let mut stdout = MemWriter::new();
    let mut stderr = MemWriter::new();
    let res = ap.parse(args, &mut stdout, &mut stderr);
    match res {
        Err(2) => return,
        Err(x) => fail!(format!("Expected code {} got {}", 2, x)),
        Ok(()) => fail!(format!("Expected failure, got success")),
    }
}

#[test]
fn test_no_arg() {
    let ap = ArgumentParser::new();
    check_ok(&ap, ~[~"./argparse_test"]);
    check_err(&ap, ~[~"./argparse_test", ~"a"]);
    check_err(&ap, ~[~"./argparse_test", ~"-a"]);
    check_err(&ap, ~[~"./argparse_test", ~"--an-option"]);
}

