use axum::{extract::{Path, State}, http::StatusCode, Json};
use sqlx::PgPool;
use crate::models::Asistencia_Clases::{AsistenciaClases, NuevaAsistenciaClases, ActualizarAsistenciaClases};
use crate::repository::Asistencia_Clases_repository::AsistenciaClasesRepository;

pub async fn obtener_asistencias(State(pool): State<PgPool>) -> Json<Vec<AsistenciaClases>> {
    let repo = AsistenciaClasesRepository::new(pool);
    match repo.obtener_asistencias().await {
        Ok(asistencias) => Json(asistencias),
        Err(_) => Json(vec![]),
    }
}

pub async fn obtener_asistencia_por_id(
    State(pool): State<PgPool>,
    Path(id_asistencia): Path<i32>,
) -> Result<Json<AsistenciaClases>, StatusCode> {
    let repo = AsistenciaClasesRepository::new(pool);
    match repo.obtener_asistencia_por_id(id_asistencia).await {
        Ok(asistencia) => Ok(Json(asistencia)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn agregar_asistencia(
    State(pool): State<PgPool>,
    Json(nueva_asistencia): Json<NuevaAsistenciaClases>,
) -> Result<Json<AsistenciaClases>, StatusCode> {
    let repo = AsistenciaClasesRepository::new(pool);
    match repo.crear_asistencia(nueva_asistencia).await {
        Ok(asistencia) => Ok(Json(asistencia)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn actualizar_asistencia(
    State(pool): State<PgPool>,
    Path(id_asistencia): Path<i32>,
    Json(datos_actualizados): Json<ActualizarAsistenciaClases>,
) -> Result<Json<AsistenciaClases>, StatusCode> {
    let repo = AsistenciaClasesRepository::new(pool);
    match repo.actualizar_asistencia(id_asistencia, datos_actualizados).await {
        Ok(asistencia) => Ok(Json(asistencia)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn eliminar_asistencia(
    State(pool): State<PgPool>,
    Path(id_asistencia): Path<i32>,
) -> Result<Json<bool>, StatusCode> {
    let repo = AsistenciaClasesRepository::new(pool);
    match repo.eliminar_asistencia(id_asistencia).await {
        Ok(_) => Ok(Json(true)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}