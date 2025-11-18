/* File: db.rs
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

//! Module for the Chocaholics Anonymous database.

use rusqlite::Connection;

const MAX_NAME_SIZE: u32 = 25;
const MAX_MEMBER_ID: u32 = 999999999; // 9 Digits
const MAX_PROVIDER_ID: u32 = 999999999; // 9 Digits
const MAX_ADDRESS_SIZE: u32 = 25;
const MAX_CITY_SIZE: u32 = 14;
const STATE_SIZE: usize = 2;
const MAX_ZIPCODE: u32 = 99999; // 5 Digits
//
const DATE_TIME_SIZE: u32 = 19; // MM-DD-YYYY HH:MM:SS
const DATE_SIZE: u32 = 10; // MM-DD-YYYY
const MAX_SERVICE_CODE: u32 = 999999; // 6 Digits
const MAX_COMMENT_SIZE: u32 = 100;

/// Path to the ChocAn database file.
const DB_PATH: &str = "./chocanon.db3";

/// A ChocAn database.
#[derive(Debug)]
pub struct DB {
    conn: Connection,
}

impl DB {
    /// Create a ChocAn database
    ///
    /// # Failure
    ///
    /// Panics on failing to connect and create tables.
    pub fn new() -> Self {
        let conn: Connection;
        match Connection::open_with_flags(
            DB_PATH,
            rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE,
        ) {
            Ok(c) => conn = c,
            Err(err) => panic!("Failed to connect to database: {}", err),
        }
        match conn.execute(
            "CREATE TABLE IF NOT EXISTS members (
                id          INTEGER CHECK (id <= {}),
                name        TEXT NOT NULL CHECK (length(name) <= {}),
                address     TEXT NOT NULL CHECK (length(address) <= {}),
                city        TEXT NOT NULL CHECK (length(city) <= {}),
                state       TEXT NOT NULL CHECK (length(state) == {}),
                zipcode     INTEGER CHECK (zipcode <= {}),
                is_valid    BIT
                PRIMARY KEY (id)
            )",
            [
                MAX_MEMBER_ID,
                MAX_NAME_SIZE,
                MAX_ADDRESS_SIZE,
                MAX_CITY_SIZE,
                STATE_SIZE.try_into().unwrap(),
                MAX_ZIPCODE,
            ],
        ) {
            Ok(n) => eprintln!("Updated {} rows.", n),
            Err(err) => panic!("ERROR: {}", err),
        }
        match conn.execute(
            "CREATE TABLE IF NOT EXISTS providers (
                id          INTEGER CHECK (id <= {}),
                name        TEXT NOT NULL CHECK (length(name) <= {}),
                address     TEXT NOT NULL CHECK (length(address) <= {}),
                city        TEXT NOT NULL CHECK (length(city) <= {}),
                state       TEXT NOT NULL CHECK (length(state) == {}),
                zipcode     INTEGER CHECK (zipcode <= {}),
                is_valid    BIT
                PRIMARY KEY (id)
            )",
            [
                MAX_PROVIDER_ID,
                MAX_NAME_SIZE,
                MAX_ADDRESS_SIZE,
                MAX_CITY_SIZE,
                STATE_SIZE.try_into().unwrap(),
                MAX_ZIPCODE,
            ],
        ) {
            Ok(n) => eprintln!("Updated {} rows.", n),
            Err(err) => panic!("ERROR: {}", err),
        }
        match conn.execute(
            "CREATE TABLE IF NOT EXISTS consultations (
                current_date_time   TEXT NOT NULL CHECK (length(name) <= {}),
                date                TEXT NOT NULL CHECK (length(date) == {}),
                member_id           INTEGER CHECK (member_id <= {}),
                provider_id         INTEGER CHECK (provider_id <= {}),
                service_code        INTEGER CHECK (service_code <= {}),
                comments            TEXT CHECK (length(comments) <= {}),
                PRIMARY KEY (member_id)
            )",
            [
                DATE_TIME_SIZE,
                DATE_SIZE,
                MAX_MEMBER_ID,
                MAX_PROVIDER_ID,
                MAX_SERVICE_CODE,
                MAX_COMMENT_SIZE,
            ],
        ) {
            Ok(n) => eprintln!("Updated {} rows.", n),
            Err(err) => panic!("ERROR: {}", err),
        }
        DB { conn: conn }
    }

    /// Sends out all member reports to all ChocAn members.
    ///
    /// # Failure
    ///
    /// Will return `Err` if any reports are not sent.
    pub fn send_member_reports() -> rusqlite::Result<bool, rusqlite::Error> {
        Ok(false)
    }

    pub fn send_provider_reports() -> rusqlite::Result<bool, rusqlite::Error> {
        Ok(false)
    }

    pub fn send_manager_report() -> rusqlite::Result<bool, rusqlite::Error> {
        Ok(false)
    }

    pub fn is_valid_member_id() -> rusqlite::Result<bool, rusqlite::Error> {
        Ok(false)
    }

    pub fn is_valid_provider_id() -> rusqlite::Result<bool, rusqlite::Error> {
        Ok(false)
    }

    pub fn is_valid_service_id() -> rusqlite::Result<bool, rusqlite::Error> {
        Ok(false)
    }

    /// Adds a member to the database.
    ///
    /// # Failure
    ///
    /// Will return `Err` if the member was not added.
    pub fn add_member(
        &self,
        person: &PersonInfo,
    ) -> rusqlite::Result<(), rusqlite::Error> {
        let state: String = person.location.state.iter().collect();

        self.conn.execute(
            "INSERT INTO members (
                id,
                name,
                address,
                city,
                state,
                zipcode
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (
                &person.id,
                &person.name,
                &person.location.address,
                &person.location.city,
                &state,
                &person.location.zipcode,
            ),
        )?;
        Ok(())
    }

    pub fn add_provider(
        &self,
        person: &PersonInfo,
    ) -> rusqlite::Result<bool, rusqlite::Error> {
        Ok(false)
    }

    pub fn remove_member(
        &self,
        id: u32,
    ) -> rusqlite::Result<bool, rusqlite::Error> {
        Ok(false)
    }

    pub fn remove_provider(
        &self,
        id: u32,
    ) -> rusqlite::Result<bool, rusqlite::Error> {
        Ok(false)
    }

    pub fn add_consultation_record() -> rusqlite::Result<bool, rusqlite::Error>
    {
        Ok(false)
    }
}

/// Information on a person in the ChocAn database.
#[derive(Debug)]
pub struct PersonInfo {
    id: u32,
    name: String,
    location: LocationInfo,
}

impl PersonInfo {
    /// Create a person.
    ///
    /// # Failure
    ///
    /// Will return `Err` if a paramater is not valid.
    pub fn new(
        name: &str,
        id: u32,
        location: &LocationInfo,
    ) -> Result<Self, String> {
        if id > MAX_MEMBER_ID {
            return Err(format!(
                "id must be less than or equal to {}: {}",
                MAX_MEMBER_ID, id
            ));
        }
        if name.chars().count() > MAX_NAME_SIZE.try_into().unwrap() {
            return Err(format!(
                "name must be less than or equal to {} characters: {}",
                MAX_NAME_SIZE, name
            ));
        }
        Ok(PersonInfo {
            name: name.to_string(),
            id: id,
            location: location.clone(),
        })
    }
}

/// Information on location for a person in the ChocAn database.
#[derive(Debug, Clone)]
pub struct LocationInfo {
    address: String,
    city: String,
    state: [char; STATE_SIZE],
    zipcode: u32,
}

impl LocationInfo {
    /// Create a location.
    ///
    /// # Failure
    ///
    /// Will return `Err` if a paramater is not valid.
    pub fn new(
        address: &str,
        city: &str,
        state: &[char; STATE_SIZE],
        zipcode: u32,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_member() {
        eprintln!("Running test_add_member");
        assert_eq!(1, 1);
    }
}
