use axum::{
    routing::{delete, get, post, put},
    Router,
};

use sqlx::PgPool;
use crate::service::Planes_service::{
    obtener_planes,
    obtener_planes_por_id,
    agregar_planes,
    actualizar_planes,
    eliminar_planes,
};

pub fn planes_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/planes", get(obtener_planes))
        .route("/planes/{id_plan}", get(obtener_planes_por_id))
        .route("/planes", post(agregar_planes))
        .route("/planes/{id_plan}", put(actualizar_planes))
        .route("/planes/{id_plan}", delete(eliminar_planes))
        .with_state(pool)
}