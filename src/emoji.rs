use std::io::{self, BufRead, Write};

use regex::{Captures, Regex};

use crate::map::EMOJI_MAP;

pub fn emojify<R, W>(reader: R, writer: &mut W) -> io::Result<()>
where
    R: BufRead,
    W: Write,
{
    let emoji_regexp = Regex::new(r":([0-9a-zA-Z_\-+]+):").unwrap();
    for line in reader.lines() {
        writeln!(
            writer,
            "{}",
            emoji_regexp.replace_all(line?.as_str(), EmojiReplacer)
        )?;
    }
    Ok(())
}

#[inline]
fn map_to_emoji(key: &str) -> Option<&'static str> {
    EMOJI_MAP.get(key).map(|str| *str)
}

struct EmojiReplacer;

impl regex::Replacer for EmojiReplacer {
    fn replace_append(&mut self, caps: &Captures<'_>, result: &mut String) {
        result.push_str(map_to_emoji(&caps[1]).unwrap_or(&caps[0]));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_to_emoji() {
        assert_eq!(map_to_emoji("heart").unwrap(), "\u{2764}");
        assert!(map_to_emoji("not_existed_one").is_none());
    }

    #[test]
    fn test_emojify() {
        let mut output = Vec::new();
        assert!(emojify("I:heart::not_existed_one:".as_bytes(), &mut output).is_ok());
        let output = String::from_utf8(output).unwrap();
        assert!(output.contains(":not_existed_one:"));
        assert!(!output.contains(":heart:"));
    }
}
