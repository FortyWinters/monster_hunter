use crate::schema::*;
use diesel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct MonsterWeakness {
    pub id: i32,
    pub monster_id: i32,
    pub part_name: String,
    pub weakness_type: i32,
    pub weakness_value: i32,
    pub game_type: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = monster_weakness)]
pub struct PostMonsterWeakness {
    pub monster_id: i32,
    pub part_name: String,
    pub weakness_type: i32,
    pub weakness_value: i32,
    pub game_type: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonsterWeaknessJson {
    pub monster_id: i32,
    pub part_name: String,
    pub weakness_type: i32,
    pub weakness_value: i32,
    pub game_type: i32,
}
