use crate::models::album::{ActualizarAlbum, Album, NuevoAlbum};
use sqlx::{PgPool, Row};

pub struct AlbumRepository {
    pool: PgPool,
}

impl AlbumRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn obtener_albumes(&self) -> sqlx::Result<Vec<Album>> {
        let filas =
            sqlx::query("SELECT id_album, titulo, fecha_lanzamiento, id_artista FROM albumes")
                .fetch_all(&self.pool)
                .await?;

        let albumes = filas
            .into_iter()
            .map(|fila| Album {
                id_album: fila.get("id_album"),
                titulo: fila.get("titulo"),
                fecha_lanzamiento: fila.get("fecha_lanzamiento"),
                id_artista: fila.get("id_artista"),
            })
            .collect();

        Ok(albumes)
    }

    pub async fn obtener_album_por_id(&self, id_album: i32) -> sqlx::Result<Option<Album>> {
        let fila = sqlx::query(
            "SELECT id_album, titulo, fecha_lanzamiento, id_artista FROM albumes WHERE id_album = $1"
        )
        .bind(id_album)
        .fetch_optional(&self.pool)
        .await?;

        Ok(fila.map(|fila| Album {
            id_album: fila.get("id_album"),
            titulo: fila.get("titulo"),
            fecha_lanzamiento: fila.get("fecha_lanzamiento"),
            id_artista: fila.get("id_artista"),
        }))
    }

    pub async fn agregar_album(&self, nuevo_album: NuevoAlbum) -> sqlx::Result<Album> {
        let fila = sqlx::query(
            "INSERT INTO albumes (titulo, fecha_lanzamiento, id_artista)
             VALUES ($1, $2, $3)
             RETURNING id_album, titulo, fecha_lanzamiento, id_artista",
        )
        .bind(nuevo_album.titulo)
        .bind(nuevo_album.fecha_lanzamiento)
        .bind(nuevo_album.id_artista)
        .fetch_one(&self.pool)
        .await?;

        Ok(Album {
            id_album: fila.get("id_album"),
            titulo: fila.get("titulo"),
            fecha_lanzamiento: fila.get("fecha_lanzamiento"),
            id_artista: fila.get("id_artista"),
        })
    }

    pub async fn actualizar_album(
        &self,
        id_album: i32,
        album_actualizado: ActualizarAlbum,
    ) -> sqlx::Result<Album> {
        let fila = sqlx::query(
            "UPDATE albumes SET titulo = $1, fecha_lanzamiento = $2, id_artista = $3
             WHERE id_album = $4
             RETURNING id_album, titulo, fecha_lanzamiento, id_artista",
        )
        .bind(album_actualizado.titulo)
        .bind(album_actualizado.fecha_lanzamiento)
        .bind(album_actualizado.id_artista)
        .bind(id_album)
        .fetch_one(&self.pool)
        .await?;

        Ok(Album {
            id_album: fila.get("id_album"),
            titulo: fila.get("titulo"),
            fecha_lanzamiento: fila.get("fecha_lanzamiento"),
            id_artista: fila.get("id_artista"),
        })
    }

    pub async fn eliminar_album(&self, id_album: i32) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM albumes WHERE id_album = $1")
            .bind(id_album)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
