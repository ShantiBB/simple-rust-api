use axum::{
    Json,
    http::StatusCode,
    response::{ IntoResponse, Json as RespJson },
    extract:: { State, Path },
};

use crate::repository::postgres::PostgresRepository;
use crate::models::item::{Item, ItemPayload};

pub async fn create_item(
    State(state): State<PostgresRepository>,
    Json(payload): Json<ItemPayload>
) -> impl IntoResponse {
    let input = ItemPayload { 
        name: payload.name, 
        description: payload.description 
    };
    match state.create_item(input).await {
        Ok(item) => (StatusCode::CREATED, RespJson(item)).into_response(),
        Err(e) => {
            eprintln!("Error create item: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_all_items(State(state): State<PostgresRepository>) -> impl IntoResponse {
    match state.get_all_items().await {
        Ok(items) => (StatusCode::OK, RespJson(items)).into_response(),
        Err(e) => {
            eprintln!("Error get all items: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_item_by_id(
    State(state): State<PostgresRepository>,
    Path(id): Path<i32>
) -> impl IntoResponse {
    match state.get_item_by_id(id).await {
        Ok(Some(item)) => (StatusCode::OK, RespJson(item)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("Error get item by id: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn update_item_by_id(
    State(state): State<PostgresRepository>,
    Path(id): Path<i32>,
    Json(payload): Json<ItemPayload>
) -> impl IntoResponse {
    let input: Item = Item { 
        id, 
        name: payload.name, 
        description: payload.description 
    };
    match state.update_item_by_id(&input).await {
        Ok(true) => (StatusCode::OK, RespJson(input)).into_response(),
        Ok(false) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("Error update item by id: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete_item_by_id(
    State(state): State<PostgresRepository>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match state.delete_item_by_id(id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            eprintln!("Error delete item by id: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
