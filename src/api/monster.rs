use crate::dao;
use crate::models;
use crate::models::monster_info::{self, MonsterInfoJson};
use crate::models::monster_weakness::MonsterWeaknessJson;
use crate::mods::spider::SpMonster;
use crate::mods::spider::Spider;
use crate::Pool;
use actix_web::{web, Error, HttpResponse};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct MonsterInfoGetReqJson {
    pub name: String,   // name or alias
    pub game_type: i32, // world=>0, rise=>1, wild=>2
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonsterInfoUpdateReqJson {
    pub game_type: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonsterWeaknessGetReqJson {
    pub monster_name: String,
    pub game_type: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonsterWeaknessGetRes {
    pub monster_id: i32,
    pub monster_name: String,
    pub monster_type: i32,
    pub game_type: i32,
    pub monster_parts: Vec<MonsterPartRes>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonsterPartRes {
    pub part_name: String,
    pub monster_weaknesses: Vec<MonsterWeaknessRes>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonsterWeaknessRes {
    pub weakness_type: i32,
    pub weakness_value: i32,
}

pub fn handle_error<E: std::fmt::Debug>(e: E, message: &str) -> actix_web::Error {
    log::error!("{}, error: {:?}", message, e);
    actix_web::error::ErrorInternalServerError("Internal server error")
}

macro_rules! api {
    ($method:ident, $path:expr, $json_type:ty, $func_name:ident) => {
        paste! {
            #[actix_web::$method($path)]
            async fn [<$func_name _handler>](
                pool: web::Data<Pool>,
                item: web::Json<$json_type>,
            ) -> Result<HttpResponse, actix_web::Error> {
                log::info!("{}, {:?}", stringify!([<$func_name _handler>]), item);
                let db_connection = &mut pool
                    .get()
                    .map_err(|e| handle_error(e, "failed to get db connection"))?;

                let res = $func_name(db_connection, item.into_inner())
                    .await
                    .map_err(|e| handle_error(e, stringify!($func_name)))?;

                Ok(HttpResponse::Ok().json(res))
            }
        }
    };
}

api!(get, "/info/get", MonsterInfoGetReqJson, get_info);
api!(post, "/info/update", MonsterInfoUpdateReqJson, update_info);
api!(
    get,
    "/weakness/get",
    MonsterWeaknessGetReqJson,
    get_weakness
);

async fn get_info(
    db_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    item: MonsterInfoGetReqJson,
) -> Result<monster_info::MonsterInfo, Error> {
    let monster_info = dao::monster_info::get_by_name(db_connection, &item.name, item.game_type)
        .await
        .map_err(|e| handle_error(e, "get_info, dao::monster::get_by_monster_name failed"))?;
    Ok(monster_info)
}

async fn update_info(
    db_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    item: MonsterInfoUpdateReqJson,
) -> Result<String, Error> {
    let sp = Spider::new().unwrap();
    let mut monster_info_json_vec: Vec<MonsterInfoJson> = Vec::new();
    let mut monster_weakness_json_vec: Vec<MonsterWeaknessJson> = Vec::new();
    let monster_info_vec: Vec<SpMonster>;

    match item.game_type {
        0 => {
            let monster_url_vec = sp.get_world_monster_url().await.unwrap();
            monster_info_vec = sp.get_world_monster_by_url(monster_url_vec).await.unwrap();
        }
        1 => {
            let lg_monster_url_vec = sp.get_rise_monster_url("lg").await.unwrap();
            let sm_monster_url_vec = sp.get_rise_monster_url("sm").await.unwrap();
            let mut monster_url_vec = lg_monster_url_vec.clone();
            monster_url_vec.extend(sm_monster_url_vec);
            monster_info_vec = sp.get_rise_monster_by_url(monster_url_vec).await.unwrap();
        }
        _ => {
            return Err(handle_error(
                "Invalid game type",
                "update_info: Unsupported game type",
            ));
        }
    }

    for m in monster_info_vec {
        monster_info_json_vec.push(models::monster_info::MonsterInfoJson {
            monster_id: m.monster_id,
            monster_name: m.monster_name,
            monster_type: 0,
            monster_description: Some(m.monster_description),
            monster_icon_url: Some(m.monster_icon_url),
            game_type: item.game_type,
        });

        for p in m.monster_parts {
            for w in p.weaknesses {
                monster_weakness_json_vec.push(models::monster_weakness::MonsterWeaknessJson {
                    monster_id: m.monster_id,
                    part_name: p.part_name.clone(),
                    weakness_type: w.weakness_type,
                    weakness_value: w.weakness_value,
                    game_type: item.game_type,
                });
            }
        }
    }

    let monster_number = dao::monster_info::add_vec(db_connection, monster_info_json_vec)
        .await
        .map_err(|e| handle_error(e, "udpate_info, dao::monster_info::add_vec failed"))?;

    let weakness_number = dao::monster_weakness::add_vec(db_connection, monster_weakness_json_vec)
        .await
        .map_err(|e| handle_error(e, "udpate_info, dao::monster_weakness::add_vec failed"))?;

    Ok(format!(
        "monster number: {} \nweakness number: {} ",
        monster_number, weakness_number
    ))
}

async fn get_weakness(
    db_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    item: MonsterWeaknessGetReqJson,
) -> Result<MonsterWeaknessGetRes, Error> {
    let monster_info =
        dao::monster_info::get_by_name(db_connection, &item.monster_name, item.game_type)
            .await
            .map_err(|e| handle_error(e, "get_info, dao::monster::get_by_name failed"))?;

    let monster_weakness = dao::monster_weakness::get_by_monster_id(
        db_connection,
        monster_info.monster_id,
        item.game_type,
    )
    .await
    .map_err(|e| {
        handle_error(
            e,
            "get_weakness, dao::weaknessr::get_by_monster_name failed",
        )
    })?;

    let mut part_map: HashMap<String, Vec<MonsterWeaknessRes>> = HashMap::new();

    for w in monster_weakness {
        let weakness_res = MonsterWeaknessRes {
            weakness_type: w.weakness_type,
            weakness_value: w.weakness_value,
        };

        part_map
            .entry(w.part_name.clone())
            .or_insert_with(Vec::new)
            .push(weakness_res);
    }

    let monster_parts: Vec<MonsterPartRes> = part_map
        .into_iter()
        .map(|(part_name, weaknesses)| MonsterPartRes {
            part_name,
            monster_weaknesses: weaknesses,
        })
        .collect();

    Ok(MonsterWeaknessGetRes {
        monster_id: monster_info.monster_id,
        monster_name: monster_info.monster_name,
        monster_type: monster_info.monster_type,
        game_type: monster_info.game_type,
        monster_parts,
    })
}
