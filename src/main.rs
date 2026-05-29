mod config;
mod controller;
mod models;
mod repository;
mod service;

use controller::album_controller::album_router;
use controller::artista_controller::artista_router;
use controller::cancion_controller::cancion_router;
use controller::usuario_streaming_controller::usuario_streaming_router;

use config::config::crear_pool;

#[tokio::main]
async fn main() {
    let direccion = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(direccion)
        .await
        .expect("No se pudo iniciar el servidor");

    println!("Servidor escuchando en http://{}", direccion);

    let pool = match crear_pool().await {
        Ok(pool) => pool,
        Err(error) => {
            eprintln!(
                "Error al conectar a la base de datos: {}\nRevisa tu variable DATABASE_URL en el archivo .env o en el entorno.",
                error
            );
            return;
        }
    };

    axum::serve(listener, unificar_routers(pool))
        .await
        .expect("Error al ejecutar el servidor");
}

fn unificar_routers(pool: sqlx::PgPool) -> axum::Router {
    artista_router(pool.clone())
        .merge(cancion_router(pool.clone()))
        .merge(album_router(pool.clone()))
        .merge(usuario_streaming_router(pool))
}
