use crate::models::monster_weakness::*;
use crate::schema::monster_weakness::dsl::*;
use anyhow::Result;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

pub async fn get_by_monster_id(
    db_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    query_monster_id: i32,
    query_game_type: i32,
) -> Result<Vec<MonsterWeakness>, diesel::result::Error> {
    let result: Vec<MonsterWeakness> = monster_weakness
        .filter(game_type.eq(query_game_type))
        .filter(monster_id.eq(query_monster_id))
        .load::<MonsterWeakness>(db_connection)?;
    Ok(result)
}

pub async fn add_vec(
    db_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    item_vec: Vec<MonsterWeaknessJson>,
) -> Result<i32, diesel::result::Error> {
    let mut sucess_num: i32 = 0;
    for item in &item_vec {
        if let Err(_) = monster_weakness
            .filter(monster_id.eq(&item.monster_id))
            .filter(game_type.eq(&item.game_type))
            .filter(part_name.eq(&item.part_name))
            .filter(weakness_type.eq(&item.weakness_type))
            .first::<MonsterWeakness>(db_connection)
        {
            let new_monster_weakness = PostMonsterWeakness {
                monster_id: item.monster_id,
                part_name: item.part_name.clone(),
                weakness_type: item.weakness_type,
                weakness_value: item.weakness_value,
                game_type: item.game_type,
            };
            insert_into(monster_weakness)
                .values(&new_monster_weakness)
                .execute(db_connection)
                .expect("save failed");
            sucess_num += 1;
        }
    }
    println!("{:?}", item_vec.get(0));
    Ok(sucess_num)
}
