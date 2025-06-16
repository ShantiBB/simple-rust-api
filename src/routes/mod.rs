pub mod item_routes;

use actix_web::web;
use crate::repository::postgres::PostgresRepository;
use self::item_routes::*;

pub fn configure_routes(cfg: &mut web::ServiceConfig, pg_repo: PostgresRepository) {
    cfg.app_data(web::Data::new(pg_repo))
        .service(
            web::scope("/api/v1")
                .service(
                    web::scope("/items")
                        .route("", web::get().to(get_all_items))
                        .route("", web::post().to(create_item))
                        .route("/{id}", web::get().to(get_item_by_id))
                        .route("/{id}", web::put().to(update_item_by_id))
                        .route("/{id}", web::delete().to(delete_item_by_id))
                )
        );
}
