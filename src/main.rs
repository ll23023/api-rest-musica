mod service;
mod controller;
mod models;
mod repository;
mod config;

use controller::artista_controller::artista_router;
use controller::cancion_controller::cancion_router;
use controller::album_controller::album_router;

use config::config::crear_pool;

#[tokio::main]
async fn main() {
    let direccion = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(direccion)
    .await
    .expect("No se pudo iniciar el servidor");

    println!("Servidor escuchando en http://{}", direccion);

    let pool = crear_pool().await.expect("No se pudo conectar a la base de datos");

    axum::serve(listener, unificar_routers(pool))
        .await
        .expect("Error al ejecutar el servidor");
}

fn unificar_routers(pool: sqlx::PgPool) -> axum::Router {
    // Aquí se combinan los routers de cada entidad
    artista_router(pool.clone())
        .merge(cancion_router(pool.clone()))
        .merge(album_router(pool))
}

