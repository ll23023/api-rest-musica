#![allow(dead_code)]

use crate::models::usuario_streaming::{ActualizarUsuario, NuevoUsuario, UsuarioStreaming};
use sqlx::{PgPool, Result};

pub struct UsuarioStreamingRepository;

impl UsuarioStreamingRepository {
    pub async fn obtener_todos_usuarios(pool: &PgPool) -> Result<Vec<UsuarioStreaming>> {
        let usuarios = sqlx::query_as::<_, UsuarioStreaming>(
            "SELECT id_usuario, nombre_usuario, tipo_suscripcion FROM Usuarios_Streaming ORDER BY id_usuario"
        )
        .fetch_all(pool)
        .await?;
        Ok(usuarios)
    }

    pub async fn obtener_usuario_id(pool: &PgPool, id: i32) -> Result<Option<UsuarioStreaming>> {
        let usuario = sqlx::query_as::<_, UsuarioStreaming>(
            "SELECT id_usuario, nombre_usuario, tipo_suscripcion FROM Usuarios_Streaming WHERE id_usuario = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(usuario)
    }

    pub async fn obtener_usuario_nombre(
        pool: &PgPool,
        nombre: &str,
    ) -> Result<Option<UsuarioStreaming>> {
        let usuario = sqlx::query_as::<_, UsuarioStreaming>(
            "SELECT id_usuario, nombre_usuario, tipo_suscripcion FROM Usuarios_Streaming WHERE nombre_usuario = $1"
        )
        .bind(nombre)
        .fetch_optional(pool)
        .await?;
        Ok(usuario)
    }

    pub async fn crear_usuario(
        pool: &PgPool,
        nuevo_usuario: NuevoUsuario,
    ) -> Result<UsuarioStreaming> {
        let usuario = sqlx::query_as::<_, UsuarioStreaming>(
            "INSERT INTO Usuarios_Streaming (nombre_usuario, tipo_suscripcion)
             VALUES ($1, $2)
             RETURNING id_usuario, nombre_usuario, tipo_suscripcion",
        )
        .bind(&nuevo_usuario.nombre_usuario)
        .bind(&nuevo_usuario.tipo_suscripcion)
        .fetch_one(pool)
        .await?;
        Ok(usuario)
    }

    pub async fn actualizar_usuario_existente(
        pool: &PgPool,
        id: i32,
        datos: ActualizarUsuario,
    ) -> Result<Option<UsuarioStreaming>> {
        match (datos.nombre_usuario, datos.tipo_suscripcion) {
            (Some(nombre), Some(tipo)) => {
                let usuario = sqlx::query_as::<_, UsuarioStreaming>(
                    "UPDATE Usuarios_Streaming
                     SET nombre_usuario = $1, tipo_suscripcion = $2
                     WHERE id_usuario = $3
                     RETURNING id_usuario, nombre_usuario, tipo_suscripcion",
                )
                .bind(nombre)
                .bind(tipo)
                .bind(id)
                .fetch_optional(pool)
                .await?;
                Ok(usuario)
            }
            (Some(nombre), None) => {
                let usuario = sqlx::query_as::<_, UsuarioStreaming>(
                    "UPDATE Usuarios_Streaming
                     SET nombre_usuario = $1
                     WHERE id_usuario = $2
                     RETURNING id_usuario, nombre_usuario, tipo_suscripcion",
                )
                .bind(nombre)
                .bind(id)
                .fetch_optional(pool)
                .await?;
                Ok(usuario)
            }
            (None, Some(tipo)) => {
                let usuario = sqlx::query_as::<_, UsuarioStreaming>(
                    "UPDATE Usuarios_Streaming
                     SET tipo_suscripcion = $1
                     WHERE id_usuario = $2
                     RETURNING id_usuario, nombre_usuario, tipo_suscripcion",
                )
                .bind(tipo)
                .bind(id)
                .fetch_optional(pool)
                .await?;
                Ok(usuario)
            }
            (None, None) => Self::obtener_usuario_id(pool, id).await,
        }
    }

    pub async fn contar(pool: &PgPool) -> Result<i64> {
        let calcular: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM Usuarios_Streaming")
            .fetch_one(pool)
            .await?;

        Ok(calcular.0)
    }
}
