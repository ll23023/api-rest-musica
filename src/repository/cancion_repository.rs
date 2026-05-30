use crate::models::cancion::{ActualizarCancion, Cancion, NuevaCancion};
use sqlx::{PgPool, Row};

pub struct CancionRepository {
    pool: PgPool,
}

impl CancionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn obtener_canciones(&self) -> sqlx::Result<Vec<Cancion>> {
        let filas = sqlx::query("SELECT id_cancion, nombre, duracion, id_album FROM canciones")
            .fetch_all(&self.pool)
            .await?;
        let canciones = filas
            .into_iter()
            .map(|fila| Cancion {
                id_cancion: fila.get("id_cancion"),
                nombre: fila.get("nombre"),
                duracion: fila.get("duracion"),
                id_album: fila.get("id_album"),
            })
            .collect();
        Ok(canciones)
    }

    pub async fn obtener_cancion_por_id(&self, id: i32) -> sqlx::Result<Cancion> {
        let fila = sqlx::query(
            "SELECT id_cancion, nombre, duracion, id_album FROM canciones WHERE id_cancion = $1",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        Ok(Cancion {
            id_cancion: fila.get("id_cancion"),
            nombre: fila.get("nombre"),
            duracion: fila.get("duracion"),
            id_album: fila.get("id_album"),
        })
    }

    pub async fn agregar_cancion(&self, nueva: NuevaCancion) -> sqlx::Result<Cancion> {
        let fila = sqlx::query("INSERT INTO canciones (nombre, duracion, id_album) VALUES ($1, $2, $3) RETURNING id_cancion, nombre, duracion, id_album")
            .bind(nueva.nombre)
            .bind(nueva.duracion)
            .bind(nueva.id_album)
            .fetch_one(&self.pool)
            .await?;
        Ok(Cancion {
            id_cancion: fila.get("id_cancion"),
            nombre: fila.get("nombre"),
            duracion: fila.get("duracion"),
            id_album: fila.get("id_album"),
        })
    }

    pub async fn actualizar_cancion(
        &self,
        id: i32,
        datos: ActualizarCancion,
    ) -> sqlx::Result<Cancion> {
        let fila = sqlx::query("UPDATE canciones SET nombre = COALESCE($1, nombre), duracion = COALESCE($2, duracion), id_album = COALESCE($3, id_album) WHERE id_cancion = $4 RETURNING id_cancion, nombre, duracion, id_album")
            .bind(datos.nombre)
            .bind(datos.duracion)
            .bind(datos.id_album)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(Cancion {
            id_cancion: fila.get("id_cancion"),
            nombre: fila.get("nombre"),
            duracion: fila.get("duracion"),
            id_album: fila.get("id_album"),
        })
    }

    pub async fn eliminar_cancion(&self, id: i32) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM canciones WHERE id_cancion = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
