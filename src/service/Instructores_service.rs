use crate::models::Instructores::{
    ActualizarInstructor, InstructorConClases, Instructores, NuevoInstructor,
};
use crate::repository::Instructores_repository::InstructoresRepository;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::PgPool;

// Retorna la lista de todos los instructores
pub async fn obtener_instructores(State(pool): State<PgPool>) -> Json<Vec<Instructores>> {
    let repo = InstructoresRepository::new(pool);
    match repo.obtener_instructores().await {
        Ok(instructores) => Json(instructores),
        Err(_) => Json(vec![]),
    }
}

// Retorna un instructor por su ID
pub async fn obtener_instructor_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Instructores>, StatusCode> {
    let repo = InstructoresRepository::new(pool);
    match repo.obtener_instructor_por_id(id).await {
        Ok(instructor) => Ok(Json(instructor)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

// Crea un instructor nuevo
pub async fn agregar_instructor(
    State(pool): State<PgPool>,
    Json(nuevo): Json<NuevoInstructor>,
) -> Result<Json<Instructores>, StatusCode> {
    let repo = InstructoresRepository::new(pool);
    match repo.crear_instructor(nuevo).await {
        Ok(instructor) => Ok(Json(instructor)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Actualiza los datos de un instructor existente
pub async fn actualizar_instructor(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(datos): Json<ActualizarInstructor>,
) -> Result<Json<Instructores>, StatusCode> {
    let repo = InstructoresRepository::new(pool);
    match repo.actualizar_instructor(id, datos).await {
        Ok(instructor) => Ok(Json(instructor)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Elimina un instructor (sus clases quedan con instructor NULL automaticamente)
pub async fn eliminar_instructor(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<bool>, StatusCode> {
    let repo = InstructoresRepository::new(pool);
    match repo.eliminar_instructor(id).await {
        Ok(_) => Ok(Json(true)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// RELACION: Retorna el instructor junto con sus clases asignadas
pub async fn obtener_instructor_con_clases(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<InstructorConClases>, StatusCode> {
    let repo = InstructoresRepository::new(pool);
    match repo.obtener_instructor_con_clases(id).await {
        Ok(resultado) => Ok(Json(resultado)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}
