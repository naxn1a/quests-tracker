use super::{
    model::{AdventurersSecret, Database, DotEnvConfig, GuildCommandersSecret, Server},
    stage::Stage,
};
use anyhow::{Ok, Result};

pub fn load() -> Result<DotEnvConfig> {
    dotenvy::dotenv().ok();

    let server = Server {
        port: std::env::var("SERVER_PORT")
            .expect("SERVER_PORT is invalid")
            .parse()?,
        limit: std::env::var("SERVER_LIMIT")
            .expect("SERVER_LIMIT is invalid")
            .parse()?,
        timeout: std::env::var("SERVER_TIMEOUT")
            .expect("SERVER_TIMEOUT is invalid")
            .parse()?,
    };

    let database = Database {
        url: std::env::var("DATABASE_URL").expect("DATABASE_URL is invalid"),
    };

    Ok(DotEnvConfig { server, database })
}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok();

    let stage_str = std::env::var("STAGE").unwrap_or("".to_string());

    Stage::try_form(&stage_str).unwrap_or_default()
}

pub fn get_adventurer_secret() -> Result<AdventurersSecret> {
    dotenvy::dotenv().ok();

    Ok(AdventurersSecret {
        secret: std::env::var("JWT_ADVENTURER_SECRET").expect("JWT_ADVENTURER_SECRET is invalid"),
        refresh_token: std::env::var("JWT_ADVENTURER_REFRESH_SECRET")
            .expect("JWT_ADVENTURER_REFRESH_TOKEN is invalid"),
    })
}

pub fn get_guild_commander_secret() -> Result<GuildCommandersSecret> {
    dotenvy::dotenv().ok();

    Ok(GuildCommandersSecret {
        secret: std::env::var("JWT_GUILD_COMMANDER_SECRET")
            .expect("JWT_GUILD_COMMANDER_SECRET is invalid"),
        refresh_token: std::env::var("JWT_GUILD_COMMANDER_REFRESH_SECRET")
            .expect("JWT_GUILD_COMMANDER_REFRESH_TOKEN is invalid"),
    })
}
