use std::{collections::HashMap, env};
use flair_general_utils::{dao::MySqlDao, file_fetch::get_data};
use flair_types::skill::io::{
    BussinessInput, BussinessOutput, ResponseCommand, ResponseCommandType,
};
use log::{debug, error, info, warn};
use mysql::prelude::Queryable;
use serde_json::{json, Value as JsonValue};

use crate::models::{session_data::SessionData, skill_config::SkillConfig, user_data::UserData};

pub async fn get_session_data(input: BussinessInput, dao: &mut MySqlDao) -> SessionData {
    let mut session_data = SessionData::new(input.session_id.clone());
    session_data.source_type = input.request_type.clone();

    if input.request_type == "ALEXA".to_string() {
        match input.extras.clone() {
            Some(extras) => match extras.get("session_data") {
                Some(_data) => match serde_json::from_value::<SessionData>(_data.clone()) {
                    Ok(_d) => session_data = _d,
                    Err(e) => error!("\nParsing session_data: {} {}", e, _data),
                },
                None => {
                    debug!("\nsession_data not found {:?}", extras)
                }
            },
            None => {
                debug!("\nextras not found")
            }
        }
    } else {
        let query = format!(
            "SELECT * FROM sessiondata WHERE 'session_id' = '{}';",
            input.session_id
        );
        match dao.conn.query_first::<(
            String,
            bool,
            bool,
            String,
            String,
            String,
            bool,
            String,
            String,
            String,
            String,
        ), &str>(&query)
        {
            Ok(_data) => match _data {
                Some((
                    id,
                    is_monetization_enabled,
                    is_display_enabled,
                    skill_name,
                    source_type,
                    content_token,
                    is_new_session,
                    skill_type,
                    locale,
                    locale_content,
                    device_size,
                )) => {
                    session_data.session_id = id.clone();
                    session_data.is_monetization_enabled = is_monetization_enabled.clone();
                    session_data.is_display_enabled = is_display_enabled.clone();
                    session_data.skill_name = skill_name.clone();
                    session_data.source_type = source_type.clone();
                    session_data.content_token = content_token.clone();
                    session_data.is_new_session = is_new_session.clone();
                    session_data.skill_type = skill_type.clone();
                    session_data.locale = locale.clone();
                    session_data.locale_content = locale_content.clone();
                    session_data.device_size = device_size.clone();
                }
                None => {}
            },
            Err(e) => error!("\n{}", e),
        }
    }

    let mut path: String = "".to_string();
    match env::var("FILES_BASE_PATH") {
        Ok(_s) => path = _s,
        _ => warn!("File base path not available"),
    }

    session_data.locale = input.locale.clone();
    session_data.is_display_enabled = input.is_display_enabled.clone();
    session_data.is_new_session = input.is_new_session.clone();
    session_data.device_size = input.device_size.clone();

    for i in input.args.clone() {
        if let Some(id) = i.get("id") {
            if id == &"SKILL_INFO".to_string() {
                if let Some(content_token) = i.get("content_token") {
                    session_data.content_token = content_token.to_string()
                }
                if let Some(skill_type) = i.get("skill_type") {
                    session_data.skill_type = skill_type.to_string()
                }
                if let Some(skill_name) = i.get("skill_name") {
                    session_data.skill_name = skill_name.to_string()
                }
            }
        }
    }
    let skill_config_path = format!("{}/{}/skill_config.json", path, session_data.content_token);

    match get_data::<SkillConfig>(skill_config_path).await {
        Ok(skill_config) => {
            if skill_config
                .supported_locale
                .contains(&input.clone().locale)
            {
                session_data.locale_content = input.locale.clone()
            } else {
                session_data.locale_content = skill_config.supported_locale[0].clone()
            }

            if skill_config
                .monetization_enabled
                .contains(&input.clone().locale)
            {
                session_data.is_monetization_enabled = true
            } else {
                session_data.is_monetization_enabled = false
            }
        }
        Err(e) => error!("\n{}", e),
    }
    session_data
}

