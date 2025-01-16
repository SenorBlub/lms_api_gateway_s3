#[macro_use] extern crate rocket;

mod routes;
mod auth;
mod services;

use routes::proxy::*;
use crate::CORS;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(rocket::Config {
            address: std::net::Ipv4Addr::UNSPECIFIED.into(),
            port: 8080,
            ..rocket::Config::default()
        })
        .attach(CORS)
        .mount("/", 
            routes![
                routes::proxy::handle_get, 
                routes::proxy::handle_post, 
                routes::proxy::handle_delete, 
                routes::proxy::handle_put, 
                routes::proxy::authorize_user,
                routes::proxy::email_authorize_user,
                routes::proxy::create_register_user,
                routes::proxy::login_user
            ]
        )
}
