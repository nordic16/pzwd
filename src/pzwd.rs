use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    len: usize,
    symbols: bool,
    min_symbols: usize,
}


pub fn interactive() {
    println!("Want to try?");
}