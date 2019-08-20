use actix_web::{ HttpResponse };
use actix_web::web::{Data, Json};
// use chrono::{NaiveDate};
use crate::types::user::{ Login, Response, UpdatePassword };
use crate::types::postgres_types::{ Postgres };
use crate::helpers::bcrypt_helper::{verify_password, hash_password};

pub fn login(login: Json<Login>, postgres: Data<Postgres>) -> HttpResponse {
   match postgres.lock() {
       Ok(db) => {
           let email = db.query("SELECT email, password FROM users WHERE email = $1", &[&login.email]);
           match email {
               Ok(email_data) => {
                   if email_data.is_empty() {
                       return HttpResponse::BadRequest().json(Response{
                           message: String::from("email or password is wrong")
                       });
                   }
                   let mut hashed_password: String = String::default();
                   for user in email_data.iter() {
                       hashed_password = user.get("password");
                       break;
                   }
                   if verify_password(&login.password, &hashed_password) {
                       return HttpResponse::Ok().json({});
                   }
                   HttpResponse::BadRequest().json(Response{
                       message: String::from("email or password is wrong")
                   })
               },
               Err(err) => {
                   println!("error while getting user {:?}", err);
                   HttpResponse::InternalServerError().body("error while getting user")
               }
           }
       }
       Err(err) => {
           println!("error while lock {:?}", err);
           HttpResponse::InternalServerError().body("get error")
       }
   }
}


pub fn update_password(input: Json<UpdatePassword>, postgres: Data<Postgres>) -> HttpResponse {
    // first check user is exists or not
    let pg_instance = postgres.lock().unwrap();
    let user = pg_instance.query("SELECT id FROM users WHERE id = $1", &[&input.id]).unwrap();
    if user.is_empty() {
        return HttpResponse::BadRequest().json(Response {
            message: "hey!! looks like user id wrong".to_string(),
        });
    }
    // hash password
    let hashed_password = hash_password(&input.password);
    // now update password
    let _password = pg_instance.query("UPDATE users SET password = $1 WHERE id = $2",
                                        &[&hashed_password, &input.id]).unwrap();
    HttpResponse::Ok().json({})

}

// pub fn check_if_birthday(postgres: Data<Postgres>) -> HttpResponse {
//     let result = postgres.lock().unwrap().query("SELECT *
// from user_dob WHERE
//     DATE_PART('day', dob) = date_part('day', CURRENT_DATE + 1)
// AND
//     DATE_PART('month', dob) = date_part('month', CURRENT_DATE)", &[]).unwrap();
//     println!("result {:?}", result);

//     HttpResponse::Ok().body("")
// }