
use clap::{builder::Str, Parser};
use colour::{black, print_bold, write_red, write_yellow_bold, yellow_bold};
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

    yellow_bold!("Password len: ");
    stdout().flush()?; // necessary, since for performance reasons, because stdout it is line-buffered
    stdin().read_line(&mut size_str)?;
    let size = size_str.trim().parse::<usize>()?;

    yellow_bold!("Minimum symbols: ");
    stdout().flush()?;
    stdin().read_line(&mut min_symbols_str)?;
    let min_symbols = min_symbols_str.trim().parse::<usize>()?;

    

    Ok(String::from("passwd"))
}