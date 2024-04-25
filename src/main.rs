mod hash;

use anyhow::Result;
use hash::crack;
use std::env;

const SHA1_HEX_STRING_LENGTH: usize = 40;
const USAGE: &str = "Usage:\nsha1_cracker: <wordlist.txt> <sha1_hash>";

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("{}", USAGE);
    }
    crack(args)?;
    Ok(())
}
