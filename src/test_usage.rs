use std::str::from_utf8;

use parser::ArgumentParser;
use super::{Store, List};

#[test]
fn test_empty() {
    let ap = ArgumentParser::new();
    let mut buf = Vec::<u8>::new();
    assert!(ap.print_usage("./argparse_test", &mut buf).is_ok());
    assert_eq!("Usage:\n  ./argparse_test\n", from_utf8(&buf[..]).unwrap());
}

#[test]
fn test_options() {
    let mut val = 0;
    let mut buf = Vec::<u8>::new();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut val)
          .add_option(&["--value"], Store,
            "Set integer value");
        assert!(ap.print_usage("./argparse_test", &mut buf).is_ok());
    }
    assert_eq!("Usage:\n  ./argparse_test [OPTIONS]\n",
        from_utf8(&buf[..]).unwrap());
}

#[test]
fn test_argument() {
    let mut val = 0;
    let mut ap = ArgumentParser::new();
    ap.refer(&mut val)
      .add_argument("value", Store,
        "Integer value");
    let mut buf = Vec::<u8>::new();
    assert!(ap.print_usage("./argparse_test", &mut buf).is_ok());
    assert_eq!("Usage:\n  ./argparse_test [VALUE]\n",
        from_utf8(&buf[..]).unwrap());
}

#[test]
fn test_arguments() {
    let mut v1 = 0;
    let mut v2 = Vec::<u32>::new();
    let mut ap = ArgumentParser::new();
    ap.refer(&mut v1)
      .add_argument("v1", Store,
        "Integer value 1");
    ap.refer(&mut v2)
      .add_argument("v2", List,
        "More values");
    let mut buf = Vec::<u8>::new();
    assert!(ap.print_usage("./argparse_test", &mut buf).is_ok());
    assert_eq!("Usage:\n  ./argparse_test [V1] [V2 ...]\n",
        from_utf8(&buf[..]).unwrap());
}
