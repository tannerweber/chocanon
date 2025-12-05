/* File: pterm.rs
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

//! Module for the provider terminal.

use crate::db::{Consultation, DB};
use std::io::{self, Write};

#[derive(Debug)]
enum MenuOption {
    AddConsultationRecord,
    GetProviderDirectory,
    ValidateMember,
    Quit,
}

/// Runs the provider terminal at the commmand line with user input and output.
///
/// # Arguments
///
/// * `db` - The database to interact with.
pub fn run(db: &DB) {
    let mut quit: bool = false;

    if !validate_provider(db) {
        println!("Invalid provider id.");
        println!(
            "Ensure that the provider has been added from the manager terminal."
        );
        return;
    }

    while !quit {
        print_menu_options();
        let option = get_menu_option();

        match option {
            MenuOption::Quit => {
                println!("Exiting provider terminal...");
                quit = true;
            }
            MenuOption::ValidateMember => {
                if validate_member(db) {
                    println!("Validated");
                    continue;
                } else {
                    println!("Invalid Number");
                    continue;
                }
            }
            MenuOption::AddConsultationRecord => {
                if !validate_member(db) {
                    println!("Invalid Number");
                    continue;
                } else {
                    println!("Validated");
                }
                print!("\n---Add Consultation Record---\n");
                let curr_date = chrono::Local::now()
                    .format("%m-%d-%Y %H:%M:%S")
                    .to_string();
                let service_date = input("Service date (MM-DD-YYYY): ");
                let provider_id: u32 =
                    input("Provider ID: ").parse().unwrap_or(0);
                let member_id: u32 = input("Member ID: ").parse().unwrap();
                let service_code: u32 = get_service_code(db);
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

fn print_menu_options() {
    println!("---Provider Terminal---");
    println!("( 0 ) Quit");
    println!("( 1 ) Add Consultation Record");
    println!("( 2 ) Get Provider Directory");
    println!("( 3 ) Validate Member");
    print!("Choice: ");
    io::stdout().flush().unwrap();
}

fn get_menu_option() -> MenuOption {
    loop {
        let mut input = String::new();

        if let Err(e) = io::stdin().read_line(&mut input) {
            println!("Input error: {e}. Try again.");
            continue;
        }

        match input.trim() {
            "0" => return MenuOption::Quit,
            "1" => return MenuOption::AddConsultationRecord,
            "2" => return MenuOption::GetProviderDirectory,
            "3" => return MenuOption::ValidateMember,
            _ => {
                println!("Invalid option. Please enter 0, 1, 2, or 3.");
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

fn validate_provider(db: &DB) -> bool {
    println!("Enter your provider number/id: ");
    io::stdout().flush().unwrap();

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

    match db.is_valid_provider_id(number) {
        Ok(is_valid_user) => is_valid_user,
        Err(err) => {
            println!("Error validating id: {}", err);
            false
        }
    }
}

fn validate_member(db: &DB) -> bool {
    println!("Enter the member number/id: ");
    io::stdout().flush().unwrap();

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

    match db.is_valid_member_id(number) {
        Ok(is_valid_user) => is_valid_user,
        Err(err) => {
            println!("Error validating id: {}", err);
            false
        }
    }
}

fn get_service_code(db: &DB) -> u32 {
    let id: u32;
    loop {
        let service_code: u32 = input("Service code: ").parse().unwrap();
        match db.is_valid_service_id(service_code) {
            Ok(valid) => {
                if valid {
                    match db.get_service_name(service_code) {
                        Ok(name) => {
                            println!("Service Name: {}", name);
                            if input("Is the name correct (y/n): ") == "y" {
                                id = service_code;
                                break;
                            }
                        }
                        Err(err) => {
                            println!("Error for getting service code: {}", err)
                        }
                    }
                }
            }
            Err(err) => println!("Error for verify service code: {}", err),
        }
    }
    id
}
