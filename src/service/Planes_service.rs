use axum::{extract::{Path, State}, http::StatusCode, Json};
use sqlx::PgPool;
use crate::models::Planes::{Planes, NuevoPlanes, ActualizarPlanes};
use crate::repository::Planes_repository::PlanesRepository;

pub async fn obtener_planes(State(pool): State<PgPool>) -> Json<Vec<Planes>> {
    let repo = PlanesRepository::new(pool);
    match repo.obtener_planes().await {
        Ok(planes) => Json(planes),
        Err(_) => Json(vec![]),
    }
}

pub async fn obtener_planes_por_id(
    State(pool): State<PgPool>,
    Path(id_plan): Path<i32>,
) -> Result<Json<Planes>, StatusCode> {
    let repo = PlanesRepository::new(pool);
    match repo.obtener_planes_por_id(id_plan).await {
        Ok(plan) => Ok(Json(plan)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn agregar_planes(
    State(pool): State<PgPool>,
    Json(nuevo_planes): Json<NuevoPlanes>,
) -> Result<Json<Planes>, StatusCode> {
    let repo = PlanesRepository::new(pool);
    match repo.crear_planes(nuevo_planes).await {
        Ok(plan) => Ok(Json(plan)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn actualizar_planes(
    State(pool): State<PgPool>,
    Path(id_plan): Path<i32>,
    Json(planes_actualizados): Json<ActualizarPlanes>,
) -> Result<Json<Planes>, StatusCode> {
    let repo = PlanesRepository::new(pool);
    match repo.actualizar_planes(id_plan, planes_actualizados).await {
        Ok(plan) => Ok(Json(plan)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn eliminar_planes(
    State(pool): State<PgPool>,
    Path(id_plan): Path<i32>,
) -> Result<Json<bool>, StatusCode> {
    let repo = PlanesRepository::new(pool);
    match repo.eliminar_planes(id_plan).await {
        Ok(_) => Ok(Json(true)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
