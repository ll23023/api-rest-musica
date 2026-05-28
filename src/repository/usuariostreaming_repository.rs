use crate::models::usuario_streaming::{ActualizarUsuario, NuevoUsuario, UsuarioStreaming};
use sqlx::{Error, PgPool, Result};

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
        let existe = Self::obtener_usuario_id(pool, id).await?;
        if existe.is_none() {
            return Ok(None);
        }

        let mut actualizar = Vec::new();
        let mut params: Vec<String> = Vec::new();
        let mut contador = 1;

        if let Some(nombre) = &datos.nombre_usuario {
            actualizar.push(format!("nombre_usuario = ${}", contador));
            params.push(nombre.clone());
            contador += 1;
        }

        if let Some(tipo) = &datos.tipo_suscripcion {
            actualizar.push(format!("tipo_suscripcion = ${}", contador));
            params.push(tipo.to_string());
            contador += 1;
        }

        if actualizar.is_empty() {
            return Self::obtener_usuario_id(pool, id).await;
        }

        let query = format!(
            "UPDATE Usuarios_Streaming
             SET {}
             WHERE id_usuario = ${}
             RETURNING id_usuario, nombre_usuario, tipo_suscripcion",
            actualizar.join(", "),
            contador
        );

        let mut consulta_params = sqlx::query_as::<_, UsuarioStreaming>(&query);
        for param in params {
            consulta_params = consulta_params.bind(param);
        }
        consulta_params = consulta_params.bind(id);

        let usuario_actualizado = consulta_params.fetch_optional(pool).await?;

        Ok(usuario_actualizado)
    }

    pub async fn eliminar(pool: &PgPool, id: i32) -> Result<bool> {
        let resultado = sqlx::query("DELETE FROM Usuarios_Streaming WHERE id_usuario = $1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(resultado.rows_affected() > 0)
    }

    pub async fn contar(pool: &PgPool) -> Result<i64> {
        let calcular: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM Usuarios_Streaming")
            .fetch_one(pool)
            .await?;

        Ok(calcular.0)
    }
}
