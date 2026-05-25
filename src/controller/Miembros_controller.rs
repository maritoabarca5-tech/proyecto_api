use crate::service::Miembros_service::{
    actualizar_miembro, agregar_miembro, eliminar_miembro, obtener_miembro_por_id,
    obtener_miembros,
};
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;

pub fn miembros_routes(pool: PgPool) -> Router {
    Router::new()
        // CRUD básico
        .route("/miembros", get(obtener_miembros))
        .route("/miembros/{id_miembro}", get(obtener_miembro_por_id))
        .route("/miembros", post(agregar_miembro))
        .route("/miembros/{id_miembro}", put(actualizar_miembro))
        .route("/miembros/{id_miembro}", delete(eliminar_miembro))
        .with_state(pool)
}
