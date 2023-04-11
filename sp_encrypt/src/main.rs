extern crate clap;

use clap::Parser;
use sp_lib::encrypt;

#[derive(Parser, Debug)]
struct Args {
    key: u8,
    msg: String,
}

fn main() {
    let args = Args::parse();
    print!("{}", encrypt(args.key, args.msg));
}
