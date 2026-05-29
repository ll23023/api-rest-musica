use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::PgPool;
use crate::service::artista_service::{
    obtener_artistas,
    obtener_artista_por_nombre,
    crear_artista,
    actualizar_artista,
    eliminar_artista,
    eliminar_artista_por_id,
};

pub fn artista_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/artistas", get(obtener_artistas))
        .route("/api/artistas/nombre/{nombre}", get(obtener_artista_por_nombre))
        .route("/api/artistas", post(crear_artista))
        .route("/api/artistas/{id_artista}", put(actualizar_artista))
        .route("/api/artistas", delete(eliminar_artista))
        .route("/api/artistas/id/{id_artista}", delete(eliminar_artista_por_id))
        .with_state(pool)
}
