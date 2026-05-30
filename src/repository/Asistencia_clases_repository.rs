use sqlx::PgPool;
use crate::models::Asistencia_Clases::{AsistenciaClases, NuevaAsistenciaClases, ActualizarAsistenciaClases};

pub struct AsistenciaClasesRepository {
    pool: PgPool,
}

impl AsistenciaClasesRepository {

    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn obtener_asistencias(&self) -> sqlx::Result<Vec<AsistenciaClases>> {
        let asistencias = sqlx::query_as::<_, AsistenciaClases>(
            "SELECT id_asistencia, id_miembro, id_clase, fecha_asistencia FROM Asistencia_Clases"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(asistencias)
    }

    pub async fn obtener_asistencia_por_id(&self, id_asistencia: i32) -> sqlx::Result<AsistenciaClases> {
        let asistencia = sqlx::query_as::<_, AsistenciaClases>(
            "SELECT id_asistencia, id_miembro, id_clase, fecha_asistencia FROM Asistencia_Clases WHERE id_asistencia = $1"
        )
        .bind(id_asistencia)
        .fetch_one(&self.pool)
        .await?;

        Ok(asistencia)
    }

    pub async fn crear_asistencia(&self, nueva: NuevaAsistenciaClases) -> sqlx::Result<AsistenciaClases> {
        let asistencia = sqlx::query_as::<_, AsistenciaClases>(
            "INSERT INTO Asistencia_Clases (id_miembro, id_clase, fecha_asistencia)
             VALUES ($1, $2, COALESCE($3, CURRENT_DATE))
             RETURNING id_asistencia, id_miembro, id_clase, fecha_asistencia"
        )
        .bind(nueva.id_miembro)
        .bind(nueva.id_clase)
        .bind(nueva.fecha_asistencia)
        .fetch_one(&self.pool)
        .await?;

        Ok(asistencia)
    }

    pub async fn actualizar_asistencia(&self, id_asistencia: i32, datos: ActualizarAsistenciaClases) -> sqlx::Result<AsistenciaClases> {
        let asistencia = sqlx::query_as::<_, AsistenciaClases>(
            "UPDATE Asistencia_Clases
             SET id_miembro = $1, id_clase = $2, fecha_asistencia = $3
             WHERE id_asistencia = $4
             RETURNING id_asistencia, id_miembro, id_clase, fecha_asistencia"
        )
        .bind(datos.id_miembro)
        .bind(datos.id_clase)
        .bind(datos.fecha_asistencia)
        .bind(id_asistencia)
        .fetch_one(&self.pool)
        .await?;

        Ok(asistencia)
    }

    pub async fn eliminar_asistencia(&self, id_asistencia: i32) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM Asistencia_Clases WHERE id_asistencia = $1")
            .bind(id_asistencia)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}