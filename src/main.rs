mod config;
mod controller;
mod models;
mod repository;
mod service;
mod utils;

use config::config::crear_pool;
use controller::Instructores_controller::instructores_routes;
use controller::Miembros_controller::miembros_routes;
use controller::Planes_controller::planes_routes;

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
    let router2 = instructores_routes(pool.clone());
    let router3 = miembros_routes(pool.clone());
    router.merge(router2).merge(router3)
}
