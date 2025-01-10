use std::path::PathBuf;
use rocket::http::Status;
use rocket::request::{Request, FromRequest, Outcome};
use rocket::serde::json::{Json, Value};
use reqwest::Client;
use serde::de::value;
use crate::auth::validate_jwt;
use crate::services::{get_service_config, ServiceConfig};
use serde::Deserialize;    

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub sub: String,
}

#[derive(Deserialize)]
pub struct SimpleJson {
    key: Value,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(auth_header) = request.headers().get_one("Authorization") {
            if let Some(token) = auth_header.strip_prefix("Bearer ") {
                match validate_jwt(token) {
                    Ok(sub) => Outcome::Success(AuthenticatedUser { sub }),
                    Err(_) => Outcome::Error((Status::Unauthorized, ())),
                }
            } else {
                Outcome::Error((Status::Unauthorized, ()))
            }
        } else {
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
    if(path_str.len()>0){
        let url = match service_name.as_str() {
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
    if(path_str.len()>0){
        let url = match service_name.as_str() {
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
    }
    let client = Client::new();
    let response = client
        .post(&url)
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
    if(path_str.len()>0){
        let url = match service_name.as_str() {
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
    }
    let client = Client::new();
    let response = client
        .put(&url)
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
    if(path_str.len()>0){
        let url = match service_name.as_str() {
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
