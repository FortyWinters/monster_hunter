use crate::models::monster_info::*;
use crate::schema::monster_info::dsl::*;
use anyhow::Result;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

pub async fn get_by_name(
    db_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    query_name: &String,
    query_game_type: i32,
) -> Result<MonsterInfo, diesel::result::Error> {
    let result: MonsterInfo = monster_info
        .filter(game_type.eq(query_game_type))
        .filter(monster_name.eq(query_name))
        .first::<MonsterInfo>(db_connection)?;
    Ok(result)
}

pub async fn add_vec(
    db_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    item_vec: Vec<MonsterInfoJson>,
) -> Result<i32, diesel::result::Error> {
    let mut sucess_num: i32 = 0;
    for item in &item_vec {
        if let Err(_) = monster_info
            .filter(monster_id.eq(&item.monster_id))
            .first::<MonsterInfo>(db_connection)
        {
            let new_monster_info = PostMonsterInfo {
                monster_id: item.monster_id,
                monster_name: item.monster_name.clone(),
                monster_type: item.monster_type,
                monster_description: item.monster_description.clone(),
                monster_icon_url: item.monster_icon_url.clone(),
                game_type: item.game_type,
            };
            insert_into(monster_info)
                .values(&new_monster_info)
                .execute(db_connection)
                .expect("save failed");
            sucess_num += 1;
        }
    }
    Ok(sucess_num)
}
