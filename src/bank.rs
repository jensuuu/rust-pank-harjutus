
use std::fs;
use std::io::{self, Write};
use rand::RngExt;
use serde::{Serialize, Deserialize};

use crate::menu;

#[derive(Serialize, Deserialize)]
pub struct User { // klassi määrang
    pub account_number: u64,
    pub username: String,
    pub password: String,
    pub balance: u128,
}

impl User { // esindab klassi
    const DEBUG: bool = true; // debug režiim

    // parem printida andmeid kui println!() ja println!() on liiga pikk
    // info_type on valikuline
    pub fn show(msg: &str, info: &str) {
        if info.is_empty() {
            println!("{}", msg);
            return;
        }

        println!("{}: {}", msg, info);
    }

    pub fn create() -> io::Result<()> {
        // kasutajale määratud sõned ja muutujad
        let mut tunnus = String::new();
        let mut password = String::new();

        let directory = "./accounts";

        // loob muutuvaks suvalised numbrid kuna mut (rng) sõne võimaldab muutumist
        let mut rng = rand::rng();
        let account_number: u64 = rng.random_range(10000000..100000000);

        // loob kasutaja kausta kui ei eksisteeri
        fs::create_dir_all(directory)?;

        Self::show("Sisesta kasutajatunnus", "");
        io::stdout().flush()?;

        io::stdin().read_line(&mut tunnus)?;

        Self::show("Sisesta parool", "");
        io::stdout().flush()?;

        io::stdin().read_line(&mut password)?;

        // puhastab sõne väljad
        let tunnus = tunnus.trim();
        let password = password.trim();

        if tunnus.is_empty() || password.is_empty() {
            Self::show("Palun täida kõik väljad!", "ERROR");
            return Ok(());
        }

        // salvestab kasutaja andmed faili
        let acnum: u64 = account_number;

        let account_data = format!("{}/kasutaja-{}.json", directory, tunnus);






        // kontrollib, kas konto on juba olemas
        if fs::metadata(&account_data).is_ok() {
            Self::show("Selline kasutaja on juba olemas!", "ERROR");
            return Ok(());
        }

        let user = User {
            account_number: acnum,
            username: tunnus.to_string(),
            password: password.to_string(),
            balance: 0,
        };

        // andmete kirjutamine faili sisse
        let json = serde_json::to_string_pretty(&user)
            .map_err(io::Error::other)?;

        fs::write(&account_data, json)?;

        if Self::DEBUG {
            Self::show("Directory", directory);
            Self::show("Account number", &account_number.to_string());
        }

        Self::show("Konto edukalt loodud!", "SUCCESS");

        Ok(())
    }

    pub fn login() -> io::Result<()> {
        // kasutajale määratud väljad ja muutujad
        let mut tunnus = String::new();
        let mut password = String::new();

        println!("==============================");
        println!("       PANGA SISSELOGIMINE");
        println!("==============================");

        // küsib kasutajalt konto tunnust ja parooli
        print!("Sisesta konto tunnus: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut tunnus)?;

        print!("Sisesta konto parool: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut password)?;

        // puhastab väljad
        let actunnus = tunnus.trim();
        let acpw = password.trim();

        let directory = "./accounts";

        // kontrollib, kas väljad on täidetud
        if actunnus.is_empty() || acpw.is_empty() {
            println!("Palun täida kõik väljad");
            return Ok(());
        }

        // määrab faili nime ja asukoha
        let account: String = format!(
            "{}/kasutaja-{}.json",
            directory,
            actunnus
        );
        // loeb faili sisu
        let account_data = match fs::read_to_string(&account) {
            Ok(data) => data,
            Err(_) => {
                println!("Konto ei leitud!");
                return Ok(());
            }
        };

        // loeb JSON-failist kasutaja andmed
        let account_data: User = match serde_json::from_str(&account_data) {
            Ok(data) => data,
            Err(_) => {
                println!("Konto andmete lugemine ebaõnnestus!");
                return Ok(());
            }
        };

        // võrdleb sisestatud parooli salvestatud parooliga
        let stored_password = account_data.password.trim();
       

        match stored_password {
            saved_password if saved_password == acpw => {
                println!("Sisselogimine õnnestus!");

                println!("Tere tulemast, {}!", account_data.username);

                if Self::DEBUG {
                    Self::show(
                        "Account number",
                        &account_data.account_number.to_string()
                    );

                    Self::show(
                        "Balance",
                        &account_data.balance.to_string()
                    );
                }
                menu::Menu::account_menu(account_data, &account)?;
            }

            _ => {
                println!("Vale kasutajatunnus või parool!");
            }
        }

        Ok(())
    }
}
