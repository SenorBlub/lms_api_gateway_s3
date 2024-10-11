#[macro_use] extern crate rocket;

mod routes;
mod auth;
mod services;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![routes::proxy::handle_get, routes::proxy::handle_post, routes::proxy::handle_delete, routes::proxy::handle_put, routes::proxy::authorize_user])
}
