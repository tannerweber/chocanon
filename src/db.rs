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

use rusqlite::{Connection, OpenFlags};

const MAX_NAME_SIZE: u32 = 25;
const MAX_MEMBER_ID: u32 = 999999999; // 9 Digits
const MAX_PROVIDER_ID: u32 = 999999999; // 9 Digits
const MAX_ADDRESS_SIZE: u32 = 25;
const MAX_CITY_SIZE: u32 = 14;
const STATE_SIZE: usize = 2;
const MAX_ZIPCODE: u32 = 99999; // 5 Digits
//
const DATE_TIME_SIZE: u32 = 19; // MM-DD-YYYY HH:MM:SS
const SERVICE_DATE_SIZE: u32 = 10; // MM-DD-YYYY
const MAX_SERVICE_CODE: u32 = 999999; // 6 Digits
const MAX_COMMENT_SIZE: u32 = 100;

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
    pub fn new(path: &str) -> Self {
        let conn = match Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        ) {
            Ok(c) => c,
            Err(err) => panic!("Failed to connect to database: {}", err),
        };
        let mut sql = format!(
            "CREATE TABLE IF NOT EXISTS members (
                id          INTEGER NOT NULL PRIMARY KEY CHECK (
                    id <= {}
                    AND id >= 0
                ),
                name        TEXT NOT NULL CHECK (length(name) <= {}),
                address     TEXT NOT NULL CHECK (length(address) <= {}),
                city        TEXT NOT NULL CHECK (length(city) <= {}),
                state       TEXT NOT NULL CHECK (length(state) == {}),
                zipcode     INTEGER NOT NULL CHECK (
                    zipcode <= {}
                    AND zipcode >= 0
                ),
                is_valid    BIT
            )",
            MAX_MEMBER_ID,
            MAX_NAME_SIZE,
            MAX_ADDRESS_SIZE,
            MAX_CITY_SIZE,
            STATE_SIZE,
            MAX_ZIPCODE,
        );
        match conn.execute(&sql, []) {
            Ok(_) => (),
            Err(err) => panic!("ERROR: {}", err),
        }
        sql = format!(
            "CREATE TABLE IF NOT EXISTS providers (
                id          INTEGER NOT NULL PRIMARY KEY CHECK (
                    id <= {}
                    AND id >= 0
                ),
                name        TEXT NOT NULL CHECK (length(name) <= {}),
                address     TEXT NOT NULL CHECK (length(address) <= {}),
                city        TEXT NOT NULL CHECK (length(city) <= {}),
                state       TEXT NOT NULL CHECK (length(state) == {}),
                zipcode     INTEGER NOT NULL CHECK (
                    zipcode <= {}
                    AND zipcode >= 0
                ),
                is_valid    BIT
            )",
            MAX_PROVIDER_ID,
            MAX_NAME_SIZE,
            MAX_ADDRESS_SIZE,
            MAX_CITY_SIZE,
            STATE_SIZE,
            MAX_ZIPCODE,
        );
        match conn.execute(&sql, []) {
            Ok(_) => (),
            Err(err) => panic!("ERROR: {}", err),
        }
        sql = format!(
            "CREATE TABLE IF NOT EXISTS consultations (
                current_date_time   TEXT NOT NULL CHECK (length(current_date_time) <= {}),
                service_date        TEXT NOT NULL CHECK (length(service_date) == {}),
                member_id           INTEGER NOT NULL CHECK (
                    member_id <= {}
                    AND member_id >= 0
                ),
                provider_id         INTEGER NOT NULL PRIMARY KEY CHECK (
                    provider_id <= {}
                    AND provider_id >= 0
                ),
                service_code        INTEGER NOT NULL CHECK (
                    service_code <= {}
                    AND service_code >= 0
                ),
                comments            TEXT CHECK (length(comments) <= {})
            )",
            DATE_TIME_SIZE,
            SERVICE_DATE_SIZE,
            MAX_MEMBER_ID,
            MAX_PROVIDER_ID,
            MAX_SERVICE_CODE,
            MAX_COMMENT_SIZE,
        );
        match conn.execute(&sql, []) {
            Ok(_) => (),
            Err(err) => panic!("ERROR: {}", err),
        }
        DB { conn }
    }

    /// Sends out all member reports to all ChocAn members.
    ///
    /// # Failure
    ///
    /// Will return `Err` if any reports are not sent.
    pub fn send_member_reports() -> rusqlite::Result<(), rusqlite::Error> {
        // ONLY SEND REPORTS FOR THOSE WITH ACTIVITY IN THE PAST WEEK
        // ONLY SEND REPORTS FOR NOT SUSPENDED
        Ok(())
    }

    pub fn send_provider_reports() -> rusqlite::Result<(), rusqlite::Error> {
        // ONLY SEND REPORTS FOR THOSE WITH ACTIVITY IN THE PAST WEEK
        // ONLY SEND REPORTS FOR NOT SUSPENDED
        Ok(())
    }

    pub fn send_manager_report() -> rusqlite::Result<(), rusqlite::Error> {
        Ok(())
    }

    pub fn send_provider_directory() -> rusqlite::Result<(), rusqlite::Error> {
        Ok(())
    }

    pub fn is_valid_member_id(
        &self,
        id: u32,
    ) -> rusqlite::Result<bool, rusqlite::Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT COUNT(*) FROM members WHERE id = ?")?;
        let count: u32 =
            stmt.query_row(rusqlite::params![id], |row| row.get(0))?;
        Ok(count > 0)
    }

    pub fn is_valid_provider_id(
        &self,
        id: u32,
    ) -> rusqlite::Result<bool, rusqlite::Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT COUNT(*) FROM providers WHERE id = ?")?;
        let count: u32 =
            stmt.query_row(rusqlite::params![id], |row| row.get(0))?;
        Ok(count > 0)
    }

    pub fn is_valid_service_id(
        &self,
        _id: u32,
    ) -> rusqlite::Result<bool, rusqlite::Error> {
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

        let mut stmt = self.conn.prepare(
            "INSERT INTO members (
                id,
                name,
                address,
                city,
                state,
                zipcode,
                is_valid
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        )?;
        stmt.execute(rusqlite::params![
            &person.id,
            &person.name,
            &person.location.address,
            &person.location.city,
            &state,
            &person.location.zipcode,
            true,
        ])?;
        Ok(())
    }

    pub fn add_provider(
        &self,
        _person: &PersonInfo,
    ) -> rusqlite::Result<(), rusqlite::Error> {
        Ok(())
    }

    pub fn remove_member(
        &self,
        id: u32,
    ) -> rusqlite::Result<(), rusqlite::Error> {
        let mut stmt = self.conn.prepare("DELETE FROM members WHERE id = ?")?;
        stmt.execute(rusqlite::params![id])?;
        Ok(())
    }

    pub fn remove_provider(
        &self,
        id: u32,
    ) -> rusqlite::Result<(), rusqlite::Error> {
        let mut stmt =
            self.conn.prepare("DELETE FROM providers WHERE id = ?")?;
        stmt.execute(rusqlite::params![id])?;
        Ok(())
    }

    pub fn add_consultation_record(
        &self,
        consul: &Consultation,
    ) -> rusqlite::Result<(), rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO consultations (
                current_date_time,
                service_date,
                provider_id,
                member_id,
                service_code,
                comments
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        )?;
        stmt.execute(rusqlite::params![
            &consul.curr_date,
            &consul.service_date,
            &consul.provider_id,
            &consul.member_id,
            &consul.service_code,
            &consul.comments,
        ])?;
        Ok(())
    }

    fn retrieve_consultations(
        &self,
        id: u32,
    ) -> rusqlite::Result<Vec<Consultation>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT
                current_date_time,
                service_date,
                member_id,
                provider_id,
                service_code,
                comments,
            FROM consultations WHERE provider_id = ?",
        )?;
        let mut consul_iter = stmt.query_map(rusqlite::params![id], |row| {
            Ok(Consultation {
                curr_date: row.get(0)?,
                service_date: row.get(1)?,
                provider_id: row.get(2)?,
                member_id: row.get(3)?,
                service_code: row.get(4)?,
                comments: row.get(5)?,
            })
        })?;
        if consul_iter.next().is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }
        let mut consuls = Vec::new();
        for consul in consul_iter {
            consuls.push(consul.unwrap());
        }
        Ok(consuls)
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
            id,
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
            state: *state,
            zipcode,
        })
    }
}

