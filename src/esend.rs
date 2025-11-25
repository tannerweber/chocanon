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
use std::fs::{File, create_dir_all};

use std::io::prelude::*;

#[non_exhaustive]
struct EmailPath;
impl EmailPath {
    pub const MEMBER: &str = "./emails/member";
    pub const PROVIDER: &str = "./emails/provider";
    pub const MANAGER: &str = "./emails/manager";
}

// creates a directory and doesn't error if
// the directory already exists
fn ensure_email_dir() {
    let _ = create_dir_all(EmailPath::MEMBER);
    let _ = create_dir_all(EmailPath::PROVIDER);
    let _ = create_dir_all(EmailPath::MANAGER);
}

pub fn send_provider_report(
    to: &str,
    from: &str, //always from chocanon
    subject: &str,
    body: &str,
) {
    let _ = send_email(to, from, subject, body, EmailPath::PROVIDER);
}

pub fn send_member_report(
    to: &str,
    from: &str, //always from chocanon
    subject: &str,
    body: &str,
) {
    let _ = send_email(to, from, subject, body, EmailPath::MEMBER);
}

pub fn send_manager_report(
    to: &str,
    from: &str, //always from chocanon
    subject: &str,
    body: &str,
) {
    let _ = send_email(to, from, subject, body, EmailPath::MANAGER);
}

pub fn send_provider_directory(
    to: &str,
    from: &str, //always from chocanon
    subject: &str,
    body: &str,
) {
    let _ = send_email(to, from, subject, body, EmailPath::PROVIDER);
}

//path: &str, add this as an argument later
// remember to make private
pub fn send_email(
    to: &str,
    from: &str, //always from chocanon
    subject: &str,
    body: &str,
    path: &str,
) -> std::io::Result<()> {
    ensure_email_dir();

    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
    let file_name = format!("{}/{}_{}.txt", path, to, timestamp);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_email() {
        match send_email(
            "User 108",
            "Chocanon Services",
            "Regarding Follow up meeting",
            "This is te body of the email.",
            EmailPath::MEMBER,
        ) {
            Ok(_) => (),
            Err(err) => panic!("ERROR {}", err),
        }
    }

    #[test]
    fn test_send_member_report() {}

    #[test]
    fn test_send_provider_report() {}

    #[test]
    fn test_send_manager_report() {}

    #[test]
    fn test_send_provider_directory() {}
}
