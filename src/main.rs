use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use actix_web::http::header;
use service::redis_service::RedisService;
use dotenv::dotenv;

mod auth;
mod config;
mod db;
mod errors;
mod models;
mod service;
mod ai;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    
    let config = config::Config::from_env().expect("配置错误");
    let db = db::init_db(&config).await.expect("数据库连接失败");
    let redis_service = RedisService::new(&config.redis_url)
    .expect("Redis 服务初始化失败");
    let ai_service = ai::service::AIServiceImpl::new(config.clone());
    
    log::info!("启动服务器 http://{}:{}", config.server_host, config.server_port);
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION])
            .max_age(3600);
    
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(redis_service.clone()))
            .app_data(web::Data::new(ai_service.clone()))
            .configure(auth::routes::auth_config)
            .configure(ai::routes::ai_config)
    })
    .bind((config.server_host.clone(), config.server_port))?
    .run()
    .await
}