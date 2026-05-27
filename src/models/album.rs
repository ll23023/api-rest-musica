use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Album {
    pub id_album: i32,
    pub titulo: String,
    pub fecha_lanzamiento: Option<NaiveDate>,
    pub id_artista: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevoAlbum {
    pub titulo: String,
    pub fecha_lanzamiento: Option<NaiveDate>,
    pub id_artista: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualizarAlbum {
    pub titulo: String,
    pub fecha_lanzamiento: Option<NaiveDate>,
    pub id_artista: i32,
}
