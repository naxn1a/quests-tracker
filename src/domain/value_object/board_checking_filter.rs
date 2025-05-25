use super::quest_statuses::QuestStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BoardCheckingFilter {
    pub name: Option<String>,
    pub status: Option<QuestStatus>,
}
