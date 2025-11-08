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

use std::fs::File;
use std::io::prelude::*;

pub struct Email {
    to: String,
    from: String,
    subject: String,
    body: String,
}

impl Email {
    pub fn new(
        to: & str,
        from: & str,
        subject: & str,
        body: & str
    ) -> Result<Self, ()> {

        Ok(Email {
            to: "Timmy".to_string(),
            from: "Chocanon".to_string(),
            subject: "Member report".to_string(),
            body: "Stuff".to_string(),
        })
    }
}

pub fn send_email(email: & Email) -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}
