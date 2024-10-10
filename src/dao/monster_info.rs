use crate::models::monster_info::*;
use crate::schema::monster_info::dsl::*;
use anyhow::Result;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

pub async fn get_by_name(
    db_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    query_name: &String,
    query_game_type: i32,
) -> Result<MonsterInfo, diesel::result::Error> {
    let result: Result<MonsterInfo, diesel::result::Error> = monster_info
        .filter(game_type.eq(query_game_type))
        .filter(monster_name.eq(query_name))
        .first::<MonsterInfo>(db_connection);

    match result {
        Ok(monster) => Ok(monster),
        Err(err) => {
            if err == diesel::result::Error::NotFound {
                let alias_result: Result<MonsterInfo, diesel::result::Error> = monster_info
                    .filter(game_type.eq(query_game_type))
                    .filter(monster_alias.eq(query_name))
                    .first::<MonsterInfo>(db_connection);

                alias_result
            } else {
                Err(err)
            }
        }
    }
}
