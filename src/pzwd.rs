
use clap::{builder::Str, Parser};
use std::{io::{stdin, stdout, Read, Write}, usize};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    len: usize,
    symbols: bool,
    min_symbols: usize,
}


pub(crate) fn interactive() -> anyhow::Result<String> {
    let mut size_str: String = Default::default();
    let mut min_symbols_str: String = Default::default();

    print!("Password length: ");
    stdout().flush()?; // necessary, since for performance reasons, because stdout it is line-buffered
    stdin().read_line(&mut size_str)?;
    let size = size_str.parse::<usize>()?;

    print!("Minimum symbols: ");
    stdout().flush()?;
    stdin().read_line(&mut min_symbols_str)?;
    let min_symbols = min_symbols_str.parse::<usize>()?;

    

    Ok(String::from("passwd"))
}