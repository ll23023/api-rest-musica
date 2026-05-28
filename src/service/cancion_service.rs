use crate::models::cancion::{ActualizarCancion, Cancion, NuevaCancion};
use crate::repository::cancion_repository::CancionRepository;
use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::PgPool;

pub async fn obtener_canciones(State(pool): State<PgPool>) -> Json<Vec<Cancion>> {
    let repo = CancionRepository::new(pool);
    match repo.obtener_canciones().await {
        Ok(canciones) => Json(canciones),
        Err(_) => Json(vec![]),
    }
}

pub async fn obtener_cancion_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Json<Option<Cancion>> {
    let repo = CancionRepository::new(pool);
    match repo.obtener_cancion_por_id(id).await {
        Ok(cancion) => Json(Some(cancion)),
        Err(_) => Json(None),
    }
}

pub async fn agregar_cancion(
    State(pool): State<PgPool>,
    Json(nueva): Json<NuevaCancion>,
) -> Json<Option<Cancion>> {
    let repo = CancionRepository::new(pool);
    match repo.agregar_cancion(nueva).await {
        Ok(cancion) => Json(Some(cancion)),
        Err(_) => Json(Some(Cancion {
            id_cancion: 0,
            nombre: "Error al crear la cancion".to_string(),
            duracion: None,
            id_album: None,
        })),
    }
}

pub async fn actualizar_cancion(
    State(pool): State<PgPool>,
    Path(id_cancion): Path<i32>,
    Json(cancion_actualizada): Json<ActualizarCancion>,
) -> Json<Cancion> {
    let repo = CancionRepository::new(pool);
    match repo
        .actualizar_cancion(id_cancion, cancion_actualizada)
        .await
    {
        Ok(cancion) => Json(cancion),
        Err(_) => Json(Cancion {
            id_cancion: 0,
            nombre: "Error al actualizar la cancion".to_string(),
            duracion: None,
            id_album: None,
        }),
    }
}

pub async fn eliminar_cancion(
    State(pool): State<PgPool>,
    Json(id_cancion): Json<i32>,
) -> Json<bool> {
    let repo = CancionRepository::new(pool);
    match repo.eliminar_cancion(id_cancion).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}

pub async fn eliminar_cancion_por_id(
    State(pool): State<PgPool>,
    Path(id_cancion): Path<i32>,
) -> Json<bool> {
    let repo = CancionRepository::new(pool);
    match repo.eliminar_cancion(id_cancion).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}
