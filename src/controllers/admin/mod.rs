use actix_web::{Scope, web};

mod user;
use user::{add_user, add_dob, update_dob};

pub fn admin_routes() -> Scope {
    web::scope("/admin")
        .service(web::resource("/add_user").route(web::post().to(add_user)))
        .service(web::resource("/dob").route(web::post().to(add_dob)))
        .service(web::resource("/dob_update").route(web::put().to(update_dob)))
}