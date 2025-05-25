use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub enum QuestStatus {
    #[default]
    Open,
    InJourney,
    Completed,
    Failed,
}

impl fmt::Display for QuestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuestStatus::Open => write!(f, "Open"),
            QuestStatus::InJourney => write!(f, "In Journey"),
            QuestStatus::Completed => write!(f, "Completed"),
            QuestStatus::Failed => write!(f, "Failed"),
        }
    }
}
