use parser::ArgumentParser;
use super::Print;
use test_parser::{check_exit};

fn print_str(args: &[&str]) -> String {
    let mut ap = ArgumentParser::new();
    ap.add_option(&["-V", "--version"],
        Print("program 0.42".to_string()),
        "Print version");
    let (out, _) = check_exit(&ap, args);
    String::from_utf8_lossy(&out).into_owned()
}

#[test]
fn test_str() {
    let expected = "program 0.42\n".to_string();
    assert_eq!(print_str(&["./argparse_test", "-V"]), expected);
    assert_eq!(print_str(&["./argparse_test", "--version"]), expected);
}
