use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Clone, Serialize, Deserialize, FromRow, Debug, PartialEq)]
pub struct Planes {
    pub id_plan: Option<i32>,
    pub nombre_plan: String,
    pub precio_mensual: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NuevoPlanes {
    pub nombre_plan: String,
    pub precio_mensual: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ActualizarPlanes {
    pub nombre_plan: String,
    pub precio_mensual: f64,
}
