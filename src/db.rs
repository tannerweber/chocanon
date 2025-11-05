//! File: db.rs
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

use rusqlite::{Connection};

const MAX_NAME_SIZE: u32 = 25;
const MAX_MEMBER_NUMBER: u32 = 999999999; // 9 Digits
const MAX_ADDRESS_SIZE: u32 = 25;
const MAX_CITY_SIZE: u32 = 14;
const STATE_SIZE: usize = 2;
const MAX_ZIPCODE: u32 = 99999; // 5 Digits

#[derive(Debug)]
pub struct DB {
    conn: Connection,
}

impl DB {
    pub fn new() -> Self {
        let conn: Connection;
        match Connection::open_in_memory() {
            Ok(c) => conn = c,
            Err(_) => panic!("Failed to connect to database"),
        }
        match conn.execute(
            "CREATE TABLE members (
                id          INTEGER PRIMARY KEY,
                name        VARCHAR(25) NOT NULL,
                location    BLOB
            )",
            (), // empty list of parameters.
        ) {
            Ok(_) => (),
            Err(_) => panic!("Failed to create members table"),
        }
        match conn.execute(
            "CREATE TABLE providers (
                id          INTEGER PRIMARY KEY,
                name        VARCHAR(25) NOT NULL,
                location    BLOB
            )",
            (), // empty list of parameters.
        ) {
            Ok(_) => (),
            Err(_) => panic!("Failed to create providers table"),
        }
        match conn.execute(
            "CREATE TABLE consultations (
                id          INTEGER PRIMARY KEY,
                name        VARCHAR(25) NOT NULL,
                location    BLOB
            )",
            (), // empty list of parameters.
        ) {
            Ok(_) => (),
            Err(_) => panic!("Failed to create consultations table"),
        }
        DB { conn: conn }
    }

    /*
    fn add_member(&self, member: & PersonInfo) -> rusqlite::Result<()> {
        self.conn.execute(
            "INSERT INTO members (id, name, location) VALUES (?1, ?2, ?3)",
            (member.number, &member.name, &member.location),
        )?;
        Ok(())
    }
    */

    fn get_member_report() {

    }

    fn send_provider_report() {

    }

    pub fn send_provider_reports() {

    }

    fn get_provider_report() {

    }
}

#[derive(Debug)]
pub struct PersonInfo {
    name: String,
    number: u32,
    location: LocationInfo,
}

impl PersonInfo {
    pub fn new(
        name: & str,
        number: u32,
        location: & LocationInfo
    ) -> Result<Self, String> {

        if name.chars().count() > MAX_NAME_SIZE.try_into().unwrap() {
            return Err(format!(
                "name must be less than or equal to {} characters: {}",
                MAX_NAME_SIZE, name
            ));
        }
        if number > MAX_MEMBER_NUMBER {
            return Err(format!(
                "number must be less than or equal to {}: {}",
                MAX_MEMBER_NUMBER, number
            ));
        }
        Ok(PersonInfo {
            name: name.to_string(),
            number: number,
            location: location.clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct LocationInfo {
    address: String,
    city: String,
    state: [char; STATE_SIZE],
    zipcode: u32,
}

impl LocationInfo {
    pub fn new(
        address: & str,
        city: & str,
        state: & [char; STATE_SIZE],
        zipcode: u32
    ) -> Result<Self, String> {
        if address.chars().count() > MAX_ADDRESS_SIZE.try_into().unwrap() {
            return Err(format!(
                "address must be less than or equal to {} characters: {}",
                MAX_ADDRESS_SIZE, address
            ));
        }
        if city.chars().count() > MAX_CITY_SIZE.try_into().unwrap() {
            return Err(format!(
                "city must be less than or equal to {} characters: {}",
                MAX_CITY_SIZE, city
            ));
        }
        if zipcode > MAX_ZIPCODE {
            return Err(format!(
                "zipcode must be less than or equal to {}: {}",
                MAX_ZIPCODE, zipcode
            ));
        }
        Ok(LocationInfo {
            address: address.to_string(),
            city: city.to_string(),
            state: state.clone(),
            zipcode: zipcode.clone(),
        })
    }
}

/*
impl rusqlite::ToSql for LocationInfo {
    fn to_sql(&self) -> Result<rusqlite::types::Value> {
        let state_str: String = self.state.iter().collect();
        Ok(rusqlite::types::Value::from((self.address.clone(), self.city.clone(), state_str, self.zipcode)))
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
