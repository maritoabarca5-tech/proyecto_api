use crate::service::Instructores_service::{
    actualizar_instructor,
    agregar_instructor,
    eliminar_instructor,
    obtener_instructor_con_clases, // ruta extra para la relacion con Clases
    obtener_instructor_por_id,
    obtener_instructores,
};
use axum::{
    Router,
    routing::{delete, get, post, put},
};
use sqlx::PgPool;

pub fn instructores_routes(pool: PgPool) -> Router {
    Router::new()
        // CRUD basico
        .route("/instructores", get(obtener_instructores))
        .route(
            "/instructores/{id_instructor}",
            get(obtener_instructor_por_id),
        )
        .route("/instructores", post(agregar_instructor))
        .route("/instructores/{id_instructor}", put(actualizar_instructor))
        .route("/instructores/{id_instructor}", delete(eliminar_instructor))
        // Ruta extra: instructor + sus clases (relacion)
        .route(
            "/instructores/{id_instructor}/clases",
            get(obtener_instructor_con_clases),
        )
        .with_state(pool)
}
