use crate::{
    config::loader::{get_adventurer_secret, get_guild_commander_secret},
    domain::repositories::{
        adventurers::AdventurersRepository, guild_commanders::GuildCommandersRepository,
    },
    infrastructure::{
        argon2,
        auth::{
            self,
            authentication_model::LoginModel,
            jwt_model::{Claims, Passport, Roles},
        },
    },
};
use anyhow::Result;
use chrono::{Duration, Utc};
use std::sync::Arc;

pub struct AuthenticationUseCase<T1, T2>
where
    T1: AdventurersRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    adventurers_repository: Arc<T1>,
    guild_commander_repository: Arc<T2>,
}

impl<T1, T2> AuthenticationUseCase<T1, T2>
where
    T1: AdventurersRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    pub fn new(adventurers_repository: Arc<T1>, guild_commander_repository: Arc<T2>) -> Self {
        Self {
            adventurers_repository,
            guild_commander_repository,
        }
    }

    pub async fn adventures_login(&self, login_model: LoginModel) -> Result<Passport> {
        let secret_env = get_adventurer_secret()?;

        let adventurer = self
            .adventurers_repository
            .find_by_username(login_model.username.clone())
            .await?;

        let original_password = adventurer.password;
        let login_password = login_model.password;

        if !argon2::verify(login_password, original_password)? {
            return Err(anyhow::anyhow!("Invalid username or password"));
        };

        let access_token_claims = Claims {
            sub: adventurer.id.to_string(),
            role: Roles::Adventurer,
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let refresh_token_claims = Claims {
            sub: adventurer.id.to_string(),
            role: Roles::Adventurer,
            exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token = auth::generate_token(secret_env.secret, &access_token_claims)?;
        let refresh_token = auth::generate_token(secret_env.refresh_token, &refresh_token_claims)?;

        Ok(Passport {
            access_token,
            refresh_token,
        })
    }

    pub async fn adventurers_refresh_token(&self, refresh_token: String) -> Result<Passport> {
        let secret_env = get_adventurer_secret()?;

        let claims = auth::verify_token(secret_env.refresh_token.clone(), refresh_token)?;

        let access_token_claims = Claims {
            sub: claims.sub.clone(),
            role: Roles::Adventurer,
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let refresh_token_claims = Claims {
            sub: claims.sub,
            role: Roles::Adventurer,
            exp: claims.exp,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token = auth::generate_token(secret_env.secret, &access_token_claims)?;
        let refresh_token = auth::generate_token(secret_env.refresh_token, &refresh_token_claims)?;

        Ok(Passport {
            access_token,
            refresh_token,
        })
    }

    pub async fn guild_commanders_login(&self, login_model: LoginModel) -> Result<Passport> {
        let secret_env = get_guild_commander_secret()?;

        let guild_commander = self
            .guild_commander_repository
            .find_by_username(login_model.username.clone())
            .await?;

        let original_password = guild_commander.password;
        let login_password = login_model.password;

        if !argon2::verify(login_password, original_password)? {
            return Err(anyhow::anyhow!("Invalid username or password"));
        };

        let access_token_claims = Claims {
            sub: guild_commander.id.to_string(),
            role: Roles::GuildCommander,
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let refresh_token_claims = Claims {
            sub: guild_commander.id.to_string(),
            role: Roles::GuildCommander,
            exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token = auth::generate_token(secret_env.secret, &access_token_claims)?;
        let refresh_token = auth::generate_token(secret_env.refresh_token, &refresh_token_claims)?;

        Ok(Passport {
            access_token,
            refresh_token,
        })
    }

    pub async fn guild_commanders_refresh_token(&self, refresh_token: String) -> Result<Passport> {
        let secret_env = get_guild_commander_secret()?;

        let claims = auth::verify_token(secret_env.refresh_token.clone(), refresh_token)?;

        let access_token_claims = Claims {
            sub: claims.sub.clone(),
            role: Roles::GuildCommander,
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let refresh_token_claims = Claims {
            sub: claims.sub,
            role: Roles::GuildCommander,
            exp: claims.exp,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token = auth::generate_token(secret_env.secret, &access_token_claims)?;
        let refresh_token = auth::generate_token(secret_env.refresh_token, &refresh_token_claims)?;

        Ok(Passport {
            access_token,
            refresh_token,
        })
    }
}
