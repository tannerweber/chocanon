/* File: esend.rs
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

//! Module for sending emails by writing them as files.
//use std::fs::create_dir_all;
use std::fs::{File, create_dir_all};
use std::io::prelude::*;

const PATH: &str = "./emails/member";

// creates a directory and doesn't error if
// the directory already exists
fn ensure_email_dir() {
    let _ = create_dir_all(PATH);
}

//send_provider_reports
//send_provider_manager
//send_member_report
//send_provider_directory

//path: &str, add this as an argument later
// remember to make private
pub fn send_email(
    to: &str,
    from: &str, //always from chocanon
    subject: &str,
    body: &str,
) -> std::io::Result<()> {
    ensure_email_dir();

    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
    let file_name = format!("{}/{}_{}.txt", PATH, to, timestamp);

    let mut file = File::create(&file_name)?;

    writeln!(file, "To: {}", to)?;
    writeln!(file, "From: {}", from)?;
    writeln!(file, "Subject: {}", subject)?;
    writeln!(
        file,
        "Date: {}\n",
        chrono::Local::now().format("%Y-%m-%d %H-%M-%S")
    )?;

    writeln!(file, "{}\n", body)?;

    Ok(())
}

/*
/// Writes an email as a file.
pub fn send_email(
    to: &str,
    from: &str,
    subject: &str,
    body: &str,
) -> std::io::Result<()> {
    //specify path
    match create_dir(PATH) {
        Ok(_) => (),
        Err(_) => (),
    }

    let date = chrono::Local::now().date_naive().to_string();
    let file_name = format!("{}_{}.txt", to, date);
    let mut file = File::create(&file_name)?;

    file.write_all(to.as_bytes())?;
    file.write_all(from.as_bytes())?;
    file.write_all(subject.as_bytes())?;
    file.write_all(body.as_bytes())?;
    Ok(())
}
*/

/*	esend::send_email(
        "User 108",
        "Chocanon Services",
        "Regarding Follow up meeting",
        "Lorem Ipsum however it goes"
    ).expect("This email failed to send");

    println!("Email testing concluded");
*/

#[cfg(test)]
mod tests {
    //super allows to use the entire file acting as if
    //it is in the scope of the functions that we want to call
    //at least that's how I interpret it.
    use super::*;
    #[test]
    fn test_send_email() {
        send_email(
            "User 108",
            "Chocanon Services",
            "Regarding Follow up meeting",
            "Lorem Ipsum however it goes",
        )
        .expect("This email failed to send");

        println!("Email testing concluded");
    }
}
