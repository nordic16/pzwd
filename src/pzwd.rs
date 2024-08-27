
use clap::{builder::Str, Parser};
use std::{io::{stdin, stdout, Read, Write}, usize};
use crate::crypt;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    len: usize,
    symbols: bool,
    min_symbols: usize,
}

// NOTE: using ANSI escape codes might not work for terminal emulators that override default colors.
pub(crate) fn interactive() -> anyhow::Result<String> {
    let mut size_str: String = Default::default();
    let mut min_symbols_str: String = Default::default();

    print!("\x1b[1;33mPassword len:\x1b[1;0m ");
    stdout().flush()?; // necessary, since, for performance reasons, stdout is line-buffered
    stdin().read_line(&mut size_str)?;
    let size = size_str.trim().parse::<usize>()?;

    print!("\x1b[1;33mMinimum symbols:\x1b[1;0m ");
    stdout().flush()?;
    stdin().read_line(&mut min_symbols_str)?;
    let min_symbols = min_symbols_str.trim().parse::<usize>()?;

    Ok(crypt::generate_secure_passwd(size))
}
