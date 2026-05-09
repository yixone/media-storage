use actix_web::web;

mod get_by_id;
mod get_list;
mod upload;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("assets"));
}
