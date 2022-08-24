use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct UserData {
    pub user_id: String,
    pub last_context: String,
    pub last_question_type: String,
    pub last_question_index: i32,
}

impl UserData {
    pub fn new(user_id: String) -> UserData {
        UserData {
            user_id,
            last_context: "".to_string(),
            last_question_type: "".to_string(),
            last_question_index: -1,
        }
    }
}

// CREATE TABLE user_data (
//     user_id VARCHAR(255) NOT NULL,
//     last_context VARCHAR(255) NOT NULL,
//     last_question_index INT NOT NULL,
//     last_question_type VARCHAR(255) NOT NULL,
//     PRIMARY KEY (user_id)
// );

// INSERT INTO user_data (user_id, last_context, last_question_type, last_question_index) VALUES ("1", "last_context", "last_question_type", 0);

// UPDATE user_data SET last_context = 'test_context', last_question_index = -1 WHERE user_id = 1;