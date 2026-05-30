use crate::models::usuario_streaming::{ActualizarUsuario, NuevoUsuario, UsuarioStreaming};
use crate::repository::usuario_streaming_repository::UsuarioStreamingRepository;

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use sqlx::PgPool;

pub async fn obtener_usuarios(State(pool): State<PgPool>) -> Json<Vec<UsuarioStreaming>> {
    match UsuarioStreamingRepository::obtener_todos_usuarios(&pool).await {
        Ok(usuarios) => Json(usuarios),
        Err(_) => Json(vec![]),
    }
}

pub async fn obtener_usuario_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Option<UsuarioStreaming>>, (StatusCode, String)> {
    match UsuarioStreamingRepository::obtener_usuario_id(&pool, id).await {
        Ok(usuario) => Ok(Json(usuario)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn obtener_usuario_por_nombre(
    State(pool): State<PgPool>,
    Path(nombre): Path<String>,
) -> Json<Option<UsuarioStreaming>> {
    match UsuarioStreamingRepository::obtener_usuario_nombre(&pool, &nombre).await {
        Ok(usuario) => Json(usuario),
        Err(_) => Json(None),
    }
}

pub async fn crear_usuario(
    State(pool): State<PgPool>,
    Json(nuevo_usuario): Json<NuevoUsuario>,
) -> Result<Json<UsuarioStreaming>, (StatusCode, String)> {
    match UsuarioStreamingRepository::crear_usuario(&pool, nuevo_usuario).await {
        Ok(usuario) => Ok(Json(usuario)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn actualizar_usuario(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(datos_actualizar): Json<ActualizarUsuario>,
) -> Json<UsuarioStreaming> {
    match UsuarioStreamingRepository::actualizar_usuario_existente(&pool, id, datos_actualizar)
        .await
    {
        Ok(Some(usuario)) => Json(usuario),
        Ok(None) => Json(UsuarioStreaming {
            id_usuario: 0,
            nombre_usuario: "Usuario no encontrado".to_string(),
            tipo_suscripcion: "Free".to_string(),
        }),
        Err(_) => Json(UsuarioStreaming {
            id_usuario: 0,
            nombre_usuario: "Error al actualizar el usuario".to_string(),
            tipo_suscripcion: "Free".to_string(),
        }),
    }
}

pub async fn contar_usuarios(State(pool): State<PgPool>) -> Json<i64> {
    match UsuarioStreamingRepository::contar(&pool).await {
        Ok(total) => Json(total),
        Err(_) => Json(0),
    }
}

pub async fn eliminar_usuario(
    State(pool): State<PgPool>,
    Json(id_usuario): Json<i32>,
) -> Json<bool> {
    match UsuarioStreamingRepository::eliminar_usuario(&pool, id_usuario).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}
pub async fn eliminar_usuario_por_id(
    State(pool): State<PgPool>,
    Path(id_usuario): Path<i32>,
) -> Json<bool> {
    match UsuarioStreamingRepository::eliminar_usuario(&pool, id_usuario).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}
