use crate::models::Asistencia_clases::{ActualizarAsistenciaClases, AsistenciaClases, NuevaAsistenciaClases};
use sqlx::PgPool;

pub async fn obtener_todas(pool: &PgPool) -> Result<Vec<AsistenciaClases>, sqlx::Error> {
    sqlx::query_as::<_, AsistenciaClases>("SELECT * FROM Asistencia_Clases")
        .fetch_all(pool)
        .await
}

pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Result<AsistenciaClases, sqlx::Error> {
    sqlx::query_as::<_, AsistenciaClases>("SELECT * FROM Asistencia_Clases WHERE id_asistencia = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn agregar(
    pool: &PgPool,
    nueva_asistencia: NuevaAsistenciaClases,
) -> Result<AsistenciaClases, sqlx::Error> {
    sqlx::query_as::<_, AsistenciaClases>(
        "INSERT INTO Asistencia_Clases (id_miembro, id_clase, fecha_asistencia) 
         VALUES ($1, $2, COALESCE($3, CURRENT_DATE)) 
         RETURNING *",
    )
    .bind(nueva_asistencia.id_miembro)
    .bind(nueva_asistencia.id_clase)
    .bind(nueva_asistencia.fecha_asistencia)
    .fetch_one(pool)
    .await
}

pub async fn actualizar(
    pool: &PgPool,
    id: i32,
    asistencia_actualizada: ActualizarAsistenciaClases,
) -> Result<AsistenciaClases, sqlx::Error> {
    sqlx::query_as::<_, AsistenciaClases>(
        "UPDATE Asistencia_Clases 
         SET id_miembro = $1, id_clase = $2, fecha_asistencia = COALESCE($3, fecha_asistencia) 
         WHERE id_asistencia = $4 
         RETURNING *",
    )
    .bind(asistencia_actualizada.id_miembro)
    .bind(asistencia_actualizada.id_clase)
    .bind(asistencia_actualizada.fecha_asistencia)
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn eliminar(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM Asistencia_Clases WHERE id_asistencia = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}