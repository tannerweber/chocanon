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
//! Dats: October 29 to December 5
//! Course: CS 314, Fall 2025
//! Instructor: Christopher Gilmore

use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Person{
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn create_connection() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE person (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
        )",
        (), // empty list of parameters.
    )?;

    let new_person = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };

    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&new_person.name, &new_person.data),
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person?);
    }
    Ok(())
}
