use crate::models::Asistencia_clases::{ActualizarAsistenciaClases, AsistenciaClases, NuevaAsistenciaClases};
use crate::repository::Asistencia_clases_repository;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;

pub async fn obtener_asistencias(State(pool): State<PgPool>) -> impl IntoResponse {
    match Asistencia_clases_repository::obtener_todas(&pool).await {
        Ok(asistencias) => (StatusCode::OK, Json(asistencias)).into_response(),
        Err(e) => {
            eprintln!("Error al obtener las asistencias: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error al obtener las asistencias",
            )
                .into_response()
        }
    }
}

pub async fn obtener_asistencia_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match Asistencia_clases_repository::obtener_por_id(&pool, id).await {
        Ok(asistencia) => (StatusCode::OK, Json(asistencia)).into_response(),
        Err(e) => {
            eprintln!("Error al obtener asistencia con ID {}: {:?}", id, e);
            (StatusCode::NOT_FOUND, "Asistencia no encontrada").into_response()
        }
    }
}

pub async fn agregar_asistencia(
    State(pool): State<PgPool>,
    Json(nueva_asistencia): Json<NuevaAsistenciaClases>,
) -> impl IntoResponse {
    match Asistencia_clases_repository::agregar(&pool, nueva_asistencia).await {
        Ok(asistencia) => (StatusCode::CREATED, Json(asistencia)).into_response(),
        Err(e) => {
            eprintln!("Error al agregar la asistencia: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error al crear la asistencia",
            )
                .into_response()
        }
    }
}

pub async fn actualizar_asistencia(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(asistencia_actualizada): Json<ActualizarAsistenciaClases>,
) -> impl IntoResponse {
    match Asistencia_clases_repository::actualizar(&pool, id, asistencia_actualizada).await {
        Ok(asistencia) => (StatusCode::OK, Json(asistencia)).into_response(),
        Err(e) => {
            eprintln!("Error al actualizar asistencia con ID {}: {:?}", id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error al actualizar la asistencia",
            )
                .into_response()
        }
    }
}

pub async fn eliminar_asistencia(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match Asistencia_clases_repository::eliminar(&pool, id).await {
        Ok(true) => (StatusCode::OK, Json(true)).into_response(),
        Ok(false) => (StatusCode::NOT_FOUND, "Asistencia no encontrada").into_response(),
        Err(e) => {
            eprintln!("Error al eliminar la asistencia con ID {}: {:?}", id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error al eliminar la asistencia",
            )
                .into_response()
        }
    }
}