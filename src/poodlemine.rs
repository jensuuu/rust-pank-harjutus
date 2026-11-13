
use std::collections::HashMap;
use std::io::{self, Write};
use std::fs;

use crate::bank::User;

pub struct Store {
    pub items: HashMap<String, u128>,
}

impl Store {
    // Loo pood ja tooted
    pub fn new() -> Self {
        let mut items = HashMap::new();

        items.insert("Vesi".to_string(), 25);
        items.insert("Leib".to_string(), 50);
        items.insert("Piim".to_string(), 40);

        Self { items }
    }

    // Osta toode
    pub fn buy_items(
        &self,
        user: &mut User,
        item: &str,
    ) -> Result<(), String> {
        let price = self.items.get(item)
            .ok_or("Toodet ei leitud!")?;

        if user.balance < *price {
            return Err("Sul pole piisavalt raha!".to_string());
        }

        user.balance -= *price;
        user.basket.push(item.to_string());

        println!("Eemaldati raha {}: {} EUR", item, price);

        Ok(())
    }

    // Kuva poe tooted
    pub fn view_items(&self) {
        println!("========== POE VALIK ==========");

        for (item, price) in &self.items {
            println!("{} - {} EUR", item, price);
        }
    }

    // Kuva ostukorv
    pub fn view_basket(&self, user: &User) {
        println!("========== SINU KORV ==========");

        if user.basket.is_empty() {
            println!("Sinu korv on tühi.");
            return;
        }

        for (index, item) in user.basket.iter().enumerate() {
            println!("{}. {}", index + 1, item);
        }
    }

    // Tagasta kõik tooted
    pub fn get_items(&self) -> &HashMap<String, u128> {
        &self.items
    }

    // Leia toote hind
    pub fn get_price(&self, item: &str) -> Result<u128, String> {
        self.items
            .get(item)
            .copied()
            .ok_or_else(|| "Viga summa leidmisel".to_string())
    }

    // Salvesta kasutaja konto
    fn save_user(user: &User, account: &str) -> io::Result<()> {
        let json = serde_json::to_string_pretty(user)
            .map_err(io::Error::other)?;

        fs::write(account, json)?;

        Ok(())
    }

    // Poe menüü
    pub fn store_menu(
        &self,
        user: &mut User,
        account: &str,
    ) -> io::Result<()> {

        loop {
            println!("\n==============================");
            println!("          POE MENÜÜ");
            println!("==============================");
            println!("Kasutaja: {}", user.username);
            println!("Korv: {}", user.basket.join(", "));
            println!("Saldo: {:.2} EUR", user.balance);
            println!("==============================");
            println!("1. Vaata poe tooteid");
            println!("2. Osta toode");
            println!("3. Vaata ostukorvi");
            println!("4. Tagasi pangakontole");
            println!("==============================");

            print!("Vali tegevus >> ");
            io::stdout().flush()?;

            let mut option = String::new();
            io::stdin().read_line(&mut option)?;

            match option.trim() {
                // Vaata tooteid
                "1" => {
                    self.view_items();
                }

                // Osta toode
                "2" => {
                    println!("\n========== OSTA TOODE ==========");

                    self.view_items();

                    print!("\nSisesta toote nimi >> ");
                    io::stdout().flush()?;

                    let mut item = String::new();
                    io::stdin().read_line(&mut item)?;

                    match self.buy_items(user, item.trim()) {
                        Ok(()) => {
                            Self::save_user(user, account)?;

                            println!("Ost edukalt sooritatud!");
                        }

                        Err(error) => {
                            println!("Viga: {}", error);
                        }
                    }
                }

                // Vaata ostukorvi
                "3" => {
                    self.view_basket(user);
                }

                // Tagasi
                "4" => {
                    Self::save_user(user, account)?;

                    println!("Lahkusid poest!");
                    break;
                }

                _ => {
                    println!("Vale valik!");
                }
            }

            println!("\nVajuta ENTER jätkamiseks...");
            let mut pause = String::new();
            io::stdin().read_line(&mut pause)?;
        }

        Ok(())
    }
}
