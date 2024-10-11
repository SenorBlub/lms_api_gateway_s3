pub struct ServiceConfig {
    pub auth: String,
    pub activity: String,
    pub ai: String,
    pub content: String,
    pub logging: String,
    pub live_chat: String,
    pub notification: String,
    pub plan: String,
}

pub fn get_service_config() -> ServiceConfig {
    ServiceConfig {
        auth: "http://127.0.0.1:9001".to_string(),  
        activity: "http://127.0.0.1:9002".to_string(),
        ai: "http://127.0.0.1:9003".to_string(),
        content: "http://127.0.0.1:9004".to_string(),
        logging: "http://127.0.0.1:9005".to_string(),
        live_chat: "http://127.0.0.1:9006".to_string(),
        notification: "http://127.0.0.1:9007".to_string(),
        plan: "http://127.0.0.1:9008".to_string(),
    }
}
