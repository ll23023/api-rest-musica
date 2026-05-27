use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Artista {
    pub id_artista: i32,
    pub nombre_artistico: String,
    pub genero_principal: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevoArtista {
    pub nombre_artistico: String,
    pub genero_principal: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualizarArtista {
    pub nombre_artistico: String,
    pub genero_principal: String
}