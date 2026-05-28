use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

pub fn obtener_conexion() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL no está configurada")
}

pub async fn crear_pool() -> sqlx::Result<sqlx::Pool<sqlx::Postgres>> {
    let database_url = obtener_conexion();
    
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}