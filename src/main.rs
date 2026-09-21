use axum::{
    routing::{get, post},
    Json, Router,
};
use sqlx::MySqlPool;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Cargar variables de entorno si usas .env
    dotenvy::dotenv().ok();

    // Conexión a MySQL (ajusta tu URL de conexión según tu base de datos)
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://usuario:password@localhost:3306/loteria_db".to_string());
    
    let pool = MySqlPool::connect(&database_url).await?;
    println!("¡Conectado exitosamente a la base de datos MySQL!");

    // Configurar rutas de la API y el servicio de archivos estáticos del Frontend
    let app = Router::new()
        // Servir los archivos estáticos de la interfaz (HTML, CSS, JS) desde la carpeta "dist"
        .nest_service("/", ServeDir::new("dist"))
        // Tus rutas de API (ejemplo)
        .route("/api/health", get(|| async { "API funcionando al 100%" }))
        // Habilitar CORS para evitar problemas de peticiones
        .layer(CorsLayer::permissive())
        .with_state(pool);

    // Escuchar en todas las interfaces en el puerto 3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Servidor corriendo en http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}