use parser::{ArgumentParser, cell, IncrInt, DecrInt, SetInt};

#[test]
fn incr_int() {
    let mut val = 0;
    let mut ap = ArgumentParser::new();
    ap.add_option(~["-i", "--incr"],
        "Increment value",
        IncrInt(cell(&mut val)));
    assert!(val == 0);
    ap.parse_list(~[~"./argparse_test"]);
    assert!(val == 0);
    ap.parse_list(~[~"./argparse_test", ~"--incr"]);
    assert!(val == 1);
    ap.parse_list(~[~"./argparse_test", ~"-iiiii"]);
    assert!(val == 6);
}

#[test]
fn test_decr_int() {
    let mut val = 5;
    let mut ap = ArgumentParser::new();
    ap.add_option(~["-d", "--decr"],
        "Decrement value",
        DecrInt(cell(&mut val)));
    assert!(val == 5);
    ap.parse_list(~[~"./argparse_test"]);
    assert!(val == 5);
    ap.parse_list(~[~"./argparse_test", ~"--decr"]);
    assert!(val == 4);
    ap.parse_list(~[~"./argparse_test", ~"-ddddd"]);
    assert!(val == -1);
}

#[test]
fn test_incr_decr() {
    let mut val = 0;
    {
        let mut ap = ArgumentParser::new();
        let c = cell(&mut val);
        ap.add_option(~["-d", "--decr"],
            "Decrement value",
            DecrInt(c.clone()));
        ap.add_option(~["-i", "--incr"],
            "Increment value",
            IncrInt(c.clone()));
        assert!(val == 0);
        ap.parse_list(~[~"./argparse_test", ~"-iiddd", ~"--incr", ~"-iii"]);
    }
    assert_eq!(val, 3);
}

#[test]
fn test_int() {
    let mut val = 0;
    let mut ap = ArgumentParser::new();
    ap.add_option(~["-s", "--set"],
        "Set integer value",
        SetInt(cell(&mut val)));
    ap.parse_list(~[~"./argparse_test", ~"-s", ~"10"]);
    assert!(val == 10);
    ap.parse_list(~[~"./argparse_test", ~"--set", ~"99"]);
    assert!(val == 99);
    ap.parse_list(~[~"./argparse_test", ~"-s", ~"7", ~"-s77"]);
    assert!(val == 77);
    ap.parse_list(~[~"./argparse_test", ~"-s333", ~"--set=123"]);
    assert!(val == 123);
}
