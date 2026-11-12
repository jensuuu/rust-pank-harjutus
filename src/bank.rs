use std::fs;
use std::io::{self, Write};
use rand::RngExt;

pub struct User; // klassi määrang

// i8    - Signed 8-bit integer (negatiivsed ja positiivsed täisarvud)
// i16   - Signed 16-bit integer
// i32   - Signed 32-bit integer
// i64   - Signed 64-bit integer
// i128  - Signed 128-bit integer
// isize - Signed integer, suurus sõltub arhitektuurist (32/64-bit)

// u8    - Unsigned 8-bit integer (ainult null ja positiivsed täisarvud)
// u16   - Unsigned 16-bit integer
// u32   - Unsigned 32-bit integer
// u64   - Unsigned 64-bit integer
// u128  - Unsigned 128-bit integer
// usize - Unsigned integer, suurus sõltub arhitektuurist (32/64-bit) 

// f32 - 32-bit floating-point number (komakohaga arv)
// f64 - 64-bit floating-point number (komakohaga arv)

// bool - Boolean (true / false)
// char - Üks Unicode märk
// str  - String slice (muutmatu tekstiviide)
// String - Dünaamiline muudetav tekst

impl User { // esindab klassi 

    pub fn create() -> io::Result<()> { // gives I/O error and info // klass funktsioon
        // Bank accounts directory
        let mut name = String::new();
        let mut password = String::new();

        let directory = "./accounts";

        // loob muutuvaks suvalised numbrid kuna mut (rng) võimaldab muutumist
        let mut rng = rand::rng();
        let account_number: u64 = rng.random_range(10000000..100000000);

        // loob kausta kui ei eksisteeri
        fs::create_dir_all(directory)?;


        println!("Sisesta nimi"); 
        io::stdout().flush()?;

        io::stdin().read_line(&mut name).expect("Vigane nime väli!"); // loeb kasutaja sisendit

        println!("Sisesta parool");
        io::stdout().flush()?;
        io::stdin().read_line(&mut password).expect("Vigane parooli väli!");


        // puhastab väljad
        let name = name.trim();
        let password = password.trim();

        // salvestab kasutaja andmed faili ja kasuta
        let acnum: u64 = account_number;
        let account_data = format!("{}/kasutaja-{}.txt", directory, acnum);

        let mut file = fs::File::create(&account_data)?;

        // andmete kirjutamine faili sisse
        writeln!(file, "NR: {}", acnum)?;
        writeln!(file, "Nimi: {}", name)?;
        writeln!(file, "Parool: {}", password)?;

    

        println!("Directory: {}", directory);
        println!("Account number: {}", account_number);
        println!("User input {}, {}", name, password);

        Ok(())
    }
}

