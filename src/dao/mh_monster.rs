use crate::models::mh_monster::*;
use crate::schema::mh_monster::dsl::*;
use anyhow::Result;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

#[allow(dead_code)]
pub async fn get_by_monster_id(
    db_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    query_monster_id: i32,
) -> Result<MHMonster, diesel::result::Error> {
    let result: MHMonster = mh_monster
        .filter(monster_id.eq(query_monster_id))
        .first::<MHMonster>(db_connection)?;
    Ok(result)
}

pub async fn get_by_monster_name(
    db_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    query_monster_name: &String,
) -> Result<MHMonster, diesel::result::Error> {
    let result: MHMonster = mh_monster
        .filter(monster_name.eq(query_monster_name))
        .first::<MHMonster>(db_connection)?;
    Ok(result)
}
