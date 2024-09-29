use actix_web::web;

pub fn monster_routes(cfg: &mut web::ServiceConfig) {
    use crate::api::monster::*;
    cfg.service(web::scope("/api/monster").service(get_info_handler));
}
