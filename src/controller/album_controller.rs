use crate::service::album_service::{
    actualizar_album, agregar_album, eliminar_album, eliminar_album_por_id, obtener_album_por_id,
    obtener_albumes,
};
use axum::{
    Router,
    routing::{delete, get, post, put},
};
use sqlx::PgPool;

pub fn album_router(pool: PgPool) -> Router {
    Router::new()
        .route("/albumes", get(obtener_albumes))
        .route("/albumes", post(agregar_album))
        .route("/albumes", delete(eliminar_album))
        .route("/albumes/{id}", delete(eliminar_album_por_id))
        .route("/albumes", put(actualizar_album))
        .route("/albumes/{id}", get(obtener_album_por_id))
        .with_state(pool)
}
