use anyhow::Result;
use std::fmt;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Stage {
    Local,
    #[default]
    Development,
    Production,
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stage = match self {
            Stage::Local => "Local",
            Stage::Development => "Development",
            Stage::Production => "Production",
        };

        write!(f, "{}", stage)
    }
}

impl Stage {
    pub fn try_form(stage: &str) -> Result<Self> {
        match stage.to_lowercase().as_str() {
            "Local" => Ok(Stage::Local),
            "Development" => Ok(Stage::Development),
            "Production" => Ok(Stage::Production),
            _ => Err(anyhow::anyhow!("Invalid stage: {}", stage)),
        }
    }
}
