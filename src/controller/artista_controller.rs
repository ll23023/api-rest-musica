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

    .route("/artistas", get(obtener_artistas))
        .route("/artistas/:nombre", get(obtener_artista_por_nombre))
        .route("/artistas", post(crear_artista))
        .route("/artistas", put(actualizar_artista))
        .route("/artistas", delete(eliminar_artista))
        .route("/artistas/id/:id_artista", delete(eliminar_artista_por_id))
        .with_state(pool)
}
