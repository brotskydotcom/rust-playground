extern crate keyring;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let service = "Adobe App Info (UGhvdG9zaG9wMXt9MjAxODA3MjAwMw)";
    let username = "App Info";

    let keyring = keyring::Keyring::new(&service, &username);

    let note = keyring.get_password()?;
    println!("The note value is '{}'", note);

    Ok(())
}
