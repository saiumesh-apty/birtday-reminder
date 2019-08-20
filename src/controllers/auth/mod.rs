use actix_web::{Scope, web};

pub mod login;
use login::{ login, update_password };

pub fn auth_routes() -> Scope {
    web::scope("/auth")
        .service(web::resource("/login").route(web::post().to(login)))
        .service(web::resource("/update_password").route(web::post().to(update_password)))
}