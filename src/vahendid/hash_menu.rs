
use std::io::{self, Write};

use crate::vahendid::hashimine;

pub struct HashMenu;

impl HashMenu {
    pub fn show() -> io::Result<()> {
        let mut input = String::new();

        println!("==============================");
        println!("       HASHIMISE TÖÖRIIST");
        println!("==============================");

        print!("Sisesta tekst >> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        let input = input.trim();

        if input.is_empty() {
            println!("Tekst ei tohi olla tühi!");
            return Ok(());
        }

        println!("\nSHA-256: {}", hashimine::hash_password(input));
        println!("SHA-512: {}", hashimine::hash_sha512(input));
        println!("SHA3-256: {}", hashimine::hash_sha3(input));
        println!("Base64: {}", hashimine::encode_base64(input));

        println!("\nArgon2:");

        match hashimine::hash_argon2(input) {
            Ok(hash) => println!("{}", hash),
            Err(err) => println!("Hashimise viga: {}", err),
        }

        Ok(())
    }
}
