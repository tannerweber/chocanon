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

use crate::db::{Consultation, DB};
use std::io::{self, Write};

enum MenuOption {
    AddConsultationRecord,
    GetProviderDirectory,
    Quit,
}

pub fn run(db: &DB) {
    let mut quit: bool = false;
    while !quit {
        print_menu_options();
        let option = get_menu_option();

        match option {
            MenuOption::Quit => {
                println!("Exiting provider terminal...");
                quit = true;
            }
            MenuOption::AddConsultationRecord => {
                // Add code here to get a consultation record first, then pass it into the add_consultation_record function

                print!("\n---Add Consultation Record---\n");
                let curr_date = chrono::Local::now()
                    .format("%m-%d-%Y %H:%M:%S")
                    .to_string();
                let service_date = input("Service date (MM-DD-YYYY): ");
                let provider_id: u32 =
                    input("Provider ID: ").parse().unwrap_or(0);
                let member_id: u32 = input("Member ID: ").parse().unwrap();
                let service_code: u32 =
                    input("Service code: ").parse().unwrap();
                let comments = input("Comments: ");

                let consul = match Consultation::new(
                    curr_date.as_str(),
                    service_date.as_str(),
                    provider_id,
                    member_id,
                    service_code,
                    comments.as_str(),
                ) {
                    Ok(c) => c,
                    Err(e) => {
                        println!("Error creating consultation: {}", e);
                        return;
                    }
                };

                match db.add_consultation_record(&consul) {
                    Ok(_) => {
                        println!("Consultation record added successfully.")
                    }
                    Err(e) => {
                        println!("Failed to add consultation record: {}", e)
                    }
                }
            }
            MenuOption::GetProviderDirectory => {
                println!();
                print!("Please enter your email address: ");
                io::stdout().flush().unwrap();

                let mut email = String::new();
                io::stdin()
                    .read_line(&mut email)
                    .expect("Failed to read input");

                let email = email.trim();
                match db.send_provider_directory(email) {
                    Ok(_) => println!("Retrieving Provider Directory."),
                    Err(e) => {
                        println!("Failed to send Provider Directory: {}", e)
                    }
                }
            }
        }
        println!();
    }
}

// Print menu options
fn print_menu_options() {
    println!("---Provider Terminal---");
    println!("( 0 ) Quit");
    println!("( 1 ) Add Consultation Record");
    println!("( 2 ) Get Provider Directory");
    print!("Choice: ");
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
            "0" => return MenuOption::Quit,
            "1" => return MenuOption::AddConsultationRecord,
            "2" => return MenuOption::GetProviderDirectory,
            _ => {
                println!("Invalid option. Please enter 0, 1, or 2.");
                print!("Choice: ");
                io::stdout().flush().unwrap();
            }
        }
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}
