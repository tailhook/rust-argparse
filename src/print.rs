use Print;
use action::{IFlagAction, ParseResult};

impl IFlagAction for Print {
    fn parse_flag(&self) -> ParseResult {
        if self.0.ends_with("\n") {
            print!("{}", self.0);
        } else {
            println!("{}", self.0);
        }
        return ParseResult::Exit;
    }
}
