use crate::schema::*;
use serde::{Deserialize, Serialize};
use diesel;

#[derive(Debug, Serialize, Deserialize, Queryable, Eq, PartialEq)]
pub struct MHMonster {
    pub id: Option<i32>,
    pub monster_id: i32,
    pub monster_name: String,
    pub monster_type: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = mh_monster)]
pub struct PostMHMonster<'a> {
    pub monster_id: &'a i32,
    pub monster_name: &'a str,
    pub monster_type: &'a i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MHMonsterJson {
    pub monster_id: i32,
    pub monster_name: String,
    pub monster_type: i32,
}
