use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;
use crate::service::Asistencia_Clases_service::{
    obtener_asistencias,
    obtener_asistencia_por_id,
    agregar_asistencia,
    actualizar_asistencia,
    eliminar_asistencia,
};

pub fn asistencia_clases_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/asistencia_clases", get(obtener_asistencias))
        .route("/asistencia_clases/{id_asistencia}", get(obtener_asistencia_por_id))
        .route("/asistencia_clases", post(agregar_asistencia))
        .route("/asistencia_clases/{id_asistencia}", put(actualizar_asistencia))
        .route("/asistencia_clases/{id_asistencia}", delete(eliminar_asistencia))
        .with_state(pool)
}