pub fn get_user_data(input: BussinessInput, dao: &mut MySqlDao) -> UserData {
    let mut data = UserData::new(input.user_id.clone());
    let query = format!(
        "SELECT * from user_data WHERE user_id = '{}';",
        input.user_id.clone()
    );
    match dao
        .conn
        .query_first::<(String, String, i32, String), &str>(&query)
    {
        Ok(_data) => match _data {
            Some((id, last_context, last_question_index, last_question_type)) => {
                data.user_id = id;
                data.last_context = last_context;
                data.last_question_index = last_question_index;
                data.last_question_type = last_question_type;
            }
            None => {
                debug!("\nUser data not found")
            }
        },
        Err(e) => error!("\n{}", e),
    }
    data
}

pub fn save_user_data(
    _input: BussinessInput,
    _resp: &mut BussinessOutput,
    user_data: UserData,
    dao: &mut MySqlDao,
) {
    let mut query = format!(
        "SELECT 1 FROM user_data WHERE user_id LIKE '{}';",
        user_data.user_id.clone()
    );
    match dao.conn.query::<i32, &str>(&query) {
        Ok(_rows) => {
            debug!("\n{:?}", _rows);
            if _rows.len() > 0 {
                query = format!("UPDATE user_data SET last_context='{}', last_question_type = '{}', last_question_index={} WHERE user_id = '{}';", user_data.last_context, user_data.last_question_type, user_data.last_question_index, user_data.user_id);
            } else {
                query = format!("INSERT INTO user_data (user_id, last_context, last_question_type, last_question_index) VALUES ('{}', '{}', '{}', {});",user_data.user_id, user_data.last_context, user_data.last_question_type, user_data.last_question_index);
            }
            match dao.conn.query_drop::<&str>(&query) {
                Ok(()) => {
                    info!("\nuserdata saved successfully");
                }
                Err(e) => {
                    error!("\nsave_user_data: {} {}", query, e);
                }
            }
        }
        Err(e) => {
            error!("\nsave_user_data: {} {}", query, e);
        }
    }
}

pub fn save_session_data(
    input: BussinessInput,
    resp: &mut BussinessOutput,
    session_data: SessionData,
    dao: &mut MySqlDao,
) {
    if input.request_type == "ALEXA".to_string() {
        let mut command =
            ResponseCommand::new(ResponseCommandType::SetSessionAttribute.to_string());
        let mut value: HashMap<String, JsonValue> = HashMap::new();
        value.insert("session_data".to_string(), json!(session_data));
        command.random = Some(value);
        if let Some(_commands) = &mut resp.commands {
            _commands.push(command);
        } else {
            resp.commands = Some(vec![command])
        }
    } else {
        let mut query = format!(
            "SELECT 1 FROM session_data WHERE session_id LIKE '{}';",
            session_data.session_id.clone()
        );
        match dao.conn.query::<i32, &str>(&query) {
            Ok(_rows) => {
                debug!("\n{:?}", _rows);
                if _rows.len() > 0 {
                    query = format!("UPDATE session_data SET is_monetization_enabled='{}', is_display_enabled = '{}', skill_name='{}',
                    source_type='{}',
                    content_token='{}', is_new_session='{}', skill_type='{}', locale='{}', locale_content='{}', device_size='{}'
                    WHERE session_id = '{}';",
                    session_data.is_monetization_enabled.to_string(),
                    session_data.is_display_enabled.to_string(),
                    session_data.skill_name,
                    session_data.source_type,
                    session_data.content_token,
                    session_data.is_new_session.to_string(),
                    session_data.skill_type,
                    session_data.locale,
                    session_data.locale_content,
                    session_data.device_size,
                    session_data.session_id
                    );
                } else {
                    query = format!("INSERT INTO session_data (session_id, is_monetization_enabled, is_display_enabled, skill_name, source_type, content_token, is_new_session, skill_type, locale, locale_content, device_size) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}');",
                    session_data.session_id,
                    session_data.is_monetization_enabled.to_string(),
                    session_data.is_display_enabled.to_string(),
                    session_data.skill_name,
                    session_data.source_type,
                    session_data.content_token,
                    session_data.is_new_session.to_string(),
                    session_data.skill_type,
                    session_data.locale,
                    session_data.locale_content,
                    session_data.device_size
                );
                }
                match dao.conn.query_drop::<&str>(&query) {
                    Ok(()) => {
                        info!("\nsessiondata saved successfully");
                    }
                    Err(e) => {
                        error!("\nsessiondata: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("\nsessiondata: {}", e);
            }
        }
    }
}
