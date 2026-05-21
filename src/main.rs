mod service;
mod utils;
mod models;
mod controller;
mod config;
mod repository;

use controller::Planes_controller::planes_routes;
use config::config::crear_pool;

#[tokio::main]
async fn main() {
    let direccion = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(direccion)
        .await
        .expect("No se pudo enlazar el puerto 3000");

    println!("Servidor escuchando en http://{direccion}");

    let pool = crear_pool()
        .await
        .expect("No se pudo conectar a la base de datos");

    axum::serve(listener, unificar_routers(pool))
        .await
        .expect("Error al iniciar el servidor");
}

fn unificar_routers(pool: sqlx::PgPool) -> axum::Router {
    // Por ahora solo tenemos Planes, los compañeros agregarán sus routers aquí
    let router = planes_routes(pool.clone());
    // Ejemplo de cómo agregar más routers:
    // let router2 = instructores_routes(pool.clone());
    // router.merge(router2)
    router
}