use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Playlist {
    pub id_playlist: i32,
    pub nombre_lista: String,
    pub id_usuario: Option<i32>,
    pub fecha_creacion: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevaPlaylist {
    pub nombre_lista: String,
    pub id_usuario: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualizarPlaylist {
    pub nombre_lista: Option<String>,
    pub id_usuario: Option<i32>,
}