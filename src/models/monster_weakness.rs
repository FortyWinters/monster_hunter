use crate::schema::*;
use diesel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct MonsterWeakness {
    pub id: i32,
    pub part_id: i32,
    pub weakness_name: String,
    pub weakness_level: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = monster_weakness)]
pub struct PostMonsterWeakness<'a> {
    pub part_id: &'a i32,
    pub weakness_name: &'a str,
    pub weakness_level: &'a i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonsterWeaknessJson {
    pub part_id: i32,
    pub weakness_name: String,
    pub weakness_level: i32,
}
