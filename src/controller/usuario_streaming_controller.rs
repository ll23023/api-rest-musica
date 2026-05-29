use axum::{
    Router,
    routing::{get, post, put},
};
use sqlx::PgPool;

use crate::service::usuario_streaming_service::{
    actualizar_usuario, contar_usuarios, crear_usuario, obtener_usuario_por_id,
    obtener_usuario_por_nombre, obtener_usuarios,
};

pub fn usuario_streaming_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/usuarios", get(obtener_usuarios))
        .route("/api/usuarios", post(crear_usuario))
        .route("/api/usuarios/actualizar/{id}", put(actualizar_usuario))
        .route("/api/usuarios/{id}", get(obtener_usuario_por_id))
        .route(
            "/api/usuarios/nombre/{nombre}",
            get(obtener_usuario_por_nombre),
        )
        .route("/api/usuarios/contar", get(contar_usuarios))
        .with_state(pool)
}
