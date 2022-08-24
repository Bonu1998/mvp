use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct SessionData {
    pub session_id: String,
    pub is_monetization_enabled: bool,
    pub is_display_enabled: bool,
    pub skill_name: String,
    pub source_type: String,
    pub content_token: String,
    pub is_new_session: bool,
    pub skill_type: String,
    pub locale: String,
    pub locale_content: String,
    pub device_size: String,
}


impl SessionData {
    pub fn new(session_id:String) -> SessionData {
        SessionData {
            session_id,
            is_monetization_enabled: false,
            is_display_enabled: false,
            skill_name: "".to_string(),
            source_type: "".to_string(),
            content_token: "".to_string(),
            is_new_session: false,
            skill_type: "".to_string(),
            locale: "".to_string(),
            locale_content: "".to_string(),
            device_size: "".to_string(),
        }
    }
}


// CREATE TABLE session_data(
//     session_id VARCHAR(255) NOT NULL,
//     is_monetization_enabled VARCHAR(255) NOT NULL,
//     is_display_enabled VARCHAR(255) NOT NULL,
//     skill_name VARCHAR(255) NOT NULL,
//     source_type VARCHAR(255) NOT NULL,
//     content_token VARCHAR(255) NOT NULL,
//     is_new_session VARCHAR(255) NOT NULL,
//     skill_type VARCHAR(255) NOT NULL,
//     locale VARCHAR(255) NOT NULL,
//     locale_content VARCHAR(255) NOT NULL,
//     device_size VARCHAR(255) NOT NULL,
//     PRIMARY KEY(session_id)
// );