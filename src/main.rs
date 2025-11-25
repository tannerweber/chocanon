/* File: main.rs
 *
 * Authors:
 * - Tanner Weber, tannerw@pdx.edu
 * - Cristian Hernandez, cristhe@pdx.edu
 * - Jethro Fernandez, jethrof@pdx.edu
 * - Torin Costales, turoczy@pdx.edu
 * - Miles Turoczy, tcostal2@pdx.edu
 *
 * Portland State University
 * Dates: October 29 to December 5
 * Course: CS 314, Fall 2025
 * Instructor: Christopher Gilmore
 */

//! Chocaholics Anonymous project.

use chocanon::db::DB;
use chocanon::{mterm, pterm};
use std::error::Error;
use std::io::{self, Write};

enum MenuOption {
    Quit,
    ProviderTerminal,
    ManagerTerminal,
}

const DB_PATH: &str = "./chocanon.db3";

fn main() {
    let _ = run();
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let db = match DB::new(DB_PATH) {
        Ok(db) => db,
        Err(err) => panic!("Error: {}", err),
    };
    print!(
        "---ChocAn Start Menu---
Enter a number corresponding to the option
( 0 ) Quit
( 1 ) Provider Terminal
( 2 ) Manager Terminal
Choice: "
    );
    io::stdout().flush().expect("Failed to flush stdout");
    let option = get_valid_user_input();
    match option {
        MenuOption::Quit => println!("Quitting"),
        MenuOption::ProviderTerminal => {
            println!("Chose provider terminal");
            pterm::run(&db);
        }
        MenuOption::ManagerTerminal => {
            println!("Chose manager terminal");
            // mterm::run_man_term(&db);
        }
    }
    Ok(())
}

fn get_valid_user_input() -> MenuOption {
    let mut valid_input = false;
    let mut number: u32 = 0;

    while !valid_input {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        match input.parse::<u32>() {
            Ok(n) => {
                valid_input = true;
                number = n;
            }
            Err(_) => println!("Invalid input. Try again"),
        }
    }

    if number == 0 {
        return MenuOption::Quit;
    }
    if number == 1 {
        return MenuOption::ProviderTerminal;
    }
    if number == 2 {
        MenuOption::ManagerTerminal
    } else {
        MenuOption::Quit
    }
}
