use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Struct principal: Representa una fila completa en la tabla Clases
#[derive(Clone, Serialize, Deserialize, FromRow, Debug, PartialEq)]
pub struct Clases {
    pub id_clase: Option<i32>,
    pub nombre_clase: String,
    pub id_instructor: Option<i32>, // Es Option porque permite valores NULL (ON DELETE SET NULL)
    pub horario: Option<String>,    // Es Option por si no se asigna horario al crearla
}

// Struct para crear una nueva clase (el id lo genera la base de datos automáticamente)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NuevaClase {
    pub nombre_clase: String,
    pub id_instructor: Option<i32>,
    pub horario: Option<String>,
}

// Struct para actualizar una clase existente
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ActualizarClase {
    pub nombre_clase: String,
    pub id_instructor: Option<i32>,
    pub horario: Option<String>,
}
