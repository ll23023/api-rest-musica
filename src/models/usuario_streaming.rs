use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct UsuarioStreaming {
    pub id_usuario: i32,
    pub nombre_usuario: String,
    pub tipo_suscripcion: TipoSuscripcion,
}

#[derive(Debug, Deserialize)]
pub struct NuevoUsuario {
    pub nombre_usuario: String,
    pub tipo_suscripcion: TipoSuscripcion,
}

#[derive(Debug, Deserialize)]
pub struct ActualizarUsuario {
    pub nombre_usuario: Option<String>,
    pub tipo_suscripcion: Option<TipoSuscripcion>,
}
