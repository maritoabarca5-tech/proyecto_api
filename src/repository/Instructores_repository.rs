use crate::models::Instructores::{
    ActualizarInstructor, ClaseResumen, InstructorConClases, Instructores, NuevoInstructor,
};
use sqlx::PgPool;

pub struct InstructoresRepository {
    pool: PgPool,
}

impl InstructoresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // Obtener todos los instructores
    pub async fn obtener_instructores(&self) -> sqlx::Result<Vec<Instructores>> {
        let instructores = sqlx::query_as::<_, Instructores>(
            "SELECT id_instructor, nombre, especialidad FROM Instructores ORDER BY id_instructor",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(instructores)
    }

    // Obtener un instructor por su ID
    pub async fn obtener_instructor_por_id(&self, id: i32) -> sqlx::Result<Instructores> {
        let instructor = sqlx::query_as::<_, Instructores>(
            "SELECT id_instructor, nombre, especialidad FROM Instructores WHERE id_instructor = $1",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        Ok(instructor)
    }

    // Crear un instructor nuevo
    pub async fn crear_instructor(&self, nuevo: NuevoInstructor) -> sqlx::Result<Instructores> {
        let instructor = sqlx::query_as::<_, Instructores>(
            "INSERT INTO Instructores (nombre, especialidad)
             VALUES ($1, $2)
             RETURNING id_instructor, nombre, especialidad",
        )
        .bind(&nuevo.nombre)
        .bind(&nuevo.especialidad)
        .fetch_one(&self.pool)
        .await?;
        Ok(instructor)
    }

    // Actualizar un instructor existente
    pub async fn actualizar_instructor(
        &self,
        id: i32,
        datos: ActualizarInstructor,
    ) -> sqlx::Result<Instructores> {
        let instructor = sqlx::query_as::<_, Instructores>(
            "UPDATE Instructores
             SET nombre = $1, especialidad = $2
             WHERE id_instructor = $3
             RETURNING id_instructor, nombre, especialidad",
        )
        .bind(&datos.nombre)
        .bind(&datos.especialidad)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        Ok(instructor)
    }

    // Eliminar un instructor (la BD pone NULL en Clases gracias al ON DELETE SET NULL)
    pub async fn eliminar_instructor(&self, id: i32) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM Instructores WHERE id_instructor = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // RELACION: Obtener un instructor junto con todas sus clases asignadas (JOIN con Clases)
    pub async fn obtener_instructor_con_clases(
        &self,
        id: i32,
    ) -> sqlx::Result<InstructorConClases> {
        // Primero traemos el instructor
        let instructor = self.obtener_instructor_por_id(id).await?;

        // Luego traemos sus clases usando JOIN
        let clases = sqlx::query_as::<_, ClaseResumen>(
            "SELECT id_clase, nombre_clase, horario
             FROM Clases
             WHERE id_instructor = $1
             ORDER BY id_clase",
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?;

        Ok(InstructorConClases {
            id_instructor: instructor.id_instructor,
            nombre: instructor.nombre,
            especialidad: instructor.especialidad,
            clases,
        })
    }
}
