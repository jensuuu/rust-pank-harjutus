mod menu;
mod bank;
mod vahendid;
mod poodlemine;
use std::io;


fn main() -> io::Result<()> {

    menu::Menu::show_menu()?;

    //bank::User::create()?;

    Ok(())

}