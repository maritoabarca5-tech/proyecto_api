use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Struct principal: representa una fila completa de la tabla Miembros
#[derive(Clone, Serialize, Deserialize, FromRow, Debug, PartialEq)]
pub struct Miembros {
    pub id_miembro: Option<i32>,
    pub nombre: String,
    pub fecha_inscripcion: Option<String>,
    pub id_plan: Option<i32>,
    pub estado_membresia: Option<bool>,
}

// Struct para crear un miembro nuevo
// No se manda id_miembro porque lo genera la base de datos.
// No se manda fecha_inscripcion porque la BD coloca CURRENT_DATE por defecto.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NuevoMiembro {
    pub nombre: String,
    pub id_plan: Option<i32>,
    pub estado_membresia: Option<bool>,
}

// Struct para actualizar un miembro existente
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ActualizarMiembro {
    pub nombre: String,
    pub id_plan: Option<i32>,
    pub estado_membresia: Option<bool>,
}
