use actix_web::{HttpResponse};
use actix_web::web::{Data, Json};
use crate::types::postgres_types::Postgres;
use crate::types::user::{AddUser, Response, DOB};
use crate::helpers::bcrypt_helper::get_default_password;
use chrono::NaiveDate;

const FORMAT: &'static str = "%Y-%m-%d";

pub fn add_user(user:Json<AddUser>, postgres_instance: Data<Postgres>) -> HttpResponse {
    if user.secret != String::from("gauravrocks") {
        return HttpResponse::BadRequest().json(Response {
            message: String::from("wrong secret key")
        })
    }
    match postgres_instance.lock() {
        Ok(pg) => {
            let query = pg.query("SELECT EMAIL FROM users where EMAIL = $1", &[&user.email]);
            match query {
                Ok(users) => {
                    if !users.is_empty() {
                        return HttpResponse::BadRequest().json(Response {
                            message: String::from("user already exists")
                        });
                    }
                    let hashed_password = get_default_password();
                    let add_user = pg.query("INSERT INTO users (email, password) VALUES ($1, $2)", &[&user.email, &hashed_password]);
                    match add_user {
                        Ok(_) => {
                            HttpResponse::Ok().json({})
                        },
                        Err(err) => {
                            println!("unable insert {:?}", err);
                            HttpResponse::InternalServerError().body("error while adding user")
                        }
                    }
                },
                Err(err) => {
                    println!("unable to get user data {:?}", err);
                    HttpResponse::InternalServerError().body("error while grabbing")
                }
            }
        },
        Err(err) => {
            println!("error while grabbing lock on PG {:?}", err);
            HttpResponse::InternalServerError().body("error while grabbing")
        }
    }
}

pub fn add_dob(input: Json<DOB>, postgres: Data<Postgres>) -> HttpResponse {
    // first check user has already DOB or not
    match postgres.lock() {
        Ok(pg) => {
            let email = pg.query("SELECT id FROM user_dob WHERE user_id = $1", &[&input.user_id]);
            match email {
                Ok(data) => {
                    if data.is_empty() {
                        let date = NaiveDate::parse_from_str(input.dob.as_str(), FORMAT);
                        let insert = pg.query("INSERT INTO user_dob (user_id, dob) VALUES ($1, $2)", &[&input.user_id, &date.unwrap()]);
                        match insert {
                            Ok(_) => {
                                return HttpResponse::Ok().json({});
                            },
                            Err(err) => {
                                println!("unable insert {:?}", err);
                                return HttpResponse::InternalServerError().body("error while adding user");
                            }
                        }
                    }
                    HttpResponse::BadRequest().json(Response{
                        message: String::from("already exists")
                    })
                },
                Err(err) => {
                    println!("unable to get user data {:?}", err);
                    HttpResponse::InternalServerError().body("unable to get user data")
                }
            }
        },
        Err(err) => {
            println!("error while grabbing lock on PG {:?}", err);
            HttpResponse::InternalServerError().body("error while grabbing lock on PG")
        }
    }
}

pub fn update_dob(input: Json<DOB>, postgres: Data<Postgres>) -> HttpResponse {
    match postgres.lock() {
        Ok(pg) => {
            let email = pg.query("SELECT id FROM user_dob WHERE user_id = $1", &[&input.user_id]);
            match email {
                Ok(data) => {
                    if !data.is_empty() {
                        let date = NaiveDate::parse_from_str(input.dob.as_str(), FORMAT);
                        let insert = pg.query("UPDATE user_dob SET dob = $1 WHERE user_id = $2", &[&date.unwrap(), &input.user_id]);
                        match insert {
                            Ok(_) => {
                                return HttpResponse::Ok().json({});
                            },
                            Err(err) => {
                                println!("unable insert {:?}", err);
                                return HttpResponse::InternalServerError().body("error while updating");
                            }
                        }
                    }
                    HttpResponse::BadRequest().json(Response{
                        message: String::from("first insert then update")
                    })
                },
                Err(err) => {
                    println!("unable to get user data {:?}", err);
                    HttpResponse::InternalServerError().body("unable to get user data")
                }
            }
        },
        Err(err) => {
            println!("error while grabbing lock on PG {:?}", err);
            HttpResponse::InternalServerError().body("error while grabbing lock on PG")
        }
    }
}