use crate::models::playlist::{ActualizarPlaylist, Playlist, NuevaPlaylist};
use sqlx::{PgPool, Row};

pub struct PlaylistRepository {
    pool: PgPool,
}

impl PlaylistRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn obtener_playlists(&self) -> sqlx::Result<Vec<Playlist>> {
        let filas = sqlx::query("SELECT id_playlist, nombre_lista, id_usuario, fecha_creacion FROM playlists")
            .fetch_all(&self.pool)
            .await?;

        let playlists = filas
            .into_iter()
            .map(|fila| Playlist {
                id_playlist: fila.get("id_playlist"),
                nombre_lista: fila.get("nombre_lista"),
                id_usuario: fila.get("id_usuario"),
                fecha_creacion: fila.get("fecha_creacion"),
            })
            .collect();

        Ok(playlists)
    }

    pub async fn obtener_playlist_por_id(&self, id_playlist: i32) -> sqlx::Result<Option<Playlist>> {
        let fila = sqlx::query(
            "SELECT id_playlist, nombre_lista, id_usuario, fecha_creacion FROM playlists WHERE id_playlist = $1"
        )
        .bind(id_playlist)
        .fetch_optional(&self.pool)
        .await?;

        Ok(fila.map(|f| Playlist {
            id_playlist: f.get("id_playlist"),
            nombre_lista: f.get("nombre_lista"),
            id_usuario: f.get("id_usuario"),
            fecha_creacion: f.get("fecha_creacion"),
        }))
    }

    pub async fn agregar_playlist(&self, nueva: NuevaPlaylist) -> sqlx::Result<Playlist> {
        let fila = sqlx::query(
            "INSERT INTO playlists (nombre_lista, id_usuario) 
             VALUES ($1, $2) 
             RETURNING id_playlist, nombre_lista, id_usuario, fecha_creacion"
        )
        .bind(nueva.nombre_lista)
        .bind(nueva.id_usuario)
        .fetch_one(&self.pool)
        .await?;

        Ok(Playlist {
            id_playlist: fila.get("id_playlist"),
            nombre_lista: fila.get("nombre_lista"),
            id_usuario: fila.get("id_usuario"),
            fecha_creacion: fila.get("fecha_creacion"),
        })
    }

    pub async fn actualizar_playlist(
        &self,
        id_playlist: i32,
        datos: ActualizarPlaylist,
    ) -> sqlx::Result<Playlist> {
        let fila = sqlx::query(
            "UPDATE playlists 
             SET nombre_lista = COALESCE($1, nombre_lista), id_usuario = COALESCE($2, id_usuario) 
             WHERE id_playlist = $3 
             RETURNING id_playlist, nombre_lista, id_usuario, fecha_creacion"
        )
        .bind(datos.nombre_lista)
        .bind(datos.id_usuario)
        .bind(id_playlist)
        .fetch_one(&self.pool)
        .await?;

        Ok(Playlist {
            id_playlist: fila.get("id_playlist"),
            nombre_lista: fila.get("nombre_lista"),
            id_usuario: fila.get("id_usuario"),
            fecha_creacion: fila.get("fecha_creacion"),
        })
    }

    pub async fn eliminar_playlist(&self, id_playlist: i32) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM playlists WHERE id_playlist = $1")
            .bind(id_playlist)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}