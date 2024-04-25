use crate::SHA1_HEX_STRING_LENGTH;
use anyhow::{bail, Result};
use sha1::Digest;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn crack(args: Vec<String>) -> Result<()> {
    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        bail!("sha1 hash is not valid")
    }

    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist_file);

    for line in reader.lines() {
        let line = line?;
        let common_password = line.trim();
        if hash_to_crack == hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
            println!("Password found: {}", &common_password);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_1() {
        let args = vec!["one".to_string(), "two".to_string(), "three".to_string()];

        let res = super::crack(args).unwrap_err();

        assert_eq!(format!("{}", res), "sha1 hash is not valid")
    }
}
