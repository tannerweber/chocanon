use chocanon::esend::{
    send_manager_report, send_member_report, send_provider_report,
};
use std::fs;

#[test]
fn test_member_email_file_exists() {
    let recipient_name = "test_member_email";
    let email_directory = "./emails/member";

    match send_member_report(
        "test_to",
        "test_from",
        "test_subject",
        "test_body",
        recipient_name,
    ) {
        Ok(_) => (),
        Err(err) => panic!("Report could not send: {}", err),
    };

    assert!(found_and_removed_email(email_directory, recipient_name));

    assert!(!found_and_removed_email(email_directory, recipient_name));
}

#[test]
fn test_provider_email_file_exists() {
    let recipient_name = "test_provider_email";
    let email_directory = "./emails/provider";

    match send_provider_report(
        "test_to",
        "test_from",
        "test_subject",
        "test_body",
        recipient_name,
    ) {
        Ok(_) => (),
        Err(err) => panic!("Report could not send: {}", err),
    };

    assert!(found_and_removed_email(email_directory, recipient_name));

    assert!(!found_and_removed_email(email_directory, recipient_name));
}

#[test]
fn test_manager_email_file_exists() {
    let recipient_name = "test_manager_email";
    let email_directory = "./emails/manager";

    match send_manager_report(
        "test_to",
        "test_from",
        "test_subject",
        "test_body",
        recipient_name,
    ) {
        Ok(_) => (),
        Err(err) => panic!("Report could not send: {}", err),
    };

    assert!(found_and_removed_email(email_directory, recipient_name));

    assert!(!found_and_removed_email(email_directory, recipient_name));
}

fn found_and_removed_email(
    email_directory: &str,
    recipient_name: &str,
) -> bool {
    let mut found = false;
    for email in fs::read_dir(email_directory).unwrap() {
        let file_name = email.unwrap().file_name().into_string().unwrap();

        if file_name.contains(recipient_name) {
            found = true;

            let file_path = format!("{}/{}", email_directory, file_name);

            match std::fs::remove_file(file_path) {
                Ok(_) => (),
                Err(err) => panic!("Could not remove test file: {}", err),
            };
        }
    }

    return found;
}
