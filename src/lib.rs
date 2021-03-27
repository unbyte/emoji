mod emoji;

use crate::emoji::map_to_emoji;
use std::io::BufRead;
use std::io;
use regex::{Regex, Captures};
use std::error::Error;

struct EmojiRegexReplacer;
impl regex::Replacer for EmojiRegexReplacer {
  fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
    dst.push_str( map_to_emoji(&caps[0]).unwrap_or(&caps[0]));
  }
}

fn emojify<R>(reader: R) -> io::Result<()>
  where R: BufRead {
  let emoji_regexp = Regex::new(r"(:[0-9a-zA-Z_\-+]+:)").unwrap();
  for line in reader.lines() {
    println!("{}",
             emoji_regexp.replace_all(
               line?.as_str(),
               EmojiRegexReplacer,
             )
    );
  }
  Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
  Ok(emojify(io::stdin().lock())?)
}

