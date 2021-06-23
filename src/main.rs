extern crate base64;
use std::error::Error;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let bytes = buffer.trim().as_bytes();
    let result = base64::encode_config(bytes, base64::URL_SAFE_NO_PAD);
    print!("{}", result);
    Ok(())
}
