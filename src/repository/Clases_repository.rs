use crate::models::Clases::{Clases, NuevaClase, ActualizarClase};
use sqlx::PgPool;

pub struct ClasesRepository {
    pool: PgPool,
}

impl ClasesRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // C: Crear clase
    pub async fn crear_clase(&self, nueva: NuevaClase) -> sqlx::Result<Clases> {
        let clase = sqlx::query_as::<_, Clases>(
            "INSERT INTO Clases (nombre_clase, id_instructor, horario)
             VALUES ($1, $2, $3)
             RETURNING id_clase, nombre_clase, id_instructor, horario",
        )
        .bind(&nueva.nombre_clase)
        .bind(nueva.id_instructor)
        .bind(&nueva.horario)
        .fetch_one(&self.pool)
        .await?;
        Ok(clase)
    }

    // R: Obtener todas las clases
    pub async fn obtener_clases(&self) -> sqlx::Result<Vec<Clases>> {
        let clases = sqlx::query_as::<_, Clases>(
            "SELECT id_clase, nombre_clase, id_instructor, horario FROM Clases ORDER BY id_clase",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(clases)
    }

    // R: Obtener clase por ID
    pub async fn obtener_clase_por_id(&self, id: i32) -> sqlx::Result<Clases> {
        let clase = sqlx::query_as::<_, Clases>(
            "SELECT id_clase, nombre_clase, id_instructor, horario FROM Clases WHERE id_clase = $1",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        Ok(clase)
    }

    // U: Actualizar clase por ID
    pub async fn actualizar_clase(&self, id: i32, datos: ActualizarClase) -> sqlx::Result<Clases> {
        let clase = sqlx::query_as::<_, Clases>(
            "UPDATE Clases
             SET nombre_clase = $1, id_instructor = $2, horario = $3
             WHERE id_clase = $4
             RETURNING id_clase, nombre_clase, id_instructor, horario",
        )
        .bind(&datos.nombre_clase)
        .bind(datos.id_instructor)
        .bind(&datos.horario)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        Ok(clase)
    }

    // D: Eliminar clase por ID
    pub async fn eliminar_clase(&self, id: i32) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM Clases WHERE id_clase = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