/// A consultation record between a member and provider.
#[derive(Debug, Clone)]
pub struct Consultation {
    curr_date: String,
    service_date: String,
    provider_id: u32,
    member_id: u32,
    service_code: u32,
    comments: String,
}

impl Consultation {
    /// Create a consultation.
    ///
    /// # Failure
    ///
    /// Will return `Err` if a paramater is not valid.
    pub fn new(
        curr_date: &str,
        service_date: &str,
        provider_id: u32,
        member_id: u32,
        service_code: u32,
        comments: &str,
    ) -> Result<Self, String> {
        if curr_date.chars().count() != DATE_TIME_SIZE.try_into().unwrap() {
            return Err(format!(
                "current date time must be equal to {} characters: {}",
                DATE_TIME_SIZE, curr_date
            ));
        }
        if service_date.chars().count() != SERVICE_DATE_SIZE.try_into().unwrap()
        {
            return Err(format!(
                "service date must be equal to {} characters: {}",
                SERVICE_DATE_SIZE, service_date
            ));
        }
        if provider_id > MAX_PROVIDER_ID {
            return Err(format!(
                "provider_id must be less than or equal to {}: {}",
                MAX_PROVIDER_ID, provider_id
            ));
        }
        if member_id > MAX_MEMBER_ID {
            return Err(format!(
                "member_id must be less than or equal to {}: {}",
                MAX_MEMBER_ID, member_id
            ));
        }
        if service_code > MAX_SERVICE_CODE {
            return Err(format!(
                "service_code must be less than or equal to {}: {}",
                MAX_SERVICE_CODE, service_code
            ));
        }
        if comments.chars().count() == MAX_COMMENT_SIZE.try_into().unwrap() {
            return Err(format!(
                "comments must be less than or equal to {} characters: {}",
                MAX_COMMENT_SIZE, comments
            ));
        }
        Ok(Consultation {
            curr_date: curr_date.to_string(),
            service_date: service_date.to_string(),
            provider_id,
            member_id,
            service_code,
            comments: comments.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Path to the ChocAn database file.
    const TEST_DB_PATH: &str = "./test_chocanon.db3";

    fn remove_test_db() {
        let _ = std::fs::remove_file(TEST_DB_PATH);
    }

    fn get_a_person() -> PersonInfo {
        let location: LocationInfo =
            LocationInfo::new("1234 Main st", "Portland", &['O', 'R'], 56789)
                .unwrap();
        let person: PersonInfo =
            PersonInfo::new("Timmy Smith", 123456789, &location).unwrap();
        person
    }

    fn get_a_consultation() -> Consultation {
        let consul: Consultation = Consultation::new(
            "01-13-2025 03:45:25",
            "01-13-2025",
            123456789,
            987654321,
            123456,
            "This is a comment.",
        )
        .unwrap();
        consul
    }

    fn get_populated_database() -> DB {
        let db = DB::new(TEST_DB_PATH);
        let location: LocationInfo = LocationInfo {
            address: "1234 main st".to_string(),
            city: "Portland".to_string(),
            state: ['O', 'R'],
            zipcode: 12345,
        };
        let person: PersonInfo = PersonInfo {
            id: 123456789,
            name: "First Last".to_string(),
            location: location,
        };
        db.add_member(&person).unwrap();
        db
    }

    #[test]
    fn test_send_member_reports() {}

    #[test]
    fn test_send_provider_reports() {}

    #[test]
    fn test_send_manager_report() {}

    #[test]
    fn test_send_provider_directory() {}

    #[test]
    fn test_is_valid_member_id() {}

    #[test]
    fn test_is_valid_provider_id() {}

    #[test]
    fn test_is_valid_service_id() {}

    #[test]
    fn test_add_member() {
        remove_test_db();
        let db: DB = DB::new(TEST_DB_PATH);
        let person: PersonInfo = get_a_person();
        match db.add_member(&person) {
            Ok(_) => (),
            Err(err) => {
                panic!("ERROR: {}", err);
            }
        }
    }

    #[test]
    fn test_add_provider() {}

    #[test]
    fn test_remove_member() {
        let db = get_populated_database();
        match db.remove_member(123456789) {
            Ok(_) => (),
            Err(err) => panic!("ERROR: {}", err),
        }
    }

    #[test]
    fn test_remove_provider() {}

    #[test]
    fn test_add_consultation_record() {
        remove_test_db();
        let db: DB = DB::new(TEST_DB_PATH);
        let consul: Consultation = get_a_consultation();
        match db.add_consultation_record(&consul) {
            Ok(_) => (),
            Err(err) => {
                panic!("ERROR: {}", err);
            }
        }
    }
}
