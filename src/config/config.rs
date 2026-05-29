use dotenvy::dotenv;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::env;
use std::time::Duration;

pub fn obtener_conexion() -> String {
    dotenv().ok();

    env::var("DATABASE_URL").unwrap_or_else(|_| {
        panic!(
            "DATABASE_URL no está configurada. Crea un archivo .env con una línea como:\nDATABASE_URL=postgres://usuario:contraseña@host:puerto/base_de_datos"
        )
    })
}

pub async fn crear_pool() -> sqlx::Result<sqlx::Pool<sqlx::Postgres>> {
    let database_url = obtener_conexion();
    let connect_options: PgConnectOptions = database_url.parse()?;

    PgPoolOptions::new()
        .max_connections(4)
        .acquire_timeout(Duration::from_secs(10))
        .connect_with(connect_options.statement_cache_capacity(0))
        .await
}
