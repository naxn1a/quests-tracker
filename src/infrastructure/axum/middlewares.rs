use crate::{
    config::loader::{get_adventurer_secret, get_guild_commander_secret},
    infrastructure::auth,
};
use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};

pub async fn guild_commanders_authorization(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if let Some(cookie_header) = req.headers().get(header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            let access_token = get_cookie_value(cookie_str, "act");

            if let Some(token) = access_token {
                if let Ok(secret_env) = get_guild_commander_secret() {
                    if let Ok(claims) = auth::verify_token(secret_env.secret.to_string(), token) {
                        if let Ok(guild_commander_id) = claims.sub.parse::<i32>() {
                            req.extensions_mut().insert(guild_commander_id);
                            return Ok(next.run(req).await);
                        }
                    };
                }
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

pub async fn adventurers_authorization(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if let Some(cookie_header) = req.headers().get(header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            let access_token = get_cookie_value(cookie_str, "act");

            if let Some(token) = access_token {
                if let Ok(secret_env) = get_adventurer_secret() {
                    if let Ok(claims) = auth::verify_token(secret_env.secret.to_string(), token) {
                        if let Ok(adventurer_id) = claims.sub.parse::<i32>() {
                            req.extensions_mut().insert(adventurer_id);
                            return Ok(next.run(req).await);
                        }
                    };
                }
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

fn get_cookie_value(cookie_header: &str, key: &str) -> Option<String> {
    cookie_header.split("; ").find_map(|cookie| {
        let mut parts = cookie.splitn(2, '=');
        let name = parts.next()?.trim();
        let value = parts.next()?.trim();
        if name == key {
            Some(value.to_string())
        } else {
            None
        }
    })
}
