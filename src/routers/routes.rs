use crate::handlers::{general::*, users::*};
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/", web::post().to(post_new_user))
            .route("/", web::get().to(get_all_users)),
    );
}

pub fn category_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/categories")
            .route("/", web::post().to(add_category))
            .route("/", web::get().to(get_all_categories)),
    );
}
