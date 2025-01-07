use std::{default, env, result};

use rocket::futures::future::ok;

pub struct ServiceConfig {
    pub auth: String,
    pub activity: String,
    pub ai: String,
    pub content: String,
    pub logging: String,
    pub live_chat: String,
    pub notification: String,
    pub plan: String,
    pub user: String,
    auth_ip: String,
    auth_port: String,
    activity_ip: String,
    activity_port: String,
    ai_ip: String,
    ai_port: String,
    content_ip: String,
    content_port: String,
    logging_ip: String,
    logging_port: String,
    live_chat_ip: String,
    live_chat_port: String,
    notification_ip: String,
    notification_port: String,
    plan_ip: String,
    plan_port: String,
    user_ip: String,
    user_port: String,
}

pub fn get_var_value(key: String) -> String {
    match env::var(key){
        Ok(val) => return val,
        Err(e) => unimplemented!("{}", e)
    }
}

pub fn get_service_config() -> ServiceConfig {
    ServiceConfig {
        auth_ip: get_var_value("AUTH_IP".to_string()),
        auth_port: get_var_value("AUTH_PORT".to_string()),
        auth: format!("{}:{}/auth", auth_ip, auth_port),

        activity_ip: get_var_value("ACTIVITY_IP".to_string()),
        activity_port: get_var_value("ACTIVITY_PORT".to_string()),
        activity: format!("{}:{}/activity", activity_ip, activity_port),

        ai_ip: get_var_value("AI_IP".to_string()),
        ai_port: get_var_value("AI_PORT".to_string()),
        ai: format!("{}:{}/ai", ai_ip, ai_port),

        content_ip: get_var_value("CONTENT_IP".to_string()),
        content_port: get_var_value("CONTENT_PORT".to_string()),
        content: format!("{}:{}/content", content_ip, content_port),

        logging_ip: get_var_value("LOGGING_IP".to_string()),
        logging_port: get_var_value("LOGGING_PORT".to_string()),
        logging: format!("{}:{}/logging", logging_ip, logging_port),

        live_chat_ip: get_var_value("LIVE_CHAT_IP".to_string()),
        live_chat_port: get_var_value("LIVE_CHAT_PORT".to_string()),
        live_chat: format!("{}:{}/liveChat", live_chat_ip, live_chat_port),

        notification_ip: get_var_value("NOTIFICATION_IP".to_string()),
        notification_port: get_var_value("NOTIFICATION_PORT".to_string()),
        notification: format!("{}:{}/notification", notification_ip, notification_port),

        plan_ip: get_var_value("PLAN_IP".to_string()),
        plan_port: get_var_value("PLAN_PORT".to_string()),
        plan: format!("{}:{}/plan", plan_ip, plan_port),

        user_ip: get_var_value("USER_IP".to_string()),
        user_port: get_var_value("USER_PORT".to_string()),
        user: format!("{}:{}/user", user_ip, user_port),
    }
}
