#![allow(dead_code)]

use crate::models::usuario_streaming::{ActualizarUsuario, NuevoUsuario, UsuarioStreaming};
use sqlx::{Error, PgPool, Result, Row};

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
        let fila = sqlx::query(
                "UPDATE Usuarios_Streaming
                 SET nombre_usuario = COALESCE($1, nombre_usuario), tipo_suscripcion = COALESCE($2, tipo_suscripcion)
                 WHERE id_usuario = $3
                 RETURNING id_usuario, nombre_usuario, tipo_suscripcion",
            )
            .bind(datos.nombre_usuario)
            .bind(datos.tipo_suscripcion)
            .bind(id)
            .fetch_optional(pool)
            .await?;

        if let Some(fila) = fila {
            Ok(Some(UsuarioStreaming {
                id_usuario: fila.get("id_usuario"),
                nombre_usuario: fila.get("nombre_usuario"),
                tipo_suscripcion: fila.get("tipo_suscripcion"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn contar(pool: &PgPool) -> Result<i64> {
        let calcular: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM Usuarios_Streaming")
            .fetch_one(pool)
            .await?;

        Ok(calcular.0)
    }

    pub async fn eliminar_usuario(pool: &PgPool, id: i32) -> Result<()> {
        sqlx::query("DELETE FROM Usuarios_Streaming WHERE id_usuario = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
