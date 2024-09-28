use crate::dao;
use crate::models::mh_monster;
use crate::Pool;
use actix_web::{get, web, Error, HttpResponse};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::SqliteConnection;

pub fn handle_error<E: std::fmt::Debug>(e: E, message: &str) -> actix_web::Error {
    log::error!("{}, error: {:?}", message, e);
    actix_web::error::ErrorInternalServerError("Internal server error")
}

#[get("/info/{monster_id}")]
pub async fn get_monster_info_by_id_handler(
    pool: web::Data<Pool>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, Error> {
    log::info!("get_monster_by_id_handler: /info/{}", path.0);
    let db_connection = &mut pool
        .get()
        .map_err(|e| handle_error(e, "get_monster_by_id_handler, failed to get db connection"))?;

    let res = get_monster_info_by_monster_id(db_connection, path.0)
        .await
        .map_err(|e| handle_error(e, "get_monster_by_id_handler, get_monster_by_id failed"))?;

    Ok(HttpResponse::Ok().json(res))
}

async fn get_monster_info_by_monster_id(
    db_connection: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    monster_id: i32,
) -> Result<mh_monster::MHMonster, Error> {
    let monster_info = dao::mh_monster::get_by_monster_id(db_connection, monster_id)
        .await
        .map_err(|e| {
            handle_error(
                e,
                "get_monster_info_by_id, dao::mh_monster::get_by_monster_id failed",
            )
        })?;

    Ok(monster_info)
}
