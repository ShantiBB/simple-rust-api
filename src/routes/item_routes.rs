use actix_web::{web, HttpResponse, Result as ActixResult};
use crate::repository::postgres::PostgresRepository;
use crate::models::item::ItemPayload;

pub async fn create_item(
    pg_repo: web::Data<PostgresRepository>,
    payload: web::Json<ItemPayload>
) -> ActixResult<HttpResponse> {
    let input = ItemPayload { 
        name: payload.name.clone(), 
        description: payload.description.clone() 
    };
    
    match pg_repo.create_item(input).await {
        Ok(item) => Ok(HttpResponse::Created().json(item)),
        Err(e) => {
            log::error!("Error creating item: {:?}", e);
            Ok(HttpResponse::InternalServerError().json("Internal server error"))
        }
    }
}

pub async fn get_all_items(
    pg_repo: web::Data<PostgresRepository>
) -> ActixResult<HttpResponse> {
    match pg_repo.get_all_items().await {
        Ok(items) => Ok(HttpResponse::Ok().json(items)),
        Err(e) => {
            log::error!("Error getting all items: {:?}", e);
            Ok(HttpResponse::InternalServerError().json("Internal server error"))
        }
    }
}

pub async fn get_item_by_id(
    pg_repo: web::Data<PostgresRepository>,
    path: web::Path<i32>
) -> ActixResult<HttpResponse> {
    let id = path.into_inner();
    
    match pg_repo.get_item_by_id(id).await {
        Ok(Some(item)) => Ok(HttpResponse::Ok().json(item)),
        Ok(None) => Ok(HttpResponse::NotFound().json("Item not found")),
        Err(e) => {
            log::error!("Error getting item by id: {:?}", e);
            Ok(HttpResponse::InternalServerError().json("Internal server error"))
        }
    }
}

pub async fn update_item_by_id(
    pg_repo: web::Data<PostgresRepository>,
    path: web::Path<i32>,
    payload: web::Json<ItemPayload>
) -> ActixResult<HttpResponse> {
    let id = path.into_inner();
    let input = ItemPayload { 
        name: payload.name.clone(), 
        description: payload.description.clone() 
    };
    
    match pg_repo.update_item_by_id(id, &input).await {
        Ok(Some(item)) => Ok(HttpResponse::Ok().json(item)),
        Ok(None) => Ok(HttpResponse::NotFound().json("Item not found")),
        Err(e) => {
            log::error!("Error updating item by id: {:?}", e);
            Ok(HttpResponse::InternalServerError().json("Internal server error"))
        }
    }
}

pub async fn delete_item_by_id(
    pg_repo: web::Data<PostgresRepository>,
    path: web::Path<i32>
) -> ActixResult<HttpResponse> {
    let id = path.into_inner();
    
    match pg_repo.delete_item_by_id(id).await {
        Ok(true) => Ok(HttpResponse::NoContent().finish()),
        Ok(false) => Ok(HttpResponse::NotFound().json("Item not found")),
        Err(e) => {
            log::error!("Error deleting item by id: {:?}", e);
            Ok(HttpResponse::InternalServerError().json("Internal server error"))
        }
    }
}
