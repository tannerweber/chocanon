/* File: main.rs
 *
 * Authors:
 * - Tanner Weber, tannerw@pdx.edu
 * - Cristian Hernandez, cristhe@pdx.edu
 * - Jethro Fernandez, jethrof@pdx.edu
 * - Torin Costales, tcostal2@pdx.edu
 * - Miles Turoczy, turoczy@pdx.edu
 *
 * Portland State University
 * Dates: October 29 to December 5
 * Course: CS 314, Fall 2025
 * Instructor: Christopher Gilmore
 */

//! Chocaholics Anonymous project.

use chocanon::db::{Consultation, DB, LocationInfo, PersonInfo};
use chocanon::{mterm, pterm};
use std::io::{self, Write};

#[derive(PartialEq, Debug)]
enum MenuOption {
    Quit,
    ProviderTerminal,
    ManagerTerminal,
    PopulateDatabase,
}

const DB_PATH: &str = "./chocanon.db3";

fn main() {
    let db = match DB::new(DB_PATH) {
        Ok(db) => db,
        Err(err) => panic!("Error: {}", err),
    };
    loop {
        print_menu();
        let option = get_valid_user_input();
        match option {
            MenuOption::Quit => {
                println!("Quitting");
                break;
            }
            MenuOption::ProviderTerminal => {
                println!("Chose provider terminal");
                pterm::run(&db);
            }
            MenuOption::ManagerTerminal => {
                println!("Chose manager terminal");
                mterm::run_man_term(&db);
            }
            MenuOption::PopulateDatabase => {
                println!("Populating database");
                populate_database(&db);
            }
        }
    }
}

fn print_menu() {
    print!(
        "\n---ChocAn Start Menu---
Enter a number corresponding to the option
( 0 ) Quit
( 1 ) Provider Terminal
( 2 ) Manager Terminal
( 3 ) Populate database with some values
Choice: "
    );
    io::stdout().flush().expect("Failed to flush stdout");
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
        return MenuOption::ManagerTerminal;
    }
    if number == 3 {
        MenuOption::PopulateDatabase
    } else {
        MenuOption::Quit
    }
}

fn populate_database(db: &DB) {
    fn create_a_unique_person(name: &str, id: u32) -> PersonInfo {
        let location =
            LocationInfo::new("1234 Main st", "Portland", "OR", 56789).unwrap();
        let email = format!("{}@pdx.edu", name);
        PersonInfo::new(name, id, &location, &email).unwrap()
    }

    /// Creates a consultation with a date of yesterday.
    fn create_a_unique_consultation(
        member_id: u32,
        provider_id: u32,
    ) -> Consultation {
        let date = (chrono::Local::now() - chrono::Duration::days(1))
            .format("%m-%d-%Y")
            .to_string();
        let date_time = (chrono::Local::now() - chrono::Duration::days(1))
            .format("%m-%d-%Y %H:%M:%S")
            .to_string();
        Consultation::new(
            &date_time,
            &date,
            provider_id,
            member_id,
            123456,
            "This is a comment created by create_a_unique_consultation",
        )
        .unwrap()
    }

    let _ = db.add_service(123456, "ServiceName123456", 99.99);
    let _ = db.add_member(&create_a_unique_person("MemberName1", 1));
    let _ = db.add_member(&create_a_unique_person("MemberName2", 2));
    let _ = db.add_member(&create_a_unique_person("MemberName3", 3));
    let _ = db.add_member(&create_a_unique_person("MemberName4", 4));
    let _ = db.add_provider(&create_a_unique_person("ProviderName1", 1));
    let _ = db.add_provider(&create_a_unique_person("ProviderName2", 2));
    let _ = db.add_consultation_record(&create_a_unique_consultation(1, 1));
    let _ = db.add_consultation_record(&create_a_unique_consultation(2, 1));
    let _ = db.add_consultation_record(&create_a_unique_consultation(2, 1));
    let _ = db.add_consultation_record(&create_a_unique_consultation(3, 1));
    let _ = db.add_consultation_record(&create_a_unique_consultation(3, 1));
    let _ = db.add_consultation_record(&create_a_unique_consultation(3, 1));
}
