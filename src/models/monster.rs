use crate::schema::*;
use diesel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Monster {
    pub id: i32,
    pub monster_id: i32,
    pub monster_name: String,
    pub monster_type: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = monster_world)]
#[diesel(table_name = monster_rise)]
#[diesel(table_name = monster_wild)]
pub struct PostMonster<'a> {
    pub monster_id: &'a i32,
    pub monster_name: &'a str,
    pub monster_type: &'a i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonsterJson {
    pub monster_id: i32,
    pub monster_name: String,
    pub monster_type: i32,
}
