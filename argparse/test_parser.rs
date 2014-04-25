use parser::ArgumentParser;

#[test]
fn test_no_arg() {
    let ap = ArgumentParser::new();
    ap.parse_list(~[~"./argparse_test"]);
}
