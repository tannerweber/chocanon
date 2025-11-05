//! File: app.rs
//!
//! Authors:
//! - Tanner Weber, tannerw@pdx.edu
//! - Cristian Hernandez, cristhe@pdx.edu
//! - Jethro Fernandez, jethrof@pdx.edu
//! - Torin Costales, turoczy@pdx.edu
//! - Miles Turoczy, tcostal2@pdx.edu
//!
//! Portland State University
//! Dates: October 29 to December 5
//! Course: CS 314, Fall 2025
//! Instructor: Christopher Gilmore

use std::error::Error;
use crate::db::{DB};

pub fn run() -> Result<(), Box<dyn Error>> {
    let db: DB = DB::new();
    Ok(())
}
