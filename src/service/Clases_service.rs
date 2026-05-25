use crate::models::Clases::{Clases, NuevaClase, ActualizarClase};
use crate::repository::Clases_repository::ClasesRepository;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::PgPool;

// Obtener todas las clases
pub async fn obtener_clases(State(pool): State<PgPool>) -> Json<Vec<Clases>> {
    let repo = ClasesRepository::new(pool);
    match repo.obtener_clases().await {
        Ok(clases) => Json(clases),
        Err(_) => Json(vec![]),
    }
}

// Obtener clase por ID
pub async fn obtener_clase_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Clases>, StatusCode> {
    let repo = ClasesRepository::new(pool);
    match repo.obtener_clase_por_id(id).await {
        Ok(clase) => Ok(Json(clase)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

// Crear nueva clase
pub async fn agregar_clase(
    State(pool): State<PgPool>,
    Json(nueva): Json<NuevaClase>,
) -> Result<Json<Clases>, StatusCode> {
    let repo = ClasesRepository::new(pool);
    match repo.crear_clase(nueva).await {
        Ok(clase) => Ok(Json(clase)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Actualizar clase
pub async fn actualizar_clase(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(datos): Json<ActualizarClase>,
) -> Result<Json<Clases>, StatusCode> {
    let repo = ClasesRepository::new(pool);
    match repo.actualizar_clase(id, datos).await {
        Ok(clase) => Ok(Json(clase)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Eliminar clase
pub async fn eliminar_clase(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<bool>, StatusCode> {
    let repo = ClasesRepository::new(pool);
    match repo.eliminar_clase(id).await {
        Ok(_) => Ok(Json(true)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
