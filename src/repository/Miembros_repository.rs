use crate::models::Miembros::{ActualizarMiembro, Miembros, NuevoMiembro};
use sqlx::PgPool;

pub struct MiembrosRepository {
    pool: PgPool,
}

impl MiembrosRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // Obtener todos los miembros
    pub async fn obtener_miembros(&self) -> sqlx::Result<Vec<Miembros>> {
        let miembros = sqlx::query_as::<_, Miembros>(
            "SELECT 
                id_miembro, 
                nombre, 
                fecha_inscripcion::text AS fecha_inscripcion, 
                id_plan, 
                estado_membresia
             FROM Miembros
             ORDER BY id_miembro",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(miembros)
    }

    // Obtener un miembro por su ID
    pub async fn obtener_miembro_por_id(&self, id: i32) -> sqlx::Result<Miembros> {
        let miembro = sqlx::query_as::<_, Miembros>(
            "SELECT 
                id_miembro, 
                nombre, 
                fecha_inscripcion::text AS fecha_inscripcion, 
                id_plan, 
                estado_membresia
             FROM Miembros
             WHERE id_miembro = $1",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(miembro)
    }

    // Crear un miembro nuevo
    pub async fn crear_miembro(&self, nuevo: NuevoMiembro) -> sqlx::Result<Miembros> {
        let miembro = sqlx::query_as::<_, Miembros>(
            "INSERT INTO Miembros (nombre, id_plan, estado_membresia)
             VALUES ($1, $2, COALESCE($3, TRUE))
             RETURNING 
                id_miembro, 
                nombre, 
                fecha_inscripcion::text AS fecha_inscripcion, 
                id_plan, 
                estado_membresia",
        )
        .bind(&nuevo.nombre)
        .bind(nuevo.id_plan)
        .bind(nuevo.estado_membresia)
        .fetch_one(&self.pool)
        .await?;

        Ok(miembro)
    }

    // Actualizar un miembro existente
    pub async fn actualizar_miembro(
        &self,
        id: i32,
        datos: ActualizarMiembro,
    ) -> sqlx::Result<Miembros> {
        let miembro = sqlx::query_as::<_, Miembros>(
            "UPDATE Miembros
             SET nombre = $1,
                 id_plan = $2,
                 estado_membresia = COALESCE($3, estado_membresia)
             WHERE id_miembro = $4
             RETURNING 
                id_miembro, 
                nombre, 
                fecha_inscripcion::text AS fecha_inscripcion, 
                id_plan, 
                estado_membresia",
        )
        .bind(&datos.nombre)
        .bind(datos.id_plan)
        .bind(datos.estado_membresia)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(miembro)
    }

    // Eliminar un miembro
    // La tabla Asistencia_Clases elimina sus registros relacionados por ON DELETE CASCADE.
    pub async fn eliminar_miembro(&self, id: i32) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM Miembros WHERE id_miembro = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
