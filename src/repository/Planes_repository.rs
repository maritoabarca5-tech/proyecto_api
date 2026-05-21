use sqlx::PgPool;
use crate::models::Planes::{Planes, NuevoPlanes, ActualizarPlanes};

pub struct PlanesRepository {
    pool: PgPool,
}

impl PlanesRepository {

    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn obtener_planes(&self) -> sqlx::Result<Vec<Planes>> {
        let planes = sqlx::query_as::<_, Planes>(
            "SELECT id_plan, nombre_plan, precio_mensual FROM Planes"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(planes)
    }

    pub async fn obtener_planes_por_id(&self, id_plan: i32) -> sqlx::Result<Planes> {
        let plan = sqlx::query_as::<_, Planes>(
            "SELECT id_plan, nombre_plan, precio_mensual FROM Planes WHERE id_plan = $1"
        )
        .bind(id_plan)
        .fetch_one(&self.pool)
        .await?;

        Ok(plan)
    }

    pub async fn crear_planes(&self, nuevo_planes: NuevoPlanes) -> sqlx::Result<Planes> {
        let plan = sqlx::query_as::<_, Planes>(
            "INSERT INTO Planes (nombre_plan, precio_mensual) VALUES ($1, $2) RETURNING id_plan, nombre_plan, precio_mensual"
        )
        .bind(&nuevo_planes.nombre_plan)
        .bind(nuevo_planes.precio_mensual)
        .fetch_one(&self.pool)
        .await?;

        Ok(plan)
    }

    pub async fn actualizar_planes(&self, id_plan: i32, planes: ActualizarPlanes) -> sqlx::Result<Planes> {
        let plan = sqlx::query_as::<_, Planes>(
            "UPDATE Planes SET nombre_plan = $1, precio_mensual = $2 WHERE id_plan = $3 RETURNING id_plan, nombre_plan, precio_mensual"
        )
        .bind(&planes.nombre_plan)
        .bind(planes.precio_mensual)
        .bind(id_plan)
        .fetch_one(&self.pool)
        .await?;

        Ok(plan)
    }

    pub async fn eliminar_planes(&self, id_plan: i32) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM Planes WHERE id_plan = $1")
            .bind(id_plan)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}