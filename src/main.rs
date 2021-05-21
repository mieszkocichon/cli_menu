
use std::io;
use std::io::*;

mod cli;
use crate::cli::menu::{menu};

fn main() {
    loop {
        initialize_menu();
    }
}

pub fn initialize_menu() {
    let mut foo = menu::Menu::new();
    foo.initialize();

    let input: String = get_input("Please type something...");
    foo.execute(&input.to_string());
}

pub fn get_input(prompt: &str) -> String{
    println!("{}",prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}