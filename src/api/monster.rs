use crate::dao;
use crate::models;
use crate::models::monster_info::{self, MonsterInfoJson};
use crate::mods::spider::Spider;
use crate::Pool;
use actix_web::{web, Error, HttpResponse};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use paste::paste;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MonsterInfoGetReqJson {
    pub name: String,   // name or alias
    pub game_type: i32, // world=>0, rise=>1, wild=>2
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonsterInfoUpdateReqJson {
    pub game_type: i32,
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
    _item: MonsterInfoUpdateReqJson,
) -> Result<i32, Error> {
    let sp = Spider::new().unwrap();
    let monster_url_vec = sp.get_world_monster_url().await.unwrap();
    let monster_info_vec = sp.get_world_monster_by_url(monster_url_vec).await.unwrap();

    let mut monster_info_json_vec: Vec<MonsterInfoJson> = Vec::new();
    for m in monster_info_vec {
        monster_info_json_vec.push(models::monster_info::MonsterInfoJson {
            monster_id: m.monster_id,
            monster_name: m.monster_name,
            monster_type: 0,
            monster_description: Some(m.monster_description),
            monster_icon_url: Some(m.monster_icon_url),
            game_type: 0,
        });
    }

    let monster_number = dao::monster_info::add_vec(db_connection, monster_info_json_vec)
        .await
        .map_err(|e| handle_error(e, "get_info, dao::monster::add_vec failed"))?;

    Ok(monster_number)
}
