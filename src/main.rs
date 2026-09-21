use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

#[derive(Deserialize)]
struct RegistroPayload {
    nombre_usuario: String,
    correo: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginPayload {
    nombre_usuario: String,
    password: String,
}

#[derive(Serialize)]
struct RespuestaApi {
    exito: bool,
    mensaje: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

   let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://admin_db:Password123*@100.70.178.71:2009/centro_juegos".to_string());
    
    let pool = MySqlPool::connect(&database_url).await?;
    println!("¡Conectado exitosamente a la base de datos MySQL!");

    let app = Router::new()
        .nest_service("/", ServeDir::new("dist"))
        .route("/api/health", get(|| async { "API funcionando al 100%" }))
        .route("/api/registro", post(manejar_registro))
        .route("/api/login", post(manejar_login))
        .layer(CorsLayer::permissive())
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Servidor corriendo en http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn manejar_registro(
    State(pool): State<MySqlPool>,
    Json(payload): Json<RegistroPayload>,
) -> Json<RespuestaApi> {
    let resultado = sqlx::query(
        "INSERT INTO usuarios (nombre_usuario, correo, password_hash) VALUES (?, ?, ?)"
    )
    .bind(&payload.nombre_usuario)
    .bind(&payload.correo)
    .bind(&payload.password)
    .execute(&pool)
    .await;

    match resultado {
        Ok(_) => Json(RespuestaApi {
            exito: true,
            mensaje: "Usuario registrado correctamente".to_string(),
        }),
        Err(e) => Json(RespuestaApi {
            exito: false,
            mensaje: format!("Error al registrar (usuario o correo duplicado): {}", e),
        }),
    }
}

async fn manejar_login(
    State(pool): State<MySqlPool>,
    Json(payload): Json<LoginPayload>,
) -> Json<RespuestaApi> {
    let resultado = sqlx::query_scalar::<_, String>(
        "SELECT nombre_usuario FROM usuarios WHERE nombre_usuario = ? AND password_hash = ?"
    )
    .bind(&payload.nombre_usuario)
    .bind(&payload.password)
    .fetch_optional(&pool)
    .await;

    match resultado {
        Ok(Some(_)) => Json(RespuestaApi {
            exito: true,
            mensaje: "Login exitoso".to_string(),
        }),
        Ok(None) => Json(RespuestaApi {
            exito: false,
            mensaje: "Usuario o contraseña incorrectos".to_string(),
        }),
        Err(e) => Json(RespuestaApi {
            exito: false,
            mensaje: format!("Error en el servidor: {}", e),
        }),
    }
}