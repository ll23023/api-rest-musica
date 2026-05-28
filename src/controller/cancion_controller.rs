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
        .route("/canciones", get(obtener_canciones))
        .route("/canciones", post(agregar_cancion))
        .route("/canciones", delete(eliminar_cancion))
        .route("/canciones/:id", delete(eliminar_cancion_por_id))
        .route("/canciones", put(actualizar_cancion))
        .route("/canciones/:id", get(obtener_cancion_por_id))
        .with_state(pool)
}
