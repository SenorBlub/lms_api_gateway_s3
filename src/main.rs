#[macro_use] extern crate rocket;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        // Set the required CORS headers
        res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        res.set_header(Header::new("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS"));
        res.set_header(Header::new("Access-Control-Allow-Headers", "Authorization, Content-Type"));
        res.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

        // Handle OPTIONS requests
        if req.method() == rocket::http::Method::Options {
            res.set_header(Header::new("Access-Control-Max-Age", "86400"));
        }
    }
}

#[rocket::options("/<path..>")]
fn options_handler(path: std::path::PathBuf) -> rocket::http::Status {
    rocket::http::Status::Ok
}


mod routes;
mod auth;
mod services;

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
                routes::proxy::login_user,
                options_handler
            ]
        )
}
