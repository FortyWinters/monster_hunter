use crate::models::monster::*;
use crate::schema::monster_rise::dsl::*;
use crate::schema::monster_wild::dsl::*;
use crate::schema::monster_world::dsl::*;
use crate::schema::{monster_rise, monster_wild, monster_world};
use anyhow::Result;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

pub async fn get_by_monster_name(
    db_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    query_monster_name: &String,
    query_game: i32,
) -> Result<Monster, diesel::result::Error> {
    let result: Monster = match query_game {
        0 => monster_world
            .filter(monster_world::monster_name.eq(query_monster_name))
            .first::<Monster>(db_connection)?,
        1 => monster_rise
            .filter(monster_rise::monster_name.eq(query_monster_name))
            .first::<Monster>(db_connection)?,
        2 => monster_wild
            .filter(monster_wild::monster_name.eq(query_monster_name))
            .first::<Monster>(db_connection)?,
        _ => {
            return Err(diesel::result::Error::QueryBuilderError(
                "get_by_monster_name, Invalid game query".into(),
            ));
        }
    };
    Ok(result)
}
