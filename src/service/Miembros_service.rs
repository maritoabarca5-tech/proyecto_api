use crate::models::Miembros::{ActualizarMiembro, Miembros, NuevoMiembro};
use crate::repository::Miembros_repository::MiembrosRepository;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;

// Retorna la lista de todos los miembros
pub async fn obtener_miembros(State(pool): State<PgPool>) -> Json<Vec<Miembros>> {
    let repo = MiembrosRepository::new(pool);

    match repo.obtener_miembros().await {
        Ok(miembros) => Json(miembros),
        Err(_) => Json(vec![]),
    }
}

// Retorna un miembro por su ID
pub async fn obtener_miembro_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Miembros>, StatusCode> {
    let repo = MiembrosRepository::new(pool);

    match repo.obtener_miembro_por_id(id).await {
        Ok(miembro) => Ok(Json(miembro)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

// Crea un miembro nuevo
pub async fn agregar_miembro(
    State(pool): State<PgPool>,
    Json(nuevo): Json<NuevoMiembro>,
) -> Result<Json<Miembros>, StatusCode> {
    let repo = MiembrosRepository::new(pool);

    match repo.crear_miembro(nuevo).await {
        Ok(miembro) => Ok(Json(miembro)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Actualiza los datos de un miembro existente
pub async fn actualizar_miembro(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(datos): Json<ActualizarMiembro>,
) -> Result<Json<Miembros>, StatusCode> {
    let repo = MiembrosRepository::new(pool);

    match repo.actualizar_miembro(id, datos).await {
        Ok(miembro) => Ok(Json(miembro)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Elimina un miembro
pub async fn eliminar_miembro(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<bool>, StatusCode> {
    let repo = MiembrosRepository::new(pool);

    match repo.eliminar_miembro(id).await {
        Ok(_) => Ok(Json(true)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
