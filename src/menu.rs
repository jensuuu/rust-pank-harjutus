use std::io::{self, Write};

use crate::bank;

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


            println!("Vali õige tegevus >>");

            io::stdout().flush()?;

            // loeb kasutaja sisendit
            io::stdin().read_line(&mut user_option)?; 

            // kontrollib sisendi määrust kas on või ei
            let user_option: u8 = match user_option.trim().parse() {
                Ok(option) => option,
                Err(_) => {
                    println!("Sisestage number!");
                    continue;
                }
            };

        match user_option {
                1 => {
                    bank::User::create()?;
                },
                2 => {
                    println!("Sisse logimine");
                },
                3 => {
                    println!("Head aega!");
                    break Ok(());
                },
                _ => {
                    println!("Pole võimalik!");
                }
            }



            println!("TERVIST PANGANDUSE POOLT! {}", user_option);


            Ok(());
        }
       

    }
}