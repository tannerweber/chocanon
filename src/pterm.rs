/* File: pterm.rs
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

//! Module for the provider terminal.

use crate::db::DB;
use std::io::{self, Write};

enum MenuOption {
    AddConsultationRecord,
    GetProviderDirectory,
    Quit,
}

pub fn run(db: &DB) {
    let mut quit: bool = false;
    while quit != true {
        print_menu_options();
        let option = get_menu_option();

        match option {
            MenuOption::AddConsultationRecord => {
                // db.add_consultation_record();
            }
            MenuOption::GetProviderDirectory => {
                // db.get_provider_directory();
            }
            MenuOption::Quit => {
                println!("Exiting provider terminal...");
                quit = true;
            }
        }
        println!();
    }
}

// Print menu options
fn print_menu_options() {
    println!("---Provider Terminal---");
    println!("1. Add Consultation Record");
    println!("2. Get Provider Directory");
    println!("3. Quit");
    print!("Enter an option (1-3): ");
    io::stdout().flush().unwrap();
}

// Prompt user for menu option
fn get_menu_option() -> MenuOption {
    loop {
        let mut input = String::new();

        if let Err(e) = io::stdin().read_line(&mut input) {
            println!("Input error: {e}. Try again.");
            continue;
        }

        // Return menu option corresponding to user's choice
        match input.trim() {
            "1" => return MenuOption::AddConsultationRecord,
            "2" => return MenuOption::GetProviderDirectory,
            "3" => return MenuOption::Quit,
            _ => {
                println!("Invalid option. Please enter 1, 2, or 3.");
                print!("Enter an option (1-3): ");
                io::stdout().flush().unwrap();
            }
        }
    }
}
