use crate::types::postgres_types::Postgres;
use std::env;
use std::process::Command;

pub fn send_birthday_reminder_email(emails: Vec<String>, postgres: Postgres) {
    let cwd = env::current_dir().unwrap();
    let file_path = cwd.join("email").join("index.js");
    let command = Command::new("node")
        .arg(file_path)
        .arg(emails.join(","))
        .output();
    match command {
        Ok(output) => {
            if String::from_utf8(output.stderr.clone()).unwrap().len() != 0 {
                println!("error while sending emails {}", String::from_utf8(output.stderr).unwrap());
            } else {
                println!("{:?}", String::from_utf8(output.stdout).unwrap());
                postgres
                    .lock()
                    .unwrap()
                    .query(
                        "INSERT into EMAIL_SENT (sent_on) VALUES (NOW())",
                        &[],
                    )
                    .unwrap();
            }
        }
        Err(err) => {
            println!("failed to write command {:?}", err);
        }
    }
}
