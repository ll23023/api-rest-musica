use crate::models::playlist::{ActualizarPlaylist, Playlist, NuevaPlaylist};
use crate::repository::PlaylistRepository;
use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::PgPool;

pub async fn obtener_playlists(State(pool): State<PgPool>) -> Json<Vec<Playlist>> {
    let repo = PlaylistRepository::new(pool);
    match repo.obtener_playlists().await {
        Ok(playlists) => Json(playlists),
        Err(_) => Json(vec![]),
    }
}

pub async fn obtener_playlist_por_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Json<Option<Playlist>> {
    let repo = PlaylistRepository::new(pool);
    match repo.obtener_playlist_por_id(id).await {
        Ok(playlist) => Json(playlist),
        Err(_) => Json(None),
    }
}

pub async fn agregar_playlist(
    State(pool): State<PgPool>,
    Json(nueva): Json<NuevaPlaylist>,
) -> Json<Option<Playlist>> {
    let repo = PlaylistRepository::new(pool);
    match repo.agregar_playlist(nueva).await {
        Ok(playlist) => Json(Some(playlist)),
        Err(_) => Json(Some(Playlist {
            id_playlist: 0,
            nombre_lista: "Error al crear la playlist".to_string(),
            id_usuario: None,
            fecha_creacion: None,
        })),
    }
}

pub async fn actualizar_playlist(
    State(pool): State<PgPool>,
    Path(id_playlist): Path<i32>,
    Json(playlist_actualizada): Json<ActualizarPlaylist>,
) -> Json<Playlist> {
    let repo = PlaylistRepository::new(pool);
    match repo.actualizar_playlist(id_playlist, playlist_actualizada).await {
        Ok(playlist) => Json(playlist),
        Err(_) => Json(Playlist {
            id_playlist: 0,
            nombre_lista: "Error al actualizar la playlist".to_string(),
            id_usuario: None,
            fecha_creacion: None,
        }),
    }
}

pub async fn eliminar_playlist(State(pool): State<PgPool>, Json(id_playlist): Json<i32>) -> Json<bool> {
    let repo = PlaylistRepository::new(pool);
    match repo.eliminar_playlist(id_playlist).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}

pub async fn eliminar_playlist_por_id(
    State(pool): State<PgPool>,
    Path(id_playlist): Path<i32>,
) -> Json<bool> {
    let repo = PlaylistRepository::new(pool);
    match repo.eliminar_playlist(id_playlist).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}