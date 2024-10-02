use crate::dao;
use crate::models::monster;
use crate::Pool;
use actix_web::{web, Error, HttpResponse};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use paste::paste;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MonsterReqJson {
    pub monster_name: String,
    pub game: i32, // world=>0, rise=>1, wild=>2
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

api!(get, "/info/get", MonsterReqJson, get_info);

async fn get_info(
    db_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    item: MonsterReqJson,
) -> Result<monster::Monster, Error> {
    let monster_info =
        dao::monster::get_by_monster_name(db_connection, &item.monster_name, item.game)
            .await
            .map_err(|e| handle_error(e, "get_info, dao::monster::get_by_monster_name failed"))?;
    Ok(monster_info)
}
