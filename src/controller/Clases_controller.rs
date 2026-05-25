use crate::service::Clases_service::{
    actualizar_clase, agregar_clase, eliminar_clase, obtener_clase_por_id, obtener_clases,
};
use axum::{
    Router,
    routing::{delete, get, post, put},
};
use sqlx::PgPool;

pub fn clases_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/clases", get(obtener_clases))
        .route("/clases/{id_clase}", get(obtener_clase_por_id))
        .route("/clases", post(agregar_clase))
        .route("/clases/{id_clase}", put(actualizar_clase))
        .route("/clases/{id_clase}", delete(eliminar_clase))
        .with_state(pool)
}
