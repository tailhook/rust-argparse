use std::str::FromStr;
use std::io::{stdout, stderr};
extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store, List};

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Command {
    play,
    record,
}

impl FromStr for Command {
    type Err = ();
    fn from_str(src: &str) -> Result<Command, ()> {
        return match src {
            "play" => Ok(Command::play),
            "record" => Ok(Command::record),
            _ => Err(()),
        };
    }
}



fn play_command(verbose: bool, args: Vec<String>) {
    let mut output = "".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Plays a sound");
        ap.refer(&mut output)
            .add_option(&["--output"], Store,
                r#"Output sink to play to"#);
        match ap.parse(args, &mut stdout(), &mut stderr()) {
            Ok(()) =>  {}
            Err(x) => {
                std::process::exit(x);
            }
        }
    }
    println!("Verbosity: {}, Output: {}", verbose, output);
}

fn record_command(verbose: bool, args: Vec<String>) {
    let mut input = "".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Records a sound");
        ap.refer(&mut input)
            .add_option(&["--input"], Store,
                r#"Output source to record from"#);
        match ap.parse(args, &mut stdout(), &mut stderr()) {
            Ok(()) =>  {}
            Err(x) => {
                std::process::exit(x);
            }
        }
    }
    println!("Verbosity: {}, Input: {}", verbose, input);
}

fn main() {
    let mut verbose = false;
    let mut subcommand = Command::play;
    let mut args = vec!();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Plays or records sound");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Be verbose");
        ap.refer(&mut subcommand).required()
            .add_argument("command", Store,
                r#"Command to run (either "play" or "record")"#);
        ap.refer(&mut args)
            .add_argument("arguments", List,
                r#"Arguments for command"#);
        ap.stop_on_first_argument(true);
        ap.parse_args_or_exit();
    }

    args.insert(0, format!("subcommand {:?}", subcommand));
    match subcommand {
        Command::play => play_command(verbose, args),
        Command::record => record_command(verbose, args),
    }
}
