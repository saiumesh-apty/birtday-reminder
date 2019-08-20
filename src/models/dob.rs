use crate::email::send_birthday_reminder_email;
use crate::types::postgres_types::Postgres;
use crate::types::user::User;
use std::thread;

pub fn process_emails(pg_instance: Postgres) {
    // check did we sent today already or not
    let todays_rows = pg_instance
        .lock()
        .unwrap()
        .query(
            "SELECT * FROM email_sent WHERE DATE_PART('day', sent_on) = date_part('day', CURRENT_DATE)
            AND DATE_PART('month', sent_on) = date_part('month', CURRENT_DATE)",
            &[],
        )
        .unwrap();
    if !todays_rows.is_empty() {
        println!("we already sent today!!");
        return;
    }

    // first get all the emails
    let rows = pg_instance
        .lock()
        .unwrap()
        .query("select id, email, password from users", &[])
        .unwrap();
    if rows.is_empty() {
        return;
    }
    let mut users: Vec<User> = vec![];
    for row in rows.iter() {
        users.push(User {
            id: row.get("id"),
            email: row.get("email"),
            password: row.get("password"),
        });
    }
    // get birthday details
    let birthday_rows = pg_instance
        .lock()
        .unwrap()
        .query(
            "SELECT user_id FROM user_dob WHERE
                                DATE_PART('day', dob) = date_part('day', CURRENT_DATE + 1)
                                AND DATE_PART('month', dob) = date_part('month', CURRENT_DATE)",
            &[],
        )
        .unwrap();
    if birthday_rows.is_empty() {
        return;
    }
    let mut birthday_ids: Vec<i32> = vec![];
    for row in birthday_rows.iter() {
        birthday_ids.push(row.get("user_id"));
    }

    // now get all users expect
    let mut emails: Vec<String> = vec![];
    for user in users {
        if !birthday_ids.contains(&user.id) {
            emails.push(user.email);
        }
    }
    if emails.len() == 0 {
        return;
    }
    thread::spawn(move || {
        send_birthday_reminder_email(emails, pg_instance);
    });
}
