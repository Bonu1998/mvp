use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct SkillConfig {
    pub supported_locale: Vec<String>,
    pub monetization_enabled: Vec<String>,
}
