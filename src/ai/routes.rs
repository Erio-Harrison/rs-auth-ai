use actix_web::web;
use super::handlers;

// ai/routes.rs
pub fn ai_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ai")
            .route("/text", web::post().to(handlers::analyze_text)) 
            .route("/image", web::post().to(handlers::analyze_image))
    );
}