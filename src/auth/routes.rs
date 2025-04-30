use actix_web::web;
use crate::auth::auth_handlers;

pub fn auth_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth") 
            .route("/register", web::post().to(auth_handlers::register))
            .route("/login", web::post().to(auth_handlers::login))
            .route("/oauth", web::post().to(auth_handlers::oauth_login))
    )
    .service(
        web::scope("/user")
            .route("/profile", web::get().to(auth_handlers::get_profile))
            .route("/update_avatar", web::put().to(auth_handlers::update_avatar))
    );
}
