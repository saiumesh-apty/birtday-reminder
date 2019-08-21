#[macro_use(lazy_static)]
extern crate lazy_static;

use actix_web::{ App, HttpServer, web::{Data}, http::header };
use actix_files::Files;
use job_scheduler::{JobScheduler, Job};
use std::time::Duration;
use std::thread;
use std::{env};
use actix_cors::Cors;

mod controllers;
use controllers::{ auth, admin };

// database
mod database;
use database::postgres::{ connect_postgres };

// types
mod types;
mod helpers;

// email
mod email;

// models
mod models;

use crate::database::postgres::get_instance;
use crate::models::dob::{ process_emails };


fn get_port() -> String {
    let port = match env::var("PORT") {
        Ok(port) => port,
        Err(_) => String::from("3000")
    };
    format!(":{}",port)
}


fn main() -> std::io::Result<()> {

    let address = format!("0.0.0.0{}", get_port());

    connect_postgres();
    let pg_instance = get_instance();

    thread::spawn(move || {
        let mut sched = JobScheduler::new();
        sched.add(Job::new("* * 21 * * *".parse().unwrap(), move || {
            process_emails(pg_instance.clone());
        }));
        loop {
            sched.tick();
            std::thread::sleep(Duration::from_millis(500));
        }
    });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:3001")
                    .allowed_methods(vec!["GET", "POST", "PUT"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
            )
            .register_data(Data::new(get_instance()))
            .service(auth::auth_routes())
            .service(admin::admin_routes())
            .service(Files::new("/", "client/build").index_file("index.html"))
    }).bind(address)?.run()
}
