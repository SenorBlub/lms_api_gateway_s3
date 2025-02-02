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
    let auth_ip = get_var_value("AUTH_IP".to_string());
    let auth_port = get_var_value("AUTH_PORT".to_string());
    let activity_ip = get_var_value("ACTIVITY_IP".to_string());
    let activity_port = get_var_value("ACTIVITY_PORT".to_string());
    let ai_ip = get_var_value("AI_IP".to_string());
    let ai_port = get_var_value("AI_PORT".to_string());
    let content_ip = get_var_value("CONTENT_IP".to_string());
    let content_port = get_var_value("CONTENT_PORT".to_string());
    let logging_ip = get_var_value("LOGGING_IP".to_string());
    let logging_port = get_var_value("LOGGING_PORT".to_string());
    let live_chat_ip = get_var_value("LIVE_CHAT_IP".to_string());
    let live_chat_port = get_var_value("LIVE_CHAT_PORT".to_string());
    let notification_ip = get_var_value("NOTIFICATION_IP".to_string());
    let notification_port = get_var_value("NOTIFICATION_PORT".to_string());
    let plan_ip = get_var_value("PLAN_IP".to_string());
    let plan_port = get_var_value("PLAN_PORT".to_string());
    let user_ip = get_var_value("USER_IP".to_string());
    let user_port = get_var_value("USER_PORT".to_string());

    ServiceConfig {
        auth_ip: auth_ip.clone(),
        auth_port: auth_port.clone(),
        activity_ip: activity_ip.clone(),
        activity_port: activity_port.clone(),
        ai_ip: ai_ip.clone(),
        ai_port: ai_port.clone(),
        content_ip: content_ip.clone(),
        content_port: content_port.clone(),
        logging_ip: logging_ip.clone(),
        logging_port: logging_port.clone(),
        live_chat_ip: live_chat_ip.clone(),
        live_chat_port: live_chat_port.clone(),
        notification_ip: notification_ip.clone(),
        notification_port: notification_port.clone(),
        plan_ip: plan_ip.clone(),
        plan_port: plan_port.clone(),
        user_ip: user_ip.clone(),
        user_port: user_port.clone(),
        auth: format!("{}:{}/auth/Auth", auth_ip, auth_port),
        activity: format!("{}:{}/activity/Activity", activity_ip, activity_port),
        ai: format!("{}:{}/ai", ai_ip, ai_port),
        content: format!("{}:{}/content/Content", content_ip, content_port),
        logging: format!("{}:{}/logging", logging_ip, logging_port),
        live_chat: format!("{}:{}/liveChat", live_chat_ip, live_chat_port),
        notification: format!("{}:{}/notification/Notification", notification_ip, notification_port),
        plan: format!("{}:{}/plan/Plan", plan_ip, plan_port),
        user: format!("{}:{}/user/User", user_ip, user_port),
    }
}

