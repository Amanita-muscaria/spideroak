extern crate clap;

use clap::Parser;
use sp_lib::decrypt;

#[derive(Parser, Debug)]
struct Args {
    key: u8,
    msg: String,
}

fn main() {
    let args = Args::parse();
    print!("{}", decrypt(args.key, args.msg));
}
