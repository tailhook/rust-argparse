use std::io::MemWriter;
use std::str::from_utf8;

use parser::ArgumentParser;
use super::{Store, List};

#[test]
fn test_empty() {
    let mut ap = ArgumentParser::new();
    let mut buf = MemWriter::new();
    ap.set_description("Test program");
    assert_eq!(ap.print_help("./argparse_test", &mut buf), Ok(()));
    assert_eq!("Usage:\n".to_string()
        + "    ./argparse_test\n"
        + "\n"
        + "Test program\n"
        + "\n"
        + "optional arguments:\n"
        + "  -h,--help             show this help message and exit\n"
        , from_utf8(buf.unwrap().as_slice()).unwrap().to_string());
}

#[test]
fn test_options() {
    let mut ap = ArgumentParser::new();
    let mut val = 0;
    let mut val2 = 0;
    ap.set_description("Test program. The description of the program is ought
        to be very long, because we want to test how word wrapping works for
        it. So some more text would be ok for the test");
    ap.refer(&mut val)
      .add_option(["--value"], box Store::<int>,
        "Set integer value");
    ap.refer(&mut val2)
      .add_option(["-L", "--long-option"], box Store::<int>,
        "Long option value");
    let mut buf = MemWriter::new();
    assert_eq!(ap.print_help("./argparse_test", &mut buf), Ok(()));
    assert_eq!("Usage:\n".to_string()
        + "    ./argparse_test [OPTIONS]

Test program. The description of the program is ought to be very long, because
we want to test how word wrapping works for it. So some more text would be ok
for the test\n"
        + "\n"
        + "optional arguments:\n"
        + "  -h,--help             show this help message and exit\n"
        + "  --value VALUE         Set integer value\n"
        + "  -L,--long-option LONG_OPTION\n"
        + "                        Long option value\n"
        , from_utf8(buf.unwrap().as_slice()).unwrap().to_string());
}

#[test]
fn test_argument() {
    let mut ap = ArgumentParser::new();
    let mut val = 0;
    ap.set_description("Test program");
    ap.refer(&mut val)
      .add_argument("value", box Store::<int>,
        "Integer value");
    let mut buf = MemWriter::new();
    assert_eq!(ap.print_help("./argparse_test", &mut buf), Ok(()));
    assert_eq!("Usage:\n".to_string()
        + "    ./argparse_test [VALUE]\n"
        + "\n"
        + "Test program\n"
        + "\n"
        + "positional arguments:\n"
        + "  value                 Integer value\n"
        + "\n"
        + "optional arguments:\n"
        + "  -h,--help             show this help message and exit\n"
        , from_utf8(buf.unwrap().as_slice()).unwrap().to_string());
}

#[test]
fn test_arguments() {
    let mut ap = ArgumentParser::new();
    let mut v1 = 0;
    let mut v2 = Vec::new();
    ap.set_description("Test program");
    ap.refer(&mut v1)
      .add_argument("v1", box Store::<int>,
        "Integer value 1");
    ap.refer(&mut v2)
      .add_argument("v2", box List::<int>,
        "More values");
    let mut buf = MemWriter::new();
    assert_eq!(ap.print_help("./argparse_test", &mut buf), Ok(()));
    assert_eq!("Usage:\n".to_string()
        + "    ./argparse_test [V1] [V2 ...]\n"
        + "\n"
        + "Test program\n"
        + "\n"
        + "positional arguments:\n"
        + "  v1                    Integer value 1\n"
        + "  v2                    More values\n"
        + "\n"
        + "optional arguments:\n"
        + "  -h,--help             show this help message and exit\n"
        , from_utf8(buf.unwrap().as_slice()).unwrap().to_string());
}

#[test]
fn test_req_arguments() {
    let mut ap = ArgumentParser::new();
    let mut v1 = 0;
    let mut v2 = Vec::new();
    ap.set_description("Test program");
    ap.refer(&mut v1)
      .add_argument("v1", box Store::<int>,
        "Integer value 1")
      .required();
    ap.refer(&mut v2)
      .add_argument("v2", box List::<int>,
        "More values")
      .required();
    let mut buf = MemWriter::new();
    assert_eq!(ap.print_help("./argparse_test", &mut buf), Ok(()));
    assert_eq!("Usage:\n".to_string()
        + "    ./argparse_test V1 V2 [...]\n"
        + "\n"
        + "Test program\n"
        + "\n"
        + "positional arguments:\n"
        + "  v1                    Integer value 1\n"
        + "  v2                    More values\n"
        + "\n"
        + "optional arguments:\n"
        + "  -h,--help             show this help message and exit\n"
        , from_utf8(buf.unwrap().as_slice()).unwrap().to_string());
}

#[test]
fn test_metavar() {
    let mut ap = ArgumentParser::new();
    let mut val2 = 0;
    ap.set_description("Test program.");
    ap.refer(&mut val2)
      .add_option(["-L", "--long-option"], box Store::<int>,
        "Long option value")
      .metavar("VAL");
    let mut buf = MemWriter::new();
    assert_eq!(ap.print_help("./argparse_test", &mut buf), Ok(()));
    assert_eq!("Usage:\n".to_string()
        + "    ./argparse_test [OPTIONS]\n"
        + "\n"
        + "Test program.\n"
        + "\n"
        + "optional arguments:\n"
        + "  -h,--help             show this help message and exit\n"
        + "  -L,--long-option VAL  Long option value\n"
        , from_utf8(buf.unwrap().as_slice()).unwrap().to_string());
}
