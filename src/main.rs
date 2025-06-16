mod config;
mod repository;
mod models;
mod handler;
mod routes;
mod middleware;

use std::fs;

use anyhow::{anyhow, Result};
use actix_web::{
    App, HttpServer, 
    middleware::{Logger, DefaultHeaders, NormalizePath, Compress}
};
use env_logger;

use crate::config::config::Config;
use crate::repository::postgres::PostgresRepository;
use crate::routes::configure_routes;
use crate::middleware::Cors;

#[actix_web::main]
async fn main() -> Result<()> {
    // Инициализация логгера для продакшена
    env_logger::init_from_env(
        env_logger::Env::new()
            .default_filter_or("info")
            .default_write_style_or("always")
    );

    log::info!("Starting Simple Rust API Server...");

    // Загрузка конфигурации
    let config_str = fs::read_to_string("src/config/local.yml")
        .map_err(|e| anyhow!("Failed to read config file: {}", e))?;
    let config: Config = serde_yaml::from_str(&config_str)
        .map_err(|e| anyhow!("Failed to parse config: {}", e))?;

    // Подключение к базе данных с проверкой соединения
    let db_url = &config.database.url;
    log::info!("Connecting to database...");
    let pg_repo = PostgresRepository::new(db_url).await
        .map_err(|e| anyhow!("Failed to connect to database: {}", e))?;
    log::info!("Database connection established successfully");

    // Запуск миграций базы данных
    pg_repo.run_migrations().await
        .map_err(|e| anyhow!("Failed to run database migrations: {}", e))?;

    let server_addr = config.web.addr.clone();
    log::info!("Starting HTTP server at {}", server_addr);

    // Запуск HTTP сервера с middleware для продакшена
    HttpServer::new(move || {
        App::new()
            // Логирование запросов
            .wrap(Logger::new(
                r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#
            ))
            // CORS поддержка
            .wrap(Cors)
            // Сжатие ответов
            .wrap(Compress::default())
            // Нормализация путей
            .wrap(NormalizePath::trim())
            // Заголовки безопасности
            .wrap(
                DefaultHeaders::new()
                    .add(("X-Frame-Options", "DENY"))
                    .add(("X-Content-Type-Options", "nosniff"))
                    .add(("X-XSS-Protection", "1; mode=block"))
                    .add(("Referrer-Policy", "strict-origin-when-cross-origin"))
            )
            // Конфигурация маршрутов
            .configure(|cfg| configure_routes(cfg, pg_repo.clone()))
    })
    .bind(&server_addr)
    .map_err(|e| anyhow!("Error binding server to {}: {}", server_addr, e))?
    .workers(8)
    .run()
    .await
    .map_err(|e| anyhow!("Error running server: {}", e))?;

    Ok(())
}
