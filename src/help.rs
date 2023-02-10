use std::str::CharIndices;
use std::io::Result as IoResult;
use std::io::Write;

use super::action::{IFlagAction, ParseResult};
use super::action::ParseResult::Help;

pub struct HelpAction;

impl IFlagAction for HelpAction {
    fn parse_flag(&self) -> ParseResult {
        return Help;
    }
}


struct WordsIter<'a> {
    data: &'a str,
    iter: CharIndices<'a>,
}

impl<'a> WordsIter<'a> {
    fn new(data: &'a str) -> WordsIter<'a> {
        return WordsIter {
            data: data,
            iter: data.char_indices(),
            };
    }
}

impl<'a> Iterator for WordsIter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        let word_start;
        loop {
            let (idx, ch) = match self.iter.next() {
                None => return None,
                Some((idx, ch)) => (idx, ch),
            };
            match ch {
                ' ' | '\t' | '\r' | '\n' => continue,
                _ => {
                    word_start = idx;
                    break;
                }
            }
        }
        loop {
            let (idx, ch) = match self.iter.next() {
                None => break,
                Some((idx, ch)) => (idx, ch),
            };
            match ch {
                ' ' | '\t' | '\r' | '\n' => {
                    return Some(&self.data[word_start..idx]);
                }
                _ => continue,
            }
        }
        return Some(&self.data[word_start..self.data.len()]);
    }
}

pub fn wrap_text(buf: &mut dyn Write, data: &str, width: usize, indent: usize)
    -> IoResult<()>
{
    let mut witer = WordsIter::new(data);
    let mut off = indent;
    match witer.next() {
        None => {
            return Ok(());
        }
        Some(word) => {
            buf.write_all(word.as_bytes())?;
            off += word.len();
        }
    }
    for word in witer {
        if off + word.len() + 1 > width {
            buf.write_all(b"\n")?;
            for _ in 0..indent {
                buf.write_all(b" ")?;
            }
            off = indent;
        } else {
            buf.write_all(b" ")?;
            off += 1;
        }
        buf.write_all(word.as_bytes())?;
        off += word.len();
    }
    return Ok(());
}
