use crate::models::album::{ActualizarAlbum, Album, NuevoAlbum};
use crate::repository::album_repository::AlbumRepository;
use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::PgPool;

pub async fn obtener_albumes(State(pool): State<PgPool>) -> Json<Vec<Album>> {
    let repo = AlbumRepository::new(pool);
    match repo.obtener_albumes().await {
        Ok(albumes) => Json(albumes),
        Err(_) => Json(vec![]),
    }
}

pub async fn obtener_album_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Json<Option<Album>> {
    let repo = AlbumRepository::new(pool);
    match repo.obtener_album_por_id(id).await {
        Ok(album) => Json(album),
        Err(_) => Json(None),
    }
}

pub async fn agregar_album(
    State(pool): State<PgPool>,
    Json(nuevo): Json<NuevoAlbum>,
) -> Json<Option<Album>> {
    let repo = AlbumRepository::new(pool);
    match repo.agregar_album(nuevo).await {
        Ok(album) => Json(Some(album)),
        Err(_) => Json(Some(Album {
            id_album: 0,
            titulo: "Error al crear el álbum".to_string(),
            fecha_lanzamiento: None,
            id_artista: 0,
        })),
    }
}
pub async fn actualizar_album(
    State(pool): State<PgPool>,
    Path(id_album): Path<i32>,
    Json(album_actualizado): Json<ActualizarAlbum>,
) -> Json<Album> {
    let repo = AlbumRepository::new(pool);
    match repo.actualizar_album(id_album, album_actualizado).await {
        Ok(album) => Json(album),
        Err(_) => Json(Album {
            id_album: 0,
            titulo: "Error al actualizar el álbum".to_string(),
            fecha_lanzamiento: None,
            id_artista: 0,
        }),
    }
}

pub async fn eliminar_album(State(pool): State<PgPool>, Json(id_album): Json<i32>) -> Json<bool> {
    let repo = AlbumRepository::new(pool);
    match repo.eliminar_album(id_album).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}

pub async fn eliminar_album_por_id(
    State(pool): State<PgPool>,
    Path(id_album): Path<i32>,
) -> Json<bool> {
    let repo = AlbumRepository::new(pool);
    match repo.eliminar_album(id_album).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}
