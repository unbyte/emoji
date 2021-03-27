use emoji::run;
use std::io;
use std::io::prelude::*;

fn main() {
    if let Err(err) = run() {
        let _ = writeln!(io::stderr(), "{}", err);
    }
}