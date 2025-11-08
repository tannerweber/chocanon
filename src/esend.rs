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

use std::fs::{create_dir, File};
use std::io::prelude::*;

const PATH: &str = "./emails";

/// Writes an email as a file.
pub fn send_email(
        to: & str,
        from: & str,
        subject: & str,
        body: & str,
    ) -> std::io::Result<()> {

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
