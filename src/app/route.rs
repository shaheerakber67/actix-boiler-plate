use crate::app::api::{users, field_sector};
use actix_web::web;

pub fn setup_routes(cfg: &mut web::ServiceConfig) -> &mut web::ServiceConfig {
    cfg.service((users::list, users::create, users::delete));
    cfg.service((field_sector::list_all, field_sector::list_all_meetings, field_sector::list_all_translations, field_sector::create, field_sector::get_field_sector_translations_by_id, field_sector::update, field_sector::delete))
}
