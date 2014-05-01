use std::io::MemWriter;
use std::str::from_utf8;

use parser::ArgumentParser;
use generic::{Store, List};

#[test]
fn test_empty() {
    let mut ap = ArgumentParser::new();
    let mut buf = MemWriter::new();
    ap.set_description("Test program");
    assert_eq!(ap.print_help("./argparse_test", &mut buf), Ok(()));
    assert_eq!(&"Usage:\n"
        + &"    ./argparse_test\n"
        + &"\n"
        + &"Test program\n"
        , from_utf8(buf.unwrap()).unwrap().to_owned());
}

#[test]
fn test_options() {
    let mut ap = ArgumentParser::new();
    let mut val = 0;
    ap.set_description("Test program. The description of the program is ought
        to be very long, because we want to test how word wrapping works for
        it. So some more text would be ok for the test");
    ap.refer(&mut val)
      .add_option(~["--value"], ~Store::<int>,
        "Set integer value");
    let mut buf = MemWriter::new();
    assert_eq!(ap.print_help("./argparse_test", &mut buf), Ok(()));
    assert_eq!(&"Usage:\n"
        + &"    ./argparse_test [options]

Test program. The description of the program is ought to be very long, because
we want to test how word wrapping works for it. So some more text would be ok
for the test\n"
        , from_utf8(buf.unwrap()).unwrap().to_owned());
}

#[test]
fn test_argument() {
    let mut ap = ArgumentParser::new();
    let mut val = 0;
    ap.set_description("Test program");
    ap.refer(&mut val)
      .add_argument("value", ~Store::<int>,
        "Integer value");
    let mut buf = MemWriter::new();
    assert_eq!(ap.print_help("./argparse_test", &mut buf), Ok(()));
    assert_eq!(&"Usage:\n"
        + &"    ./argparse_test [VALUE]\n"
        + &"\n"
        + &"Test program\n"
        , from_utf8(buf.unwrap()).unwrap().to_owned());
}

#[test]
fn test_arguments() {
    let mut ap = ArgumentParser::new();
    let mut v1 = 0;
    let mut v2 = ~[];
    ap.set_description("Test program");
    ap.refer(&mut v1)
      .add_argument("v1", ~Store::<int>,
        "Integer value 1");
    ap.refer(&mut v2)
      .add_argument("v2", ~List::<int>,
        "More values");
    let mut buf = MemWriter::new();
    assert_eq!(ap.print_help("./argparse_test", &mut buf), Ok(()));
    assert_eq!(&"Usage:\n"
        + &"    ./argparse_test [V1] [V2 ...]\n"
        + &"\n"
        + &"Test program\n"
        , from_utf8(buf.unwrap()).unwrap().to_owned());
}
