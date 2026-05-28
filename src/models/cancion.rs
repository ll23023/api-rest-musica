use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Cancion {
    pub id_cancion: i32,
    pub nombre: String,
    pub duracion: Option<String>,
    pub id_album: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevaCancion {
    pub nombre: String,
    pub duracion: Option<String>,
    pub id_album: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualizarCancion {
    pub nombre: Option<String>,
    pub duracion: Option<String>,
    pub id_album: Option<i32>,
}
