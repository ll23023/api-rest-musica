use sqlx::{PgPool, Row};
use crate::models::artista::{Artista, NuevoArtista, ActualizarArtista};

pub struct ArtistaRepository {
    pool: PgPool,
}

impl ArtistaRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn obtener_artistas(&self) -> sqlx::Result<Vec<Artista>> {
        let filas = sqlx::query("SELECT id_artista, nombre_artistico, genero_principal FROM artistas")
            .fetch_all(&self.pool)
            .await?;

        let artistas = filas.into_iter().map(|fila| Artista {
            id_artista: fila.get("id_artista"),
            nombre_artistico: fila.get("nombre_artistico"),
            genero_principal: fila.get("genero_principal"),
        }).collect();

        Ok(artistas)
    }

    pub async fn obtener_artista_por_nombre(&self, nombre_artistico: &str) -> sqlx::Result<Option<Artista>> {
        let fila = sqlx::query("SELECT id_artista, nombre_artistico, genero_principal FROM artistas WHERE nombre_artistico LIKE $1")
            .bind(format!("%{}%", nombre_artistico))
            .fetch_optional(&self.pool)
            .await?;

        if let Some(fila) = fila {
            Ok(Some(Artista {
                id_artista: fila.get("id_artista"),
                nombre_artistico: fila.get("nombre_artistico"),
                genero_principal: fila.get("genero_principal"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn agregar_artista(&self, nuevo_artista: NuevoArtista) -> sqlx::Result<Artista> {
        let fila = sqlx::query("INSERT INTO artistas (nombre_artistico, genero_principal) VALUES ($1, $2) RETURNING id_artista, nombre_artistico, genero_principal")
            .bind(nuevo_artista.nombre_artistico)
            .bind(nuevo_artista.genero_principal)
            .fetch_one(&self.pool)
            .await?;

        Ok(Artista {
            id_artista: fila.get("id_artista"),
            nombre_artistico: fila.get("nombre_artistico"),
            genero_principal: fila.get("genero_principal"),
        })
    }

    pub async fn actualizar_artista(&self, id_artista: i32, artista_actualizado: ActualizarArtista) -> sqlx::Result<Artista> {
        let fila = sqlx::query("UPDATE artistas SET nombre_artistico = COALESCE($1, nombre_artistico), genero_principal = COALESCE($2, genero_principal) WHERE id_artista = $3 RETURNING id_artista, nombre_artistico, genero_principal")
            .bind(artista_actualizado.nombre_artistico)
            .bind(artista_actualizado.genero_principal)
            .bind(id_artista)
            .fetch_one(&self.pool)
            .await?;

        Ok(Artista {
            id_artista: fila.get("id_artista"),
            nombre_artistico: fila.get("nombre_artistico"),
            genero_principal: fila.get("genero_principal"),
        })
    }

    pub async fn eliminar_artista(&self, id_artista: i32) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM artistas WHERE id_artista = $1")
            .bind(id_artista)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn eliminar_artista_por_id(&self, id_artista: i32) -> sqlx::Result<()> {
        self.eliminar_artista(id_artista).await
    }
}

