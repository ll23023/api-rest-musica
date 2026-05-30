use crate::service::cancion_service::{
    actualizar_cancion, agregar_cancion, eliminar_cancion, eliminar_cancion_por_id,
    obtener_cancion_por_id, obtener_canciones,
};
use axum::{
    Router,
    routing::{delete, get, post, put},
};
use sqlx::PgPool;

pub fn cancion_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/canciones", get(obtener_canciones))
        .route("/api/canciones", post(agregar_cancion))
        .route("/api/canciones", delete(eliminar_cancion))
        .route("/api/canciones/{id}", delete(eliminar_cancion_por_id))
        .route("/api/canciones/{id}", put(actualizar_cancion))
        .route("/api/canciones/{id}", get(obtener_cancion_por_id))
        .with_state(pool)
}
