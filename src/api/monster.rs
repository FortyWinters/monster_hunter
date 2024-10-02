use crate::dao;
use crate::models::mh_monster;
use crate::Pool;
use actix_web::{get, web, Error, HttpResponse};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use serde::{Deserialize, Serialize};

pub fn handle_error<E: std::fmt::Debug>(e: E, message: &str) -> actix_web::Error {
    log::error!("{}, error: {:?}", message, e);
    actix_web::error::ErrorInternalServerError("Internal server error")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonsterRequestJson {
    pub monster_name: String,
}

#[get("/info")]
pub async fn get_info_handler(
    pool: web::Data<Pool>,
    item: web::Json<MonsterRequestJson>,
) -> Result<HttpResponse, Error> {
    log::info!("get_monster_by_id_handler: /info/{:?}", item);
    let db_connection = &mut pool
        .get()
        .map_err(|e| handle_error(e, "get_monster_by_id_handler, failed to get db connection"))?;

    let res = get_info(db_connection, item.into_inner())
        .await
        .map_err(|e| handle_error(e, "get_monster_by_id_handler, get_monster_by_id failed"))?;

    Ok(HttpResponse::Ok().json(res))
}

async fn get_info(
    db_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    item: MonsterRequestJson,
) -> Result<mh_monster::MHMonster, Error> {
    log::info!(
        "get_monster_by_id, get by monster_name: {}",
        item.monster_name
    );
    let monster_info = dao::mh_monster::get_by_monster_name(db_connection, &item.monster_name)
        .await
        .map_err(|e| handle_error(e, "get_info, dao::mh_monster::get_by_monster_name failed"))?;
    Ok(monster_info)
}
