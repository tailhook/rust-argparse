use Print;
use action::{IFlagAction, ParseResult};

impl IFlagAction for Print {
    fn parse_flag(&self) -> ParseResult {
        return ParseResult::Exit(Some(self.0.clone()));
    }
}
