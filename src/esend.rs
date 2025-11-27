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

/// The various paths in the file system where emails shall be written.
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

/// Sends an email for a provider report.
///
/// # Arguments
///
/// * `to` - The email address of the recipient.
/// * `from` - The email address of the sender (ChocAn).
/// * `subject` - The subject line of the email.
/// * `body` - The entire body of the email.
/// * `recipient_name` - The name of the recipient.
///
/// # Failure
///
/// Will return `Err` for IO errors.
pub fn send_provider_report(
    to: &str,
    from: &str,
    subject: &str,
    body: &str,
    recipient_name: &str,
) -> std::io::Result<()> {
    _ = send_email(
        to,
        from,
        subject,
        body,
        recipient_name,
        EmailPath::PROVIDER,
    );
    Ok(())
}

/// Sends an email for a member report.
///
/// # Arguments
///
/// * `to` - The email address of the recipient.
/// * `from` - The email address of the sender (ChocAn).
/// * `subject` - The subject line of the email.
/// * `body` - The entire body of the email.
/// * `recipient_name` - The name of the recipient.
///
/// # Failure
///
/// Will return `Err` for IO errors.
pub fn send_member_report(
    to: &str,
    from: &str,
    subject: &str,
    body: &str,
    recipient_name: &str,
) -> std::io::Result<()> {
    _ = send_email(to, from, subject, body, recipient_name, EmailPath::MEMBER);
    Ok(())
}

/// Sends an email for a manager report.
///
/// # Arguments
///
/// * `to` - The email address of the recipient.
/// * `from` - The email address of the sender (ChocAn).
/// * `subject` - The subject line of the email.
/// * `body` - The entire body of the email.
/// * `recipient_name` - The name of the recipient.
///
/// # Failure
///
/// Will return `Err` for IO errors.
pub fn send_manager_report(
    to: &str,
    from: &str,
    subject: &str,
    body: &str,
    recipient_name: &str,
) -> std::io::Result<()> {
    _ = send_email(to, from, subject, body, recipient_name, EmailPath::MANAGER);
    Ok(())
}

/// Sends an email for a provider directory.
///
/// # Arguments
///
/// * `to` - The email address of the recipient.
/// * `from` - The email address of the sender (ChocAn).
/// * `subject` - The subject line of the email.
/// * `body` - The entire body of the email.
/// * `recipient_name` - The name of the recipient.
///
/// # Failure
///
/// Will return `Err` for IO errors.
pub fn send_provider_directory(
    to: &str,
    from: &str,
    subject: &str,
    body: &str,
    recipient_name: &str,
) -> std::io::Result<()> {
    send_email(to, from, subject, body, recipient_name, EmailPath::PROVIDER)?;
    Ok(())
}

/// Writes out an email to the specified path.
///
/// # Arguments
///
/// * `to` - The email address of the recipient.
/// * `from` - The email address of the sender (ChocAn).
/// * `subject` - The subject line of the email.
/// * `body` - The entire body of the email.
/// * `recipient_name` - The name of the recipient.
/// * `path` - The path to send the file, not including the file name.
///
/// # Failure
///
/// Will return `Err` for IO errors.
fn send_email(
    to: &str,
    from: &str,
    subject: &str,
    body: &str,
    recipient_name: &str,
    path: &str,
) -> std::io::Result<()> {
    ensure_email_dir();

    let timestamp =
        chrono::Local::now().format("%m-%d-%Y_%H:%M:%S").to_string();
    let file_name = format!("{}/{}_{}.txt", path, recipient_name, timestamp);
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
            "Name",
            EmailPath::MEMBER,
        ) {
            Ok(_) => (),
            Err(err) => panic!("ERROR {}", err),
        }
    }

    #[test]
    fn test_send_member_report() {
        let _ =
            send_member_report("user", "chocanon", "matter", "testst", "God");

        let dir = std::path::Path::new(EmailPath::MEMBER);
        assert!(
            dir.read_dir().unwrap().next().is_some(),
            "send member report did not create a file"
        );
    }

    #[test]
    fn test_send_provider_report() {
        let _ = send_provider_report(
            "user1", "Chocanon", "consults", "do this", "You",
        );

        let dir = std::path::Path::new(EmailPath::PROVIDER);
        assert!(
            dir.read_dir().unwrap().next().is_some(),
            "send provider report did not create a file"
        );
    }

    #[test]
    fn test_send_manager_report() {
        let _ = send_manager_report(
            "Provider",
            "Chocanon",
            "Serious",
            "Office NOW!",
            "You",
        );

        let dir = std::path::Path::new(EmailPath::MANAGER);
        assert!(
            dir.read_dir().unwrap().next().is_some(),
            "send manager report did not create a file"
        );
    }

    #[test]
    fn test_send_provider_directory() {
        let _ = send_provider_directory(
            "user1", "Chocanon", "consults", "do this", "You",
        );

        let dir = std::path::Path::new(EmailPath::PROVIDER);
        assert!(
            dir.read_dir().unwrap().next().is_some(),
            "send provider directory did not create a file"
        );
    }
}
