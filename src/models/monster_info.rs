use crate::schema::*;
use diesel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct MonsterInfo {
    pub id: i32,
    pub monster_id: i32,
    pub monster_name: String,
    pub monster_type: i32,
    pub monster_alias: Option<String>,
    pub monster_description: Option<String>,
    pub game_type: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = monster_info)]
pub struct PostMonsterInfo {
    pub monster_id: i32,
    pub monster_name: String,
    pub monster_type: i32,
    pub monster_alias: Option<String>,
    pub monster_description: Option<String>,
    pub game_type: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonsterInfoJson {
    pub monster_id: i32,
    pub monster_name: String,
    pub monster_type: i32,
    pub monster_alias: Option<String>,
    pub monster_description: Option<String>,
    pub game_type: i32,
}
