use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Struct principal: representa una fila completa de la tabla Instructores
#[derive(Clone, Serialize, Deserialize, FromRow, Debug, PartialEq)]
pub struct Instructores {
    pub id_instructor: Option<i32>,
    pub nombre: String,
    pub especialidad: Option<String>, // Es Option porque la columna permite NULL
}

// Struct para crear un instructor nuevo (sin id, lo genera la BD)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NuevoInstructor {
    pub nombre: String,
    pub especialidad: Option<String>,
}

// Struct para actualizar un instructor existente
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ActualizarInstructor {
    pub nombre: String,
    pub especialidad: Option<String>,
}

// Struct especial para mostrar el instructor junto con sus clases asignadas (relacion con Clases)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstructorConClases {
    pub id_instructor: Option<i32>,
    pub nombre: String,
    pub especialidad: Option<String>,
    pub clases: Vec<ClaseResumen>, // Lista de clases que imparte este instructor
}

// Struct auxiliar que resume los datos de una clase (para no traer datos de mas)
#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct ClaseResumen {
    pub id_clase: Option<i32>,
    pub nombre_clase: String,
    pub horario: Option<String>,
}
