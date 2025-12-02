/* File: db.rs
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

//! Module for the Chocaholics Anonymous database.

use crate::esend::*;
use chrono::{Duration, Local};
use regex::Regex;
use rusqlite::{Connection, OpenFlags};
use std::collections::HashMap;
use std::collections::hash_map::Entry;

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
//
/// Reports newer than this many days ago will be sent.
const REPORT_DATE_RANGE: i64 = 7;
const CHOCAN_EMAIL: &str = "testing@chocan.com";

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Sql(rusqlite::Error),
    Regex(regex::Error),
    EmptyInput,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "STD IO error: {}", err),
            Error::Sql(ref err) => write!(f, "Rusqlite error: {}", err),
            Error::Regex(ref err) => write!(f, "Regex error: {}", err),
            Error::EmptyInput => write!(f, "Empty input error"),
        }
    }
}

/// A ChocAn database.
#[derive(Debug)]
pub struct DB {
    conn: Connection,
}

impl DB {
    /// Create a ChocAn database
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the database file (E.g. `./database.db3`).
    ///
    /// # Failure
    ///
    /// Will return `Err` if database could not be established.
    pub fn new(path: &str) -> Result<Self, Error> {
        let conn = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        )
        .map_err(Error::Sql)?;
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
                email       TEXT NOT NULL,
                is_valid    BIT
            )",
            MAX_MEMBER_ID,
            MAX_NAME_SIZE,
            MAX_ADDRESS_SIZE,
            MAX_CITY_SIZE,
            STATE_SIZE,
            MAX_ZIPCODE,
        );
        conn.execute(&sql, []).map_err(Error::Sql)?;
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
                email       TEXT NOT NULL,
                is_valid    BIT
            )",
            MAX_PROVIDER_ID,
            MAX_NAME_SIZE,
            MAX_ADDRESS_SIZE,
            MAX_CITY_SIZE,
            STATE_SIZE,
            MAX_ZIPCODE,
        );
        conn.execute(&sql, []).map_err(Error::Sql)?;
        sql = format!(
            "CREATE TABLE IF NOT EXISTS consultations (
                current_date_time   TEXT NOT NULL CHECK (
                    length(current_date_time) <= {}
                ),
                service_date        TEXT NOT NULL CHECK (
                    length(service_date) == {}
                ),
                member_id           INTEGER NOT NULL CHECK (
                    member_id <= {}
                    AND member_id >= 0
                ),
                provider_id         INTEGER NOT NULL CHECK (
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
        conn.execute(&sql, []).map_err(Error::Sql)?;
        sql = format!(
            "CREATE TABLE IF NOT EXISTS provider_directory (
                service_id  INTEGER NOT NULL PRIMARY KEY CHECK (
                    service_id <= {}
                    AND service_id >= 0
                ),
                name        TEXT NOT NULL,
                fee         REAL NOT NULL CHECK (fee >= 0)
            )",
            MAX_SERVICE_CODE,
        );
        conn.execute(&sql, []).map_err(Error::Sql)?;
        Ok(DB { conn })
    }

    /// Sends out all member reports to all ChocAn members.
    ///
    /// Reports will only be sent to those with recent activity.
    /// Reports will only be sent valid persons.
    ///
    /// # Failure
    ///
    /// Will return `Err` if any reports are not sent.
    pub fn send_member_reports(&self) -> Result<(), Error> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT
                consultations.service_date,
                consultations.member_id,
                consultations.provider_id,
                consultations.service_code
                FROM consultations
                ORDER BY consultations.service_date ASC",
            )
            .map_err(Error::Sql)?;
        let rows = stmt
            .query_map([], |row| {
                let service_date: String = row.get(0)?;
                let member_id: u32 = row.get(1)?;
                let provider_id: u32 = row.get(2)?;
                let service_id: u32 = row.get(3)?;
                Ok((service_date, member_id, provider_id, service_id))
            })
            .map_err(Error::Sql)?;

        let mut reports = HashMap::new();
        for (service_date, member_id, provider_id, service_id) in rows.flatten()
        {
            if service_date
                < (Local::now() - Duration::days(REPORT_DATE_RANGE))
                    .format("%m-%d-%Y")
                    .to_string()
            {
                continue;
            }
            let member: PersonInfo = self.get_member_info(member_id)?;
            let provider: PersonInfo = self.get_provider_info(provider_id)?;
            let service_name: String = self.get_service_name(service_id)?;
            let subject = "Member Report for ".to_owned() + &member.name;
            let consul_text = Self::create_consultation_text(
                &service_date,
                &provider.name,
                &service_name,
            );

            if let Entry::Vacant(e) = reports.entry(member_id) {
                let body = Self::create_member_report_body(&member);
                e.insert((member.email, subject, body, member.name));
            }
            if let Some(values) = reports.get_mut(&member_id) {
                values.2.push_str(&consul_text);
                *values = (
                    values.0.clone(),
                    values.1.clone(),
                    values.2.clone(),
                    values.3.clone(),
                );
            }
        }

        for (_key, (email, subject, body, name)) in reports {
            send_member_report(&email, CHOCAN_EMAIL, &subject, &body, &name)
                .map_err(Error::Io)?;
        }
        Ok(())
    }

    fn create_member_report_body(member: &PersonInfo) -> String {
        format!("Member name: {}\n", member.name)
            + &format!("Member number: {}\n", member.id)
            + &format!("Member street address: {}\n", member.location.address)
            + &format!("Member city: {}\n", member.location.city)
            + &format!("Member state: {}\n", member.location.state)
            + &format!("Member zip code: {}\n", member.location.zipcode)
    }

    fn create_consultation_text(
        service_date: &str,
        provider_name: &str,
        service_name: &str,
    ) -> String {
        "----------------------------------------\n".to_string()
            + &format!("Date of service: {}\n", service_date)
            + &format!("Provider name: {}\n", provider_name)
            + &format!("Service name: {}\n", service_name)
    }

    /// Sends out all provider reports to all ChocAn providers.
    ///
    /// Reports will only be sent to those with recent activity.
    /// Reports will only be sent valid persons.
    ///
    /// # Failure
    ///
    /// Will return `Err` if any reports are not sent.
    pub fn send_provider_reports(&self) -> Result<(), Error> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT
                consultations.service_date,
                consultations.member_id,
                consultations.provider_id,
                consultations.service_code,
                consultations.current_date_time
                FROM consultations
                ORDER BY consultations.service_date ASC",
            )
            .map_err(Error::Sql)?;
        let rows = stmt
            .query_map([], |row| {
                let service_date: String = row.get(0)?;
                let member_id: u32 = row.get(1)?;
                let provider_id: u32 = row.get(2)?;
                let service_id: u32 = row.get(3)?;
                let current_date_time: String = row.get(4)?;
                Ok((
                    service_date,
                    member_id,
                    provider_id,
                    service_id,
                    current_date_time,
                ))
            })
            .map_err(Error::Sql)?;

        let mut reports = HashMap::new();
        for (
            service_date,
            member_id,
            provider_id,
            service_id,
            current_date_time,
        ) in rows.flatten()
        {
            if service_date
                < (Local::now() - Duration::days(REPORT_DATE_RANGE))
                    .format("%m-%d-%Y")
                    .to_string()
            {
                continue;
            }
            let member: PersonInfo = self.get_member_info(member_id)?;
            let provider: PersonInfo = self.get_provider_info(provider_id)?;
            let fee: f64 = self.get_service_fee(service_id)?;
            let subject = "Provider Report for ".to_owned() + &provider.name;
            let consul_text = Self::create_provider_consultation_text(
                &service_date,
                &current_date_time,
                &member.name,
                member_id,
                service_id,
                fee,
            );

            if let Entry::Vacant(e) = reports.entry(provider_id) {
                let body = Self::create_provider_report_body(&provider);
                e.insert((provider.email, subject, body, provider.name));
            }
            if let Some(values) = reports.get_mut(&provider_id) {
                values.2.push_str(&consul_text);
                *values = (
                    values.0.clone(),
                    values.1.clone(),
                    values.2.clone(),
                    values.3.clone(),
                );
            }
        }

        for (_key, (email, subject, body, name)) in reports {
            send_provider_report(&email, CHOCAN_EMAIL, &subject, &body, &name)
                .map_err(Error::Io)?;
        }
        Ok(())
    }

    fn create_provider_report_body(provider: &PersonInfo) -> String {
        format!("Provider name: {}\n", provider.name)
            + &format!("Provider number: {}\n", provider.id)
            + &format!(
                "Provider street address: {}\n",
                provider.location.address
            )
            + &format!("Provider city: {}\n", provider.location.city)
            + &format!("Provider state: {}\n", provider.location.state)
            + &format!("Provider zip code: {}\n", provider.location.zipcode)
    }

    fn create_provider_consultation_text(
        service_date: &str,
        service_date_time: &str,
        member_name: &str,
        member_number: u32,
        service_code: u32,
        fee: f64,
    ) -> String {
        "----------------------------------------\n".to_string()
            + &format!("Date of service: {}\n", service_date)
            + &format!(
                "Date and time data were received by the computer: {}\n",
                service_date_time
            )
            + &format!("Member name: {}\n", member_name)
            + &format!("Member number: {}\n", member_number)
            + &format!("Service code: {}\n", service_code)
            + &format!("Fee: {}\n", fee)
    }

    fn create_provider_report_footer(
        total_consultations: u32,
        total_fee: f64,
    ) -> String {
        "----------------------------------------\n".to_string()
            + &format!("Total consultations: {}\n", total_consultations)
            + &format!("Total fee: {}\n", total_fee)
    }

    /// Sends out a manager report to the ChocAn manager.
    ///
    /// # Failure
    ///
    /// Will return `Err` if any reports are not sent.
    pub fn send_manager_report(&self) -> Result<(), Error> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT
                current_date_time,
                service_date,
                member_id,
                provider_id,
                service_code,
                comments
            FROM consultations",
            )
            .map_err(Error::Sql)?;
        let rows = stmt
            .query_map([], |row| {
                Ok(Consultation {
                    curr_date: row.get(0)?,
                    service_date: row.get(1)?,
                    provider_id: row.get(2)?,
                    member_id: row.get(3)?,
                    service_code: row.get(4)?,
                    comments: row.get(5)?,
                })
            })
            .map_err(Error::Sql)?;

        let mut report: String = "".to_string();
        for consul in rows.flatten() {
            report.push_str(&format!("{}\n", consul));
        }
        send_manager_report(
            "manager@pdx.edu",
            CHOCAN_EMAIL,
            "Manager report",
            &report,
            "ManagerName",
        )
        .map_err(Error::Io)?;
        Ok(())
    }

    /// Sends out a the provider directory to the specified email.
    ///
    /// # Arguments
    ///
    /// * `email` - The email address to send the provider directory to.
    ///
    /// # Failure
    ///
    /// Will return `Err` if not sent.
    pub fn send_provider_directory(&self, email: &str) -> Result<(), Error> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT
                service_id,
                name,
                fee
            FROM provider_directory
            ORDER BY name ASC",
            )
            .map_err(Error::Sql)?;
        let rows = stmt
            .query_map([], |row| {
                let service_id: u32 = row.get(0)?;
                let name: String = row.get(1)?;
                let fee: f64 = row.get(2)?;
                Ok((service_id, name, fee))
            })
            .map_err(Error::Sql)?;

        let mut email_body: String = "".to_string();
        for (service_id, name, fee) in rows.flatten() {
            email_body.push_str(&format!(
                "{}, ID: {}, Fee: {}\n",
                name, service_id, fee
            ));
        }
        send_provider_directory(
            email,
            CHOCAN_EMAIL,
            "Provider Directory",
            &email_body,
            "ProviderName",
        )
        .map_err(Error::Io)?;
        Ok(())
    }

    /// Checks if the member id belongs to a member in the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The member id to check.
    ///
    /// # Success
    ///
    /// Will return `Ok` wrapping `true` if valid person found.
    /// Otherwise, will return `Ok` wrapping `false`.
    ///
    /// # Failure
    ///
    /// Will return `Err` on database error.
    pub fn is_valid_member_id(&self, id: u32) -> Result<bool, Error> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT COUNT(*) FROM members WHERE id = ? AND is_valid = 1",
            )
            .map_err(Error::Sql)?;
        let count: u32 = stmt
            .query_row(rusqlite::params![id], |row| row.get(0))
            .map_err(Error::Sql)?;
        Ok(count > 0)
    }

    /// Checks if the provider id belongs to a provider in the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The provider id to check.
    ///
    /// # Success
    ///
    /// Will return `Ok` wrapping `true` if valid person found.
    /// Otherwise, will return `Ok` wrapping `false`.
    ///
    /// # Failure
    ///
    /// Will return `Err` on database error.
    pub fn is_valid_provider_id(&self, id: u32) -> Result<bool, Error> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT COUNT(*) FROM providers WHERE id = ? AND is_valid = 1",
            )
            .map_err(Error::Sql)?;
        let count: u32 = stmt
            .query_row(rusqlite::params![id], |row| row.get(0))
            .map_err(Error::Sql)?;
        Ok(count > 0)
    }

    /// Checks if the service id belongs to a service in the directory.
    ///
    /// # Arguments
    ///
    /// * `id` - The service id to check.
    ///
    /// # Success
    ///
    /// Will return `Ok` wrapping `true` if valid service found.
    /// Otherwise, will return `Ok` wrapping `false`.
    ///
    /// # Failure
    ///
    /// Will return `Err` on database error.
    pub fn is_valid_service_id(&self, id: u32) -> Result<bool, Error> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT COUNT(*) FROM provider_directory WHERE service_id = ?",
            )
            .map_err(Error::Sql)?;
        let count: u32 = stmt
            .query_row(rusqlite::params![id], |row| row.get(0))
            .map_err(Error::Sql)?;
        Ok(count > 0)
    }

    /// Adds a member to the database.
    ///
    /// # Arguments
    ///
    /// * `person` - The member to add to the database.
    ///
    /// # Failure
    ///
    /// Will return `Err` if the member was not added.
    pub fn add_member(&self, person: &PersonInfo) -> Result<(), Error> {
        let mut stmt = self
            .conn
            .prepare(
                "INSERT INTO members (
                id,
                name,
                address,
                city,
                state,
                zipcode,
                email,
                is_valid
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            )
            .map_err(Error::Sql)?;
        stmt.execute(rusqlite::params![
            &person.id,
            &person.name,
            &person.location.address,
            &person.location.city,
            &person.location.state,
            &person.location.zipcode,
            &person.email,
            1,
        ])
        .map_err(Error::Sql)?;
        Ok(())
    }

    /// Adds a provider to the database.
    ///
    /// # Arguments
    ///
    /// * `person` - The provider to add to the database.
    ///
    /// # Failure
    ///
    /// Will return `Err` if the provider was not added.
    pub fn add_provider(&self, person: &PersonInfo) -> Result<(), Error> {
        let mut stmt = self
            .conn
            .prepare(
                "INSERT INTO providers (
                id,
                name,
                address,
                city,
                state,
                zipcode,
                email,
                is_valid
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            )
            .map_err(Error::Sql)?;
        stmt.execute(rusqlite::params![
            &person.id,
            &person.name,
            &person.location.address,
            &person.location.city,
            &person.location.state,
            &person.location.zipcode,
            &person.email,
            1,
        ])
        .map_err(Error::Sql)?;
        Ok(())
    }

    /// Removes a member from the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the member to remove.
    ///
    /// # Failure
    ///
    /// Will return `Err` if the member was not removed.
    pub fn remove_member(&self, id: u32) -> Result<(), Error> {
        let mut stmt = self
            .conn
            .prepare("DELETE FROM members WHERE id = ?")
            .map_err(Error::Sql)?;
        let n = stmt.execute(rusqlite::params![id]).map_err(Error::Sql)?;
        if n == 0 {
            return Err(Error::Sql(rusqlite::Error::QueryReturnedNoRows));
        }
        Ok(())
    }

    /// Removes a provider from the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the provider to remove.
    ///
    /// # Failure
    ///
    /// Will return `Err` if the provider was not removed.
    pub fn remove_provider(&self, id: u32) -> Result<(), Error> {
        let mut stmt = self
            .conn
            .prepare("DELETE FROM providers WHERE id = ?")
            .map_err(Error::Sql)?;
        let n = stmt.execute(rusqlite::params![id]).map_err(Error::Sql)?;
        if n == 0 {
            return Err(Error::Sql(rusqlite::Error::QueryReturnedNoRows));
        }
        Ok(())
    }

    /// Adds a consultation record to the database.
    ///
    /// # Arguments
    ///
    /// * `consul` - The consultation to add to the database.
    ///
    /// # Failure
    ///
    /// Will return `Err` if the record was not added.
    pub fn add_consultation_record(
        &self,
        consul: &Consultation,
    ) -> Result<(), Error> {
        let mut stmt = self
            .conn
            .prepare(
                "INSERT INTO consultations (
                current_date_time,
                service_date,
                provider_id,
                member_id,
                service_code,
                comments
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            )
            .map_err(Error::Sql)?;
        stmt.execute(rusqlite::params![
            &consul.curr_date,
            &consul.service_date,
            &consul.provider_id,
            &consul.member_id,
            &consul.service_code,
            &consul.comments,
        ])
        .map_err(Error::Sql)?;
        Ok(())
    }

    /// Adds a service to the provider directory.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the service.
    /// * `name` - The name of the service.
    /// * `fee` - The fee for the service.
    ///
    /// # Failure
    ///
    /// Will return `Err` if the service was not added.
    pub fn add_service(
        &self,
        id: u32,
        name: &str,
        fee: f64,
    ) -> Result<(), Error> {
        if name.chars().count() == 0 {
            return Err(Error::EmptyInput);
        }
        let mut stmt = self
            .conn
            .prepare(
                "INSERT INTO provider_directory (
                service_id,
                name,
                fee
            ) VALUES (?1, ?2, ?3)",
            )
            .map_err(Error::Sql)?;
        stmt.execute(rusqlite::params![id, name, fee])
            .map_err(Error::Sql)?;
        Ok(())
    }

    /// Gets the name corresponding to the specified service code id.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the service to get the name of.
    ///
    /// # Failure
    ///
    /// Will return `Err` if the name could not be retrieved.
    pub fn get_service_name(&self, id: u32) -> Result<String, Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT name FROM provider_directory WHERE service_id = ?")
            .map_err(Error::Sql)?;
        let rows = stmt
            .query_map([id], |row| {
                let name: String = row.get(0)?;
                Ok(name)
            })
            .map_err(Error::Sql)?;
        if let Some(name) = rows.flatten().next() {
            return Ok(name);
        }
        Err(Error::Sql(rusqlite::Error::QueryReturnedNoRows))
    }

    /// Gets the fee corresponding to the specified service code id.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the service to get the fee of.
    ///
    /// # Failure
    ///
    /// Will return `Err` if the fee could not be retrieved.
    pub fn get_service_fee(&self, id: u32) -> Result<f64, Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT fee FROM provider_directory WHERE service_id = ?")
            .map_err(Error::Sql)?;
        let rows = stmt
            .query_map([id], |row| {
                let fee: f64 = row.get(0)?;
                Ok(fee)
            })
            .map_err(Error::Sql)?;
        if let Some(fee) = rows.flatten().next() {
            return Ok(fee);
        }
        Err(Error::Sql(rusqlite::Error::QueryReturnedNoRows))
    }

    /// Gets the data for a member in the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the member to get the data for.
    ///
    /// # Failure
    ///
    /// Will return `Err` if the data could not be retrieved.
    pub fn get_member_info(&self, id: u32) -> Result<PersonInfo, Error> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT
                name,
                address,
                city,
                state,
                zipcode,
                email
                FROM members WHERE id = ?",
            )
            .map_err(Error::Sql)?;
        let rows = stmt
            .query_map([id], |row| {
                let name: String = row.get(0)?;
                let address: String = row.get(1)?;
                let city: String = row.get(2)?;
                let state: String = row.get(3)?;
                let zipcode: u32 = row.get(4)?;
                let email: String = row.get(5)?;
                let location =
                    LocationInfo::new(&address, &city, &state, zipcode)
                        .unwrap();
                Ok(PersonInfo::new(&name, id, &location, &email).unwrap())
            })
            .map_err(Error::Sql)?;
        if let Some(person) = rows.flatten().next() {
            return Ok(person);
        }
        Err(Error::Sql(rusqlite::Error::QueryReturnedNoRows))
    }

    /// Gets the data for a provider in the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the provider to get the data for.
    ///
    /// # Failure
    ///
    /// Will return `Err` if the data could not be retrieved.
    pub fn get_provider_info(&self, id: u32) -> Result<PersonInfo, Error> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT
                name,
                address,
                city,
                state,
                zipcode,
                email
                FROM providers WHERE id = ?",
            )
            .map_err(Error::Sql)?;
        let rows = stmt
            .query_map([id], |row| {
                let name: String = row.get(0)?;
                let address: String = row.get(1)?;
                let city: String = row.get(2)?;
                let state: String = row.get(3)?;
                let zipcode: u32 = row.get(4)?;
                let email: String = row.get(5)?;
                let location =
                    LocationInfo::new(&address, &city, &state, zipcode)
                        .unwrap();
                Ok(PersonInfo::new(&name, id, &location, &email).unwrap())
            })
            .map_err(Error::Sql)?;
        if let Some(person) = rows.flatten().next() {
            return Ok(person);
        }
        Err(Error::Sql(rusqlite::Error::QueryReturnedNoRows))
    }
}

/// Information on a person in the ChocAn database.
#[derive(Debug)]
pub struct PersonInfo {
    id: u32,
    name: String,
    location: LocationInfo,
    email: String,
}

impl PersonInfo {
    /// Create a person.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the person. Constrained by `MAX_NAME_SIZE`.
    /// * `id` - The unique id for the person. Constrained by `MAX_MEMBER_ID`.
    /// * `location` - The location of the person.
    /// * `email` - The email of the person.
    ///
    /// # Failure
    ///
    /// Will return `Err` if a paramater is not valid.
    pub fn new(
        name: &str,
        id: u32,
        location: &LocationInfo,
        email: &str,
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
        match email.find('@') {
            Some(_) => (),
            None => {
                return Err("Email does not have an '@' symbol".to_string());
            }
        }
        Ok(PersonInfo {
            name: name.to_string(),
            id,
            location: location.clone(),
            email: email.to_string(),
        })
    }
}

/// Information on location for a person in the ChocAn database.
#[derive(Debug, Clone)]
pub struct LocationInfo {
    address: String,
    city: String,
    state: String,
    zipcode: u32,
}

impl LocationInfo {
    /// Create a location.
    ///
    /// # Arguments
    ///
    /// * `address` - The address. Constrained by `MAX_ADDRESS_SIZE`.
    /// * `city` - The city name. Constrained by `MAX_CITY_SIZE`.
    /// * `state` - The two character state name. Constrained by `STATE_SIZE`.
    /// * `zipcode` - The zipcode number. Constrained by `MAX_ZIPCODE`.
    ///
    /// # Failure
    ///
    /// Will return `Err` if a paramater is not valid.
    pub fn new(
        address: &str,
        city: &str,
        state: &str,
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
            state: state.to_string(),
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

impl std::fmt::Display for Consultation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Current date-time: {}\n
            Service date: {}\n
            Provider ID: {}\n
            Member ID: {}\n
            Servide Code: {}\n
            Comments: {}\n",
            self.curr_date,
            self.service_date,
            self.provider_id,
            self.member_id,
            self.service_code,
            self.comments,
        )
    }
}

impl Consultation {
    /// Create a consultation.
    ///
    /// # Arguments
    ///
    /// * `curr_date` - The current date time string when the email is being
    ///   sent. Constrained by `DATE_TIME_SIZE`. Format: "MM-DD-YYYY HH:MM:SS".
    /// * `service_date` - The date when the consultation occured.
    ///   Constrained by `SERVICE_DATE_SIZE`. Format: "MM-DD-YYYY".
    /// * `provider_id` - The id of the provider. Constrained
    ///   by `MAX_PROVIDER_ID`.
    /// * `member_id` - The id of the member. Constrained
    ///   by `MAX_MEMBER_ID`.
    /// * `service_code` - The code of the service issued. Constrained
    ///   by `MAX_SERVICE_CODE`.
    /// * `comments` - Comments on the service. Constrained
    ///   by `MAX_COMMENT_SIZE`.
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
        let re = match Regex::new(
            r"^(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01])-(\d{4})$",
        ) {
            Ok(re) => re,
            Err(err) => {
                return Err(format!("Error creating regex: {}", err));
            }
        };
        if !re.is_match(service_date) {
            return Err(format!(
                "service date must match format MM-DD-YYYY: {}",
                service_date
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
        let location =
            LocationInfo::new("1234 Main st", "Portland", "OR", 56789).unwrap();
        let person = PersonInfo::new(
            "Timmy Smith",
            123456789,
            &location,
            "timmmy@pdx.edu",
        )
        .unwrap();
        person
    }

    fn create_a_unique_person(name: &str, id: u32) -> PersonInfo {
        let location =
            LocationInfo::new("1234 Main st", "Portland", "OR", 56789).unwrap();
        let email = format!("{}@pdx.edu", name);
        let person = PersonInfo::new(name, id, &location, &email).unwrap();
        person
    }

    /// Creates a consultation with a date of yesterday.
    fn create_a_unique_consultation(
        member_id: u32,
        provider_id: u32,
    ) -> Consultation {
        let date = (Local::now() - Duration::days(1))
            .format("%m-%d-%Y")
            .to_string();
        let date_time = (Local::now() - Duration::days(1))
            .format("%m-%d-%Y %H:%M:%S")
            .to_string();
        let consul = Consultation::new(
            &date_time,
            &date,
            provider_id,
            member_id,
            123456,
            "This is a comment created by create_a_unique_consultation",
        )
        .unwrap();
        consul
    }

    fn get_a_consultation() -> Consultation {
        let consul: Consultation = Consultation::new(
            "01-13-2025 03:45:25",
            "01-13-2025",
            123456789,
            987654321,
            123456,
            "This is a comment for a consultation create by 
            get_a_consultation.",
        )
        .unwrap();
        consul
    }

    #[test]
    fn test_consultation_constructor() {
        match Consultation::new(
            "01-13-2025:07:45:39",
            "01-13-2025",
            123456789,
            123456789,
            123456,
            "This is a comment",
        ) {
            Ok(_) => (),
            Err(err) => {
                panic!("test_consultation_constructor() ERROR: {}", err)
            }
        }
        match Consultation::new(
            "01-13-2025:07:45:39",
            "01_13-2025",
            123456789,
            123456789,
            123456,
            "This is a comment",
        ) {
            Ok(_) => panic!("Invalid format should give an error"),
            Err(_) => (),
        }
    }

    #[test]
    fn test_send_member_reports() {
        remove_test_db();
        let db = DB::new(TEST_DB_PATH).unwrap();
        db.add_service(123456, "ServiceName123456", 99.99).unwrap();
        db.add_member(&create_a_unique_person("MemberName1", 1))
            .unwrap();
        db.add_member(&create_a_unique_person("MemberName2", 2))
            .unwrap();
        db.add_member(&create_a_unique_person("MemberName3", 3))
            .unwrap();
        db.add_member(&create_a_unique_person("MemberName4", 4))
            .unwrap();
        db.add_provider(&create_a_unique_person("ProviderName1", 61))
            .unwrap();
        db.add_provider(&create_a_unique_person("ProviderName2", 62))
            .unwrap();
        db.add_consultation_record(&create_a_unique_consultation(1, 61))
            .unwrap();
        db.add_consultation_record(&create_a_unique_consultation(2, 61))
            .unwrap();
        db.add_consultation_record(&create_a_unique_consultation(2, 61))
            .unwrap();
        db.add_consultation_record(&create_a_unique_consultation(3, 61))
            .unwrap();
        db.add_consultation_record(&create_a_unique_consultation(3, 61))
            .unwrap();
        db.add_consultation_record(&create_a_unique_consultation(3, 61))
            .unwrap();
        match db.send_member_reports() {
            Ok(_) => (),
            Err(err) => panic!("send_member_reports() ERROR: {}", err),
        }
    }

    #[test]
    fn test_send_provider_reports() {
        remove_test_db();
        let db = DB::new(TEST_DB_PATH).unwrap();
        db.add_service(123456, "ServiceName123456", 99.99).unwrap();
        db.add_member(&create_a_unique_person("MemberName1", 1))
            .unwrap();
        db.add_member(&create_a_unique_person("MemberName2", 2))
            .unwrap();
        db.add_member(&create_a_unique_person("MemberName3", 3))
            .unwrap();
        db.add_member(&create_a_unique_person("MemberName4", 4))
            .unwrap();
        db.add_provider(&create_a_unique_person("ProviderName1", 61))
            .unwrap();
        db.add_provider(&create_a_unique_person("ProviderName2", 62))
            .unwrap();
        db.add_consultation_record(&create_a_unique_consultation(1, 61))
            .unwrap();
        db.add_consultation_record(&create_a_unique_consultation(2, 61))
            .unwrap();
        db.add_consultation_record(&create_a_unique_consultation(2, 61))
            .unwrap();
        db.add_consultation_record(&create_a_unique_consultation(3, 61))
            .unwrap();
        db.add_consultation_record(&create_a_unique_consultation(3, 62))
            .unwrap();
        db.add_consultation_record(&create_a_unique_consultation(3, 62))
            .unwrap();
        match db.send_provider_reports() {
            Ok(_) => (),
            Err(err) => panic!("send_provider_reports() ERROR: {}", err),
        }
    }

    #[test]
    fn test_send_manager_report() {
        remove_test_db();
        let db = DB::new(TEST_DB_PATH).unwrap();

        db.add_consultation_record(&create_a_unique_consultation(1, 1))
            .unwrap();
        db.add_consultation_record(&create_a_unique_consultation(2, 2))
            .unwrap();
        db.add_consultation_record(&create_a_unique_consultation(3, 3))
            .unwrap();
        db.add_consultation_record(&create_a_unique_consultation(4, 4))
            .unwrap();
        db.add_consultation_record(&create_a_unique_consultation(5, 5))
            .unwrap();
        match db.send_manager_report() {
            Ok(_) => (),
            Err(err) => panic!("send_manager_report() ERROR: {}", err),
        }
    }

    #[test]
    fn test_send_provider_directory() {
        remove_test_db();
        let db: DB = DB::new(TEST_DB_PATH).unwrap();
        db.add_service(111112, "Therapy2", 20.99).unwrap();
        db.add_service(111111, "Therapy1", 10.99).unwrap();
        db.add_service(111113, "Therapy3", 30.99).unwrap();
        db.send_provider_directory("providername@pdx.edu").unwrap();
    }

    #[test]
    fn test_is_valid_member_id() {
        remove_test_db();
        let db: DB = DB::new(TEST_DB_PATH).unwrap();

        db.add_member(&create_a_unique_person("MemberName", 1))
            .unwrap();
        db.add_member(&create_a_unique_person("MemberName", 2))
            .unwrap();
        db.add_member(&create_a_unique_person("MemberName", 123456789))
            .unwrap();
        match db.is_valid_member_id(1) {
            Ok(valid) => {
                if !valid {
                    panic!("Id should be valid")
                }
            }
            Err(err) => panic!("is_valid_member_id() ERROR: {}", err),
        }
        match db.is_valid_member_id(2) {
            Ok(valid) => {
                if !valid {
                    panic!("Id should be valid")
                }
            }
            Err(err) => panic!("is_valid_member_id() ERROR: {}", err),
        }
        match db.is_valid_member_id(123456789) {
            Ok(valid) => {
                if !valid {
                    panic!("Id should be valid")
                }
            }
            Err(err) => panic!("is_valid_member_id() ERROR: {}", err),
        }
        match db.is_valid_member_id(666666666) {
            Ok(exists) => {
                if exists {
                    panic!("Member id should be invalid.");
                }
            }
            Err(_) => (),
        }
    }

    #[test]
    fn test_is_valid_provider_id() {
        remove_test_db();
        let db: DB = DB::new(TEST_DB_PATH).unwrap();

        db.add_provider(&create_a_unique_person("ProviderName", 1))
            .unwrap();
        db.add_provider(&create_a_unique_person("ProviderName", 2))
            .unwrap();
        db.add_provider(&create_a_unique_person("ProviderName", 123456789))
            .unwrap();
        match db.is_valid_provider_id(1) {
            Ok(valid) => {
                if !valid {
                    panic!("Id should be valid")
                }
            }
            Err(err) => panic!("is_valid_provider_id() ERROR: {}", err),
        }
        match db.is_valid_provider_id(2) {
            Ok(valid) => {
                if !valid {
                    panic!("Id should be valid")
                }
            }
            Err(err) => panic!("is_valid_provider_id() ERROR: {}", err),
        }
        match db.is_valid_provider_id(123456789) {
            Ok(valid) => {
                if !valid {
                    panic!("Id should be valid")
                }
            }
            Err(err) => panic!("is_valid_provider_id() ERROR: {}", err),
        }
        match db.is_valid_provider_id(666666666) {
            Ok(exists) => {
                if exists {
                    panic!("Provider id should be invalid.");
                }
            }
            Err(_) => (),
        }
    }

    #[test]
    fn test_is_valid_service_id() {
        remove_test_db();
        let db: DB = DB::new(TEST_DB_PATH).unwrap();

        db.add_service(1, "Therapy1", 99.99).unwrap();
        db.add_service(2, "Therapy2", 99.99).unwrap();
        db.add_service(123456, "Therapy3", 99.99).unwrap();
        match db.is_valid_service_id(1) {
            Ok(valid) => {
                if !valid {
                    panic!("Id should be valid")
                }
            }
            Err(err) => panic!("is_valid_service_id() ERROR: {}", err),
        }
        match db.is_valid_service_id(2) {
            Ok(valid) => {
                if !valid {
                    panic!("Id should be valid")
                }
            }
            Err(err) => panic!("is_valid_service_id() ERROR: {}", err),
        }
        match db.is_valid_service_id(123456) {
            Ok(valid) => {
                if !valid {
                    panic!("Id should be valid")
                }
            }
            Err(err) => panic!("is_valid_service_id() ERROR: {}", err),
        }
        match db.is_valid_service_id(666666666) {
            Ok(valid) => {
                if valid {
                    panic!("Id should be invalid.");
                }
            }
            Err(_) => (),
        }
    }

    #[test]
    fn test_add_member() {
        remove_test_db();
        let db: DB = DB::new(TEST_DB_PATH).unwrap();
        let person: PersonInfo = get_a_person();

        match db.add_member(&person) {
            Ok(_) => (),
            Err(err) => {
                panic!("add_member() ERROR: {}", err);
            }
        }
        match db.add_member(&person) {
            Ok(_) => panic!("Member should already exist and not be added."),
            Err(_) => (),
        }
    }

    #[test]
    fn test_add_provider() {
        remove_test_db();
        let db: DB = DB::new(TEST_DB_PATH).unwrap();
        let person: PersonInfo = get_a_person();

        match db.add_provider(&person) {
            Ok(_) => (),
            Err(err) => {
                panic!("add_provider() ERROR: {}", err);
            }
        }
        match db.add_provider(&person) {
            Ok(_) => panic!("Provider should already exist and not be added."),
            Err(_) => (),
        }
    }

    #[test]
    fn test_remove_member() {
        remove_test_db();
        let db: DB = DB::new(TEST_DB_PATH).unwrap();

        match db.remove_member(123456789) {
            Ok(_) => {
                panic!("Member should not exist and not be able to be removed.")
            }
            Err(_) => (),
        }
        db.add_member(&create_a_unique_person("MemberName", 123456789))
            .unwrap();
        match db.remove_member(123456789) {
            Ok(_) => (),
            Err(err) => panic!("remove_member() ERROR: {}", err),
        }
    }

    #[test]
    fn test_remove_provider() {
        remove_test_db();
        let db: DB = DB::new(TEST_DB_PATH).unwrap();

        match db.remove_provider(123456789) {
            Ok(_) => {
                panic!(
                    "Provider should not exist and not be able to be removed."
                )
            }
            Err(_) => (),
        }
        db.add_provider(&create_a_unique_person("ProviderName", 123456789))
            .unwrap();
        match db.remove_provider(123456789) {
            Ok(_) => (),
            Err(err) => panic!("remove_provider() ERROR: {}", err),
        }
    }

    #[test]
    fn test_add_consultation_record() {
        remove_test_db();
        let db: DB = DB::new(TEST_DB_PATH).unwrap();
        let consul: Consultation = get_a_consultation();

        match db.add_consultation_record(&consul) {
            Ok(_) => (),
            Err(err) => {
                panic!("add_consultation_record() ERROR: {}", err);
            }
        }
    }

    #[test]
    fn test_add_service() {
        remove_test_db();
        let db: DB = DB::new(TEST_DB_PATH).unwrap();

        db.add_service(123456, "Service1", 99.99).unwrap();
        match db.add_service(123456, "Serv", 99.99) {
            Ok(_) => panic!("Error expected for duplicate ID."),
            Err(_) => (),
        }
        match db.add_service(222222, "", 99.99) {
            Ok(_) => panic!("Error expected for empty name."),
            Err(_) => (),
        }
    }

    #[test]
    fn test_get_service_name() {
        remove_test_db();
        let db: DB = DB::new(TEST_DB_PATH).unwrap();

        match db.get_service_name(123456) {
            Ok(_) => panic!("Error expected on empty database"),
            Err(_) => (),
        }
        db.add_service(123456, "Service1", 99.99).unwrap();
        let name = db.get_service_name(123456).unwrap();
        if name != "Service1" {
            panic!("Name should match for retrieved name.");
        }
    }

    #[test]
    fn test_get_member_info() {
        remove_test_db();
        let db: DB = DB::new(TEST_DB_PATH).unwrap();

        match db.add_member(&create_a_unique_person("PersonName", 123456789)) {
            Ok(_) => (),
            Err(err) => panic!("add_member() ERROR: {}", err),
        }

        match db.get_member_info(123456789) {
            Ok(info) => {
                assert_eq!(info.name, "PersonName");
                assert_eq!(info.id, 123456789);
            }
            Err(err) => panic!("get_member_info() ERROR: {}", err),
        }

        match db.get_member_info(777777777) {
            Ok(_) => panic!(
                "Member info should give error when member does not exist"
            ),
            Err(_) => (),
        }
    }

    #[test]
    fn test_get_provider_info() {
        remove_test_db();
        let db: DB = DB::new(TEST_DB_PATH).unwrap();

        match db.add_provider(&create_a_unique_person("PersonName", 123456789))
        {
            Ok(_) => (),
            Err(err) => panic!("add_provider() ERROR: {}", err),
        }

        match db.get_provider_info(123456789) {
            Ok(info) => {
                assert_eq!(info.name, "PersonName");
                assert_eq!(info.id, 123456789);
            }
            Err(err) => panic!("get_provider_info() ERROR: {}", err),
        }

        match db.get_provider_info(777777777) {
            Ok(_) => panic!(
                "Provider info should give error when member does not exist"
            ),
            Err(_) => (),
        }
    }
}
