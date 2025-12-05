/* File: mterm.rs
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

//! Module for the manager terminal.
use crate::db::{DB, LocationInfo, PersonInfo};
use std::io::{self, Write};

//driver function that initializes the manager terminal
//param DB - database to be passed to the manager terminal
pub fn run_man_term(db: &DB) {
    let mut quit = false;
    while !quit {
        let choice = display_options();

        match choice.as_str() {
            "1" => add_person_ui(db),
            "2" => remove_person(db),
            "3" => match db.send_member_reports() {
                Ok(()) => println!("Member reports sent."),
                Err(e) => eprintln!("Error sending member reports: {e}"),
            },
            "4" => match db.send_provider_reports() {
                Ok(()) => println!("Provider reports sent."),
                Err(e) => eprintln!("Error sending provider reports: {e}"),
            },
            "5" => match db.send_manager_report() {
                Ok(()) => println!("Manager report sent."),
                Err(e) => eprintln!("Error sending manager report: {e}"),
            },
            "6" => quit = true,
            _ => println!("Invalid input."),
        }
    }
}

//displays options for the manager terminal
//returns string
fn display_options() -> String {
    println!("----MANAGER TERMINAL----");
    println!("1. Add new person");
    println!("2. Remove person");
    println!("3. Send out member reports");
    println!("4. Send out provider reports");
    println!("5. Request manager report");
    println!("6. Quit");
    read_choice()
}

//reads the choice of the user as string
fn read_choice() -> String {
    use std::io::{self, Write};

    print!("Select an option: ");
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Invalid input");
    buf.trim().to_string()
}

//adds member to the database
//param DB - database to add the member too
fn add_person_ui(db: &DB) {
    println!("----Add New Person----");
    let name = read_line("Name: ");
    let id_str = read_line("9 digit ID: ");
    let address = read_line("Street address: ");
    let city = read_line("City name: ");
    let state = read_line("State (2 letters): ");
    let zip_str = read_line("5 digit zip: ");
    let email = read_line("Email: ");

    let id: u32 = match id_str.parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Invalid id, enter valid 9 digit id number");
            return;
        }
    };

    let zipcode: u32 = match zip_str.parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Invalid zip, enter a valid 5 digit number");
            return;
        }
    };

    //set state input to an array of 2 uppercase chars
    if state.chars().count() != 2 {
        eprintln!("State must be 2 characters.");
        return;
    }
    let state_upper = state.trim().to_uppercase();

    let location =
        match LocationInfo::new(&address, &city, &state_upper, zipcode) {
            Ok(loc) => loc,
            Err(msg) => {
                eprintln!("Location error {msg}");
                return;
            }
        };

    let person = match PersonInfo::new(&name, id, &location, &email) {
        Ok(p) => p,
        Err(msg) => {
            eprintln!("Person error {msg}");
            return;
        }
    };

    //determine if user is adding member or provider
    let person_type = read_line("Is this person a provider? (y/n): ");
    if person_type.to_lowercase().starts_with('y') {
        match db.add_provider(&person) {
            Ok(_) => println!("Provider was successfully added."),
            Err(e) => eprintln!("Error adding provider: {e}"),
        }
    } else {
        match db.add_member(&person) {
            Ok(_) => println!("Member was successfully added."),
            Err(e) => eprintln!("Error adding member: {e}"),
        }
    }
}
//removes member based off of the member id
fn remove_person(db: &DB) {
    let person_type = read_line("Is this person a provider? (y/n)");
    if person_type.to_lowercase().starts_with('y') {
        let id_str: String = read_line("Enter the provider ID to be removed: ");
        let id: u32 = match id_str.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("ID is invalid, please enter valid 9 digit number.");
                return;
            }
        };
        match db.remove_provider(id) {
            Ok(()) => println!("Provider removed successfully."),
            Err(e) => eprintln!("Error removing provider: {e}"),
        }
    } else {
        let id_str: String = read_line("Enter the member ID to remove: ");
        let id: u32 = match id_str.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("ID is invalid, please enter valid 9 digit number.");
                return;
            }
        };
        match db.remove_member(id) {
            Ok(()) => println!("Member removed successfully."),
            Err(e) => eprintln!("Error removing Member: {e}"),
        }
    }
}

//helper function to read line from user input
fn read_line(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");
    buf.trim().to_string()
}
