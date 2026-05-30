use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Serialize, Deserialize, FromRow, Debug, PartialEq)]
pub struct AsistenciaClases {
    pub id_asistencia: Option<i32>,
    pub id_miembro: i32,
    pub id_clase: i32,
    pub fecha_asistencia: Option<NaiveDate>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NuevaAsistenciaClases {
    pub id_miembro: i32,
    pub id_clase: i32,
    pub fecha_asistencia: Option<NaiveDate>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ActualizarAsistenciaClases {
    pub id_miembro: i32,
    pub id_clase: i32,
    pub fecha_asistencia: Option<NaiveDate>,
}