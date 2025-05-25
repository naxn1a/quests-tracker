use crate::infrastructure::postgresql::schema::adventurers;
use diesel::prelude::*;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
#[diesel(table_name = adventurers)]
pub struct AdventurerEntity {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[diesel(table_name = adventurers)]
pub struct RegisterAdventurerEntity {
    pub username: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
