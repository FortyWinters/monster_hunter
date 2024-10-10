use crate::schema::*;
use diesel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct MonsterPart {
    pub id: i32,
    pub monster_id: i32,
    pub part_id: i32,
    pub part_name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = monster_part)]
pub struct PostMonsterPart<'a> {
    pub monster_id: &'a i32,
    pub part_id: &'a i32,
    pub part_name: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonsterPartJson {
    pub monster_id: i32,
    pub part_id: i32,
    pub part_name: String,
}
