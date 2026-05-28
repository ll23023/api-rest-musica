use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::PgPool;
use crate::models::artista::{NuevoArtista, ActualizarArtista, Artista};
use crate::repository::artista_repository::ArtistaRepository;

pub async fn obtener_artistas(State(pool): State<PgPool>) -> Json<Vec<Artista>> {
    let repo = ArtistaRepository::new(pool);

    match repo.obtener_artistas().await {
        Ok(artistas) => Json(artistas),
        Err(_) => Json(vec![]),
    }
}

pub async fn obtener_artista_por_nombre(State(pool): State<PgPool>, Path(nombre): Path<String>) -> Json<Option<Artista>> {
    let repo = ArtistaRepository::new(pool);

    match repo.obtener_artista_por_nombre(&nombre).await {
        Ok(artista) => Json(artista),
        Err(_) => Json(None),
    }
}

pub async fn crear_artista(State(pool): State<PgPool>, Json(nuevo_artista): Json<NuevoArtista>) -> Json<Option<Artista>> {
    let repo = ArtistaRepository::new(pool);

    match repo.agregar_artista(nuevo_artista).await {
        Ok(artista) => Json(Some(artista)),
        Err(_) => Json(Some(Artista {
            id_artista: 0,
            nombre_artistico: "No se pudo crear el artista".to_string(),
            genero_principal: "Hubo un error al crear el artista".to_string(),
        })),
    }
}

pub async fn actualizar_artista(
    State(pool): State<PgPool>,
    Path(id_artista): Path<i32>,
    Json(artista_actualizado): Json<ActualizarArtista>,
) -> Json<Artista> {
    let repo = ArtistaRepository::new(pool);

    match repo.actualizar_artista(id_artista, artista_actualizado).await {
        Ok(artista) => Json(artista),
        Err(_) => Json(Artista {
            id_artista: 0,
            nombre_artistico: "No se pudo actualizar el artista".to_string(),
            genero_principal: "Hubo un error al actualizar el artista".to_string(),
        }),
    }
}

pub async fn eliminar_artista(State(pool): State<PgPool>, Path(id_artista): Path<i32>) -> Json<bool> {
    let repo = ArtistaRepository::new(pool);

    match repo.eliminar_artista(id_artista).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}

pub async fn eliminar_artista_por_id(State(pool): State<PgPool>, Path(id_artista): Path<i32>) -> Json<bool> {
    let repo = ArtistaRepository::new(pool);

    match repo.eliminar_artista_por_id(id_artista).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}