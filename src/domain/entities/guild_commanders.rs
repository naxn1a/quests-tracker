use crate::infrastructure::postgresql::schema::guild_commanders;
use diesel::prelude::*;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
#[diesel(table_name = guild_commanders)]
pub struct GuildCommanderEntity {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[diesel(table_name = guild_commanders)]
pub struct RegisterGuildCommanderEntity {
    pub username: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
