/* File: mterm.rs
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

//! Module for the manager terminal.
use crate::db::{DB, LocationInfo, PersonInfo};
use std::io::{self, Write};

pub fn run_man_term(db: &DB) {
    let mut quit = false;
    while !quit {
        let choice = display_options();

        match choice.as_str() {
            "1" => add_member_ui(db),
            "2" => println!("todo"),
            "3" => println!("todo"),
            "4" => println!("todo"),
            "5" => println!("todo"),
            "6" => quit = true,
            _ => println!("Invalid input."),
        }
    }
}

//displays options for the manager terminal
fn display_options() -> String {
    println!("----MANAGER TERMINAL----");
    println!("1. Add new member");
    println!("2. Add new provider");
    println!("3. Remove member");
    println!("4. remove provider");
    println!("5. Request report");
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
fn add_member_ui(db: &DB) {
    println!("----Add New Member----");
    let name = read_line("Name:");
    let id_str = read_line("9 digit member ID:");
    let address = read_line("Street address:");
    let city = read_line("City name:");
    let state = read_line("State (2 letters):");
    let zip_str = read_line("5 digit zip:");
    let email = read_line("Email:");

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
    let state_upper = state.trim().to_uppercase();
    let mut chars = state_upper.chars();
    let state0 = chars.next().unwrap_or(' ');
    let state1 = chars.next().unwrap_or(' ');
    let state_arr = [state0, state1];

    let location = match LocationInfo::new(&address, &city, &state_arr, zipcode)
    {
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
    match db.add_member(&person) {
        Ok(()) => println!("Member successfully added."),
        Err(e) => eprintln!("Error adding member: {e}"),
    }
}

fn read_line(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");
    buf.trim().to_string()
}
