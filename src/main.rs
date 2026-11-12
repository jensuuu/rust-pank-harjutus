mod menu;
mod bank;


use std::io;


fn main() -> io::Result<()> {

    menu::Menu::show_menu()?;

    //bank::User::create()?;

    Ok(())

}