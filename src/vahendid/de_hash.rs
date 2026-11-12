
use std::io::{self, Write};

use crate::vahendid::hashimine;

pub struct DeHashMenu;

impl DeHashMenu {
    pub fn show() -> io::Result<()> {
        let mut input = String::new();

        println!("==============================");
        println!("       DE-HASHIMISE TÖÖRIIST");
        println!("==============================");
        println!("1. Base64 dekodeerimine");
        println!("2. SHA-256 kontrollimine");
        println!("3. SHA-512 kontrollimine");
        println!("4. SHA3-256 kontrollimine");
        println!("5. Argon2 parooli kontrollimine");
        println!("==============================");

        print!("Vali tegevus >> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => {
                let mut encoded = String::new();

                print!("Sisesta Base64 tekst >> ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut encoded)?;

                match hashimine::decode_base64(encoded.trim()) {
                    Ok(decoded) => {
                        match String::from_utf8(decoded) {
                            Ok(text) => println!("Dekodeeritud tekst: {}", text),
                            Err(_) => println!("Tulemus ei ole korrektne UTF-8 tekst!"),
                        }
                    }
                    Err(err) => println!("Dekodeerimise viga: {}", err),
                }
            }

            "2" | "3" | "4" => {
                let mut original = String::new();
                let mut hashed = String::new();

                print!("Sisesta algne tekst >> ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut original)?;

                print!("Sisesta kontrollitav hash >> ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut hashed)?;

                let original = original.trim();
                let hashed = hashed.trim();

                let calculated = match input.trim() {
                    "2" => hashimine::hash_password(original),
                    "3" => hashimine::hash_sha512(original),
                    "4" => hashimine::hash_sha3(original),
                    _ => unreachable!(),
                };

                if calculated == hashed {
                    println!("Hash vastab algsele tekstile! SUCCESS");
                } else {
                    println!("Hash ei vasta algsele tekstile! ERROR");
                }
            }

            "5" => {
                let mut password = String::new();
                let mut hashed = String::new();

                print!("Sisesta parool >> ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut password)?;

                print!("Sisesta Argon2 hash >> ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut hashed)?;

                if hashimine::verify_argon2(
                    password.trim(),
                    hashed.trim()
                ) {
                    println!("Parool on õige! SUCCESS");
                } else {
                    println!("Vale parool või hash! ERROR");
                }
            }

            _ => {
                println!("Vale valik!");
            }
        }

        Ok(())
    }
}
