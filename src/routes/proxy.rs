use std::path::PathBuf;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::{Json, Value};
use reqwest::Client;
use crate::auth::validate_jwt;
use crate::services::{get_service_config, ServiceConfig};
use serde::Deserialize;
use rocket::{fairing::{Fairing, Info, Kind}, Request, Response};


#[derive(Debug)]
pub struct AuthenticatedUser {
    pub sub: String,
}

#[derive(Deserialize)]
pub struct SimpleJson {
    key: Value,
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(rocket::http::Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(rocket::http::Header::new("Access-Control-Allow-Methods", "GET, POST, OPTIONS, PUT, DELETE"));
        response.set_header(rocket::http::Header::new("Access-Control-Allow-Headers", "Authorization, Content-Type"));
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(auth_header) = request.headers().get_one("Authorization") {
            print!("{}", auth_header);
            if let Some(token) = auth_header.strip_prefix("Bearer ") {
                print!("{}", token);
                match validate_jwt(token) {
                    Ok(sub) => Outcome::Success(AuthenticatedUser { sub }),
                    Err(_) => {println!("JWT validation error");
                     Outcome::Error((Status::Unauthorized, ()))},
                }
            } else {
                println!("Invalid Authorization header format.");
                Outcome::Error((Status::Unauthorized, ()))
            }
        } else {
            println!("Missing Authorization header.");
            Outcome::Error((Status::Unauthorized, ()))
        }
    }
}

//not secure
#[post("/<service_name>/authorize", format = "application/json", data = "<data>")]
pub async fn authorize_user(service_name: String, data: Json<SimpleJson>) -> Result<String, Status> {
    let config: ServiceConfig = get_service_config();
    let url = match service_name.as_str() {
        "auth" => format!("{}/authorize", config.auth),
        _ => return Err(Status::NotFound),
    };

    print!("{}", url.as_str());
    let client = Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&data.key) // make this the data for the auth service so the user can authenticate
        .send()
        .await
        .map_err(|_| Status::BadGateway)?
        .text()
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(response)
}

#[post("/<service_name>/email-authorize", format = "application/json", data = "<data>")]
pub async fn email_authorize_user(service_name: String, data: Json<SimpleJson>) -> Result<String, Status> {
    let config: ServiceConfig = get_service_config();
    let url = match service_name.as_str() {
        "auth" => format!("{}/email-authorize", config.auth),
        _ => return Err(Status::NotFound),
    };

    print!("{}", url.as_str());
    let client = Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&data.key) // make this the data for the auth service so the user can authenticate
        .send()
        .await
        .map_err(|_| Status::BadGateway)?
        .text()
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(response)
}

#[post("/<service_name>/register", format = "application/json", data = "<data>")]
pub async fn create_register_user(service_name: String, data: Json<SimpleJson>) -> Result<String, Status> {
    let config: ServiceConfig = get_service_config();
    let url = match service_name.as_str() {
        "auth" => format!("{}/register", config.auth),
        _ => return Err(Status::NotFound),
    };

    print!("{}", url.as_str());
    let client = Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&data.key) // make this the data for the auth service so the user can authenticate
        .send()
        .await
        .map_err(|_| Status::BadGateway)?
        .text()
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(response)
}

#[post("/<service_name>/login", format = "application/json", data = "<data>")]
pub async fn login_user(service_name: String, data: Json<SimpleJson>) -> Result<String, Status> {
    let config: ServiceConfig = get_service_config();
    let url = match service_name.as_str() {
        "user" => format!("{}/login", config.user),
        _ => return Err(Status::NotFound),
    };

    print!("{}", url.as_str());
    let client = Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&data.key) // make this the data for the auth service so the user can authenticate
        .send()
        .await
        .map_err(|_| Status::BadGateway)?
        .text()
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(response)
}

#[get("/<service_name>/<path..>")]
pub async fn handle_get(service_name: String, path: PathBuf, _user: AuthenticatedUser) -> Result<String, Status> {
    let path_str = path.to_str().ok_or(Status::BadRequest)?;
    let config = get_service_config();
    let url;
    if(path_str.len()>0){
        url = match service_name.as_str() {
            "auth" => format!("{}/{}", config.auth, path_str),
            "activity" => format!("{}/{}", config.activity, path_str),
            "ai" => format!("{}/{}", config.ai, path_str),
            "content" => format!("{}/{}", config.content, path_str),
            "logging" => format!("{}/{}", config.logging, path_str),
            "live_chat" => format!("{}/{}", config.live_chat, path_str),
            "notification" => format!("{}/{}", config.notification, path_str),  
            "plan" => format!("{}/{}", config.plan, path_str),
            "user" => format!("{}/{}", config.user, path_str),
            _ => return Err(Status::NotFound),
        };
    }else{
        url = match service_name.as_str() {
            "auth" => format!("{}", config.auth),
            "activity" => format!("{}", config.activity),
            "ai" => format!("{}", config.ai),
            "content" => format!("{}", config.content),
            "logging" => format!("{}", config.logging),
            "live_chat" => format!("{}", config.live_chat),
            "notification" => format!("{}", config.notification),  
            "plan" => format!("{}", config.plan),
            "user" => format!("{}", config.user),
            _ => return Err(Status::NotFound),
        };
    }

    let client = Client::new();
    let response = client.get(&url)
        .send().await
        .map_err(|_| Status::InternalServerError)?
        .text().await
        .map_err(|_| Status::InternalServerError)?;

    Ok(response)
}

