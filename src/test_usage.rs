use std::old_io::MemWriter;
use std::str::from_utf8;

use parser::ArgumentParser;
use super::{Store, List};

#[test]
fn test_empty() {
    let ap = ArgumentParser::new();
    let mut buf = MemWriter::new();
    assert_eq!(ap.print_usage("./argparse_test", &mut buf), Ok(()));
    assert_eq!("Usage:\n    ./argparse_test\n",
        from_utf8(buf.into_inner().as_slice()).unwrap());
}

#[test]
fn test_options() {
    let mut val = 0;
    let mut buf = MemWriter::new();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut val)
          .add_option(&["--value"], box Store::<isize>,
            "Set integer value");
        assert_eq!(ap.print_usage("./argparse_test", &mut buf), Ok(()));
    }
    assert_eq!("Usage:\n    ./argparse_test [OPTIONS]\n",
        from_utf8(buf.into_inner().as_slice()).unwrap());
}

#[test]
fn test_argument() {
    let mut val = 0;
    let mut ap = ArgumentParser::new();
    ap.refer(&mut val)
      .add_argument("value", box Store::<isize>,
        "Integer value");
    let mut buf = MemWriter::new();
    assert_eq!(ap.print_usage("./argparse_test", &mut buf), Ok(()));
    assert_eq!("Usage:\n    ./argparse_test [VALUE]\n",
        from_utf8(buf.into_inner().as_slice()).unwrap());
}

#[test]
fn test_arguments() {
    let mut v1 = 0;
    let mut v2 = Vec::new();
    let mut ap = ArgumentParser::new();
    ap.refer(&mut v1)
      .add_argument("v1", box Store::<isize>,
        "Integer value 1");
    ap.refer(&mut v2)
      .add_argument("v2", box List::<isize>,
        "More values");
    let mut buf = MemWriter::new();
    assert_eq!(ap.print_usage("./argparse_test", &mut buf), Ok(()));
    assert_eq!("Usage:\n    ./argparse_test [V1] [V2 ...]\n",
        from_utf8(buf.into_inner().as_slice()).unwrap());
}
