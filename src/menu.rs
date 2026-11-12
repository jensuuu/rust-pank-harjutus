use std::io::{self, Write};
use std::fs;

use crate::bank;
use crate::bank::User;
use crate::vahendid::hash_menu;
use crate::vahendid::de_hash;
pub struct Menu;


impl Menu {

    pub fn show_menu() -> io::Result<()> {

        // nagu create thread fivemis aga liht loop
        loop {

            // kasutaja sisestus
            let mut user_option = String::new();

            println!("==============================");
            println!("     TERE TULEMAST PANKA!");
            println!("==============================");
            println!("1. Loo konto");
            println!("2. Logi sisse");
            println!("3. Välju");
            println!("==============================");
            println!("     MUUD VAHENDID");
            println!("==============================");
            println!("4. Hashimine");
            println!("5. De-hashimine");
            println!("==============================");


            println!("Vali õige tegevus >>");

            io::stdout().flush()?; // sund tekst ekraanile

            // loeb kasutaja sisendit
            io::stdin().read_line(&mut user_option)?; 

            // kontrollib sisendi määrust kas on või on tühi
            let user_option: u8 = match user_option.trim().parse() {
                Ok(option) => option,
                Err(_) => {
                    println!("Sisestage number!");
                    continue;
                }
            };
        
        // kasutaja valiku kontroll
        match user_option {
                1 => {
                    bank::User::create()?;
                },
                2 => {
                    println!("Sisse logimine");
                    bank::User::login()?;
                },
                3 => {
                    println!("Head aega!");
                    break;
                    // Ok(()) ei tohi siin olla
                },
                4 => {
                    hash_menu::HashMenu::show()?;

                },
                5 => {
                    println!("De-hashimise tööriist");
                    de_hash::DeHashMenu::show()?;
                },
                _ => {
                    println!("Pole võimalik!");
                }
            }



            println!("TERVIST PANGANDUSE POOLT! {}", user_option);


           
        }

        // tagastus ei tohi olla loopi sees
        Ok(())

    }




    // kasutaja pangakonto menüü
    pub fn account_menu(mut user: User, account: &str) -> io::Result<()> {
        loop {
            println!("\n==============================");
            println!("       PANGA KONTOMENÜÜ");
            println!("==============================");
            println!("Kasutaja: {}", user.username);
            println!("Saldo: {:.2} EUR", user.balance);
            println!("==============================");
            println!("1. Vaata kontojääki");
            println!("2. Lisa raha");
            println!("3. Väljasta raha");
            println!("4. Logi välja");
            println!("==============================");

            print!("Vali tegevus >> ");
            io::stdout().flush()?;

            let mut option = String::new();
            io::stdin().read_line(&mut option)?;

            match option.trim() {
                "1" => {
                    println!("Kontojääk: {:.2} EUR", user.balance);
                }

                "2" => {
                    print!("Sisesta lisatav summa >> ");
                    io::stdout().flush()?;

                    let mut amount = String::new();
                    io::stdin().read_line(&mut amount)?;

                    let amount: u128 = match amount.trim().replace(',', ".").parse() {
                        Ok(value) if value > 0 => value,
                        _ => {
                            println!("Vigane summa!");
                            continue;
                        }
                    };

                    user.balance += amount;

                    let json = serde_json::to_string_pretty(&user)
                        .map_err(io::Error::other)?;

                    fs::write(account, json)?;

                    println!("Raha edukalt lisatud!");
                }

                "3" => {
                    print!("Sisesta väljastatav summa >> ");
                    io::stdout().flush()?;

                    let mut amount = String::new();
                    io::stdin().read_line(&mut amount)?;

                    let amount: u128 = match amount.trim().replace(',', ".").parse() {
                        Ok(value) if value > 0 => value,
                        _ => {
                            println!("Vigane summa!");
                            continue;
                        }
                    };

                    if amount > user.balance {
                        println!("Kontol pole piisavalt raha!");
                        continue;
                    }

                    user.balance -= amount;

                    let json = serde_json::to_string_pretty(&user)
                        .map_err(io::Error::other)?;

                    fs::write(account, json)?;

                    println!(
                        "Raha edukalt väljastatud!" 
                    );
                }

                "4" => {
                    println!("Oled edukalt välja logitud!");
                    break;
                }

                _ => {
                    println!("Vale valik!");
                }
            }
        }

        Ok(())
    }

}