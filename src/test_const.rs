use parser::ArgumentParser;
use super::{PushConst};
use test_parser::{check_ok};


fn push_const(args: &[&str]) -> Vec<u32> {
    let mut res = vec!();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut res)
          .add_option(&["-o", "--one"], PushConst(1),
            "Add one to the list")
          .add_option(&["-t", "--two"], PushConst(2),
            "Add two to the list")
          .add_option(&["-3", "--three"], PushConst(3),
            "Add three to the list");
        check_ok(&ap,  args);
    }
    return res;
}

#[test]
fn test_push() {
    assert_eq!(push_const(&["./argparse_test"]), vec!());
    assert_eq!(push_const(&["./argparse_test", "--one"]), vec!(1));
    assert_eq!(push_const(&["./argparse_test", "-3"]), vec!(3));
    assert_eq!(push_const(&["./argparse_test", "-oo3tt"]), vec!(1, 1, 3, 2, 2));
}