#[post("/<service_name>/<path..>", format = "application/json", data = "<data>")]
pub async fn handle_post(service_name: String,
    path: PathBuf,
    _user: AuthenticatedUser,
    data: Json<SimpleJson>,
) -> Result<String, Status> {
    let path_str = path.to_str().ok_or(Status::BadRequest)?;
    let config = get_service_config();
    let url;
    if(path_str.len()>0){
        url = match service_name.as_str() {
            "auth" => format!("{}/{}", config.auth, path_str),
            "activity" => format!("{}/{}", config.activity, path_str),
            "ai" => format!("{}/{}", config.ai, path_str),
            "content" => format!("{}/{}", config.content, path_str),
            "logging" => format!("{}/{}", config.logging, path_str),
            "live_chat" => format!("{}/{}", config.live_chat, path_str),
            "notification" => format!("{}/{}", config.notification, path_str),  
            "plan" => format!("{}/{}", config.plan, path_str),
            "user" => format!("{}/{}", config.user, path_str),
            _ => return Err(Status::NotFound),
        };
    }else{
        url = match service_name.as_str() {
            "auth" => format!("{}", config.auth),
            "activity" => format!("{}", config.activity),
            "ai" => format!("{}", config.ai),
            "content" => format!("{}", config.content),
            "logging" => format!("{}", config.logging),
            "live_chat" => format!("{}", config.live_chat),
            "notification" => format!("{}", config.notification),  
            "plan" => format!("{}", config.plan),
            "user" => format!("{}", config.user),
            _ => return Err(Status::NotFound),
        };
    }

    let client = Client::new();
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&data.key) // instead of key, make this actual request data
        .send()
        .await
        .map_err(|_| Status::BadGateway)?
        .text()
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(response)
}

#[put("/<service_name>/<path..>", format = "application/json", data = "<data>")]
pub async fn handle_put(service_name: String, path: PathBuf, _user: AuthenticatedUser, data: Json<SimpleJson>) -> Result<String, Status> {
    let path_str = path.to_str().ok_or(Status::BadRequest)?;
    let config = get_service_config();
    let url;
    if(path_str.len()>0){
        url = match service_name.as_str() {
            "auth" => format!("{}/{}", config.auth, path_str),
            "activity" => format!("{}/{}", config.activity, path_str),
            "ai" => format!("{}/{}", config.ai, path_str),
            "content" => format!("{}/{}", config.content, path_str),
            "logging" => format!("{}/{}", config.logging, path_str),
            "live_chat" => format!("{}/{}", config.live_chat, path_str),
            "notification" => format!("{}/{}", config.notification, path_str),  
            "plan" => format!("{}/{}", config.plan, path_str),
            "user" => format!("{}/{}", config.user, path_str),
            _ => return Err(Status::NotFound),
        };
    }else{
        url = match service_name.as_str() {
            "auth" => format!("{}", config.auth),
            "activity" => format!("{}", config.activity),
            "ai" => format!("{}", config.ai),
            "content" => format!("{}", config.content),
            "logging" => format!("{}", config.logging),
            "live_chat" => format!("{}", config.live_chat),
            "notification" => format!("{}", config.notification),  
            "plan" => format!("{}", config.plan),
            "user" => format!("{}", config.user),
            _ => return Err(Status::NotFound),
        };
    }

    let client = Client::new();
    let response = client
        .put(&url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&data.key)  // instead of key, make this actual request data
        .send()
        .await
        .map_err(|_| Status::BadGateway)?
        .text()
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(response)
}

#[delete("/<service_name>/<path..>")]
pub async fn handle_delete(service_name: String, path: PathBuf, _user: AuthenticatedUser) -> Result<String, Status> {
    let path_str = path.to_str().ok_or(Status::BadRequest)?;
    let config = get_service_config();
    let url;
    if(path_str.len()>0){
        url = match service_name.as_str() {
            "auth" => format!("{}/{}", config.auth, path_str),
            "activity" => format!("{}/{}", config.activity, path_str),
            "ai" => format!("{}/{}", config.ai, path_str),
            "content" => format!("{}/{}", config.content, path_str),
            "logging" => format!("{}/{}", config.logging, path_str),
            "live_chat" => format!("{}/{}", config.live_chat, path_str),
            "notification" => format!("{}/{}", config.notification, path_str),  
            "plan" => format!("{}/{}", config.plan, path_str),
            "user" => format!("{}/{}", config.user, path_str),
            _ => return Err(Status::NotFound),
        };
    }else{
        url = match service_name.as_str() {
            "auth" => format!("{}", config.auth),
            "activity" => format!("{}", config.activity),
            "ai" => format!("{}", config.ai),
            "content" => format!("{}", config.content),
            "logging" => format!("{}", config.logging),
            "live_chat" => format!("{}", config.live_chat),
            "notification" => format!("{}", config.notification),  
            "plan" => format!("{}", config.plan),
            "user" => format!("{}", config.user),
            _ => return Err(Status::NotFound),
        };
    }
    
    let client = Client::new();
    let response = client
        .delete(&url)
        .send()
        .await
        .map_err(|_| Status::BadGateway)?
        .text()
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(response)
}
