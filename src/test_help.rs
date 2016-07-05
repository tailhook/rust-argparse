use std::str::from_utf8;

use parser::ArgumentParser;
use super::{Store, List};

#[test]
fn test_empty() {
    let mut ap = ArgumentParser::new();
    let mut buf = Vec::<u8>::new();
    ap.set_description("Test program");
    assert!(ap.print_help("./argparse_test", &mut buf).is_ok());
    assert_eq!("Usage:\n".to_string()
        + "  ./argparse_test\n"
        + "\n"
        + "Test program\n"
        + "\n"
        + "Optional arguments:\n"
        + "  -h,--help             Show this help message and exit\n"
        , from_utf8(&buf[..]).unwrap().to_string());
}

#[test]
fn test_options() {
    let mut val = 0;
    let mut val2 = 0;
    let mut ap = ArgumentParser::new();
    ap.set_description("Test program. The description of the program is ought
        to be very long, because we want to test how word wrapping works for
        it. So some more text would be ok for the test");
    ap.refer(&mut val)
      .add_option(&["--value"], Store,
        "Set integer value");
    ap.refer(&mut val2)
      .add_option(&["-L", "--long-option"], Store,
        "Long option value");
    let mut buf = Vec::<u8>::new();
    assert!(ap.print_help("./argparse_test", &mut buf).is_ok());
    assert_eq!("Usage:\n".to_string()
        + "  ./argparse_test [OPTIONS]

Test program. The description of the program is ought to be very long, because
we want to test how word wrapping works for it. So some more text would be ok
for the test\n"
        + "\n"
        + "Optional arguments:\n"
        + "  -h,--help             Show this help message and exit\n"
        + "  --value VALUE         Set integer value\n"
        + "  -L,--long-option LONG_OPTION\n"
        + "                        Long option value\n"
        , from_utf8(&buf[..]).unwrap().to_string());
}

#[test]
fn test_argument() {
    let mut val = 0;
    let mut ap = ArgumentParser::new();
    ap.set_description("Test program");
    ap.refer(&mut val)
      .add_argument("value", Store,
        "Integer value");
    let mut buf = Vec::<u8>::new();
    assert!(ap.print_help("./argparse_test", &mut buf).is_ok());
    assert_eq!("Usage:\n".to_string()
        + "  ./argparse_test [VALUE]\n"
        + "\n"
        + "Test program\n"
        + "\n"
        + "Positional arguments:\n"
        + "  value                 Integer value\n"
        + "\n"
        + "Optional arguments:\n"
        + "  -h,--help             Show this help message and exit\n"
        , from_utf8(&buf[..]).unwrap().to_string());
}

#[test]
fn test_arguments() {
    let mut v1 = 0;
    let mut v2 = Vec::<u32>::new();
    let mut ap = ArgumentParser::new();
    ap.set_description("Test program");
    ap.refer(&mut v1)
      .add_argument("v1", Store,
        "Integer value 1");
    ap.refer(&mut v2)
      .add_argument("v2", List,
        "More values");
    let mut buf = Vec::<u8>::new();
    assert!(ap.print_help("./argparse_test", &mut buf).is_ok());
    assert_eq!("Usage:\n".to_string()
        + "  ./argparse_test [V1] [V2 ...]\n"
        + "\n"
        + "Test program\n"
        + "\n"
        + "Positional arguments:\n"
        + "  v1                    Integer value 1\n"
        + "  v2                    More values\n"
        + "\n"
        + "Optional arguments:\n"
        + "  -h,--help             Show this help message and exit\n"
        , from_utf8(&buf[..]).unwrap().to_string());
}

#[test]
fn test_req_arguments() {
    let mut v1 = 0;
    let mut v2 = Vec::<u32>::new();
    let mut ap = ArgumentParser::new();
    ap.set_description("Test program");
    ap.refer(&mut v1)
      .add_argument("v1", Store,
        "Integer value 1")
      .required();
    ap.refer(&mut v2)
      .add_argument("v2", List,
        "More values")
      .required();
    let mut buf = Vec::<u8>::new();
    assert!(ap.print_help("./argparse_test", &mut buf).is_ok());
    assert_eq!("Usage:\n".to_string()
        + "  ./argparse_test V1 V2 [...]\n"
        + "\n"
        + "Test program\n"
        + "\n"
        + "Positional arguments:\n"
        + "  v1                    Integer value 1\n"
        + "  v2                    More values\n"
        + "\n"
        + "Optional arguments:\n"
        + "  -h,--help             Show this help message and exit\n"
        , from_utf8(&buf[..]).unwrap().to_string());
}

#[test]
fn test_metavar() {
    let mut val2 = 0;
    let mut ap = ArgumentParser::new();
    ap.set_description("Test program.");
    ap.refer(&mut val2)
      .add_option(&["-L", "--long-option"], Store,
        "Long option value")
      .metavar("VAL");
    let mut buf = Vec::<u8>::new();
    assert!(ap.print_help("./argparse_test", &mut buf).is_ok());
    assert_eq!("Usage:\n".to_string()
        + "  ./argparse_test [OPTIONS]\n"
        + "\n"
        + "Test program.\n"
        + "\n"
        + "Optional arguments:\n"
        + "  -h,--help             Show this help message and exit\n"
        + "  -L,--long-option VAL  Long option value\n"
        , from_utf8(&buf[..]).unwrap().to_string());
}
