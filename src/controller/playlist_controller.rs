use crate::service::playlist_service::{
    actualizar_playlist, agregar_playlist, eliminar_playlist, eliminar_playlist_por_id,
    obtener_playlist_por_id, obtener_playlists,
};
use axum::{
    Router,
    routing::{delete, get, post, put},
};
use sqlx::PgPool;

pub fn playlist_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/playlists", get(obtener_playlists))
        .route("/api/playlists", post(agregar_playlist))
        .route("/api/playlists", delete(eliminar_playlist))
        .route("/api/playlists/{id}", delete(eliminar_playlist_por_id))
        .route("/api/playlists", put(actualizar_playlist))
        .route("/api/playlists/{id}", get(obtener_playlist_por_id))
        .with_state(pool)
}