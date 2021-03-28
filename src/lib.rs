use crate::emoji::emojify;
use std::error::Error;
use std::io;

mod emoji;
mod map;

pub fn run() -> Result<(), Box<dyn Error>> {
    Ok(emojify(io::stdin().lock(), &mut io::stdout())?)
}
