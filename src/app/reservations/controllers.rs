use super::{models, services};
use crate::commons::{constants::endpoints, structs::response::TMeta};
use axum::{
    extract::Query,
    response::IntoResponse,
    routing::{get, post, Router},
    Json,
};

pub async fn get_all(Query(params): Query<TMeta>) -> impl IntoResponse {
    services::fetch_reservations(params).await
}

pub async fn create(Json(payload): Json<models::CreateReservation>) -> impl IntoResponse {
    services::create_reservation(Json(payload)).await
}

pub fn reservation_controller() -> Router {
    Router::new()
        .route(endpoints::GET_ALL, get(get_all))
        .route(endpoints::GET_DETAIL, get(get_all))
        .route(endpoints::DELETE, get(get_all))
        .route(endpoints::UPDATE, get(get_all))
        .route(endpoints::CREATE, post(create))
}
