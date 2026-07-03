//Importamos las librerias a usar
use askama::Template;
use askama_web::WebTemplate; 
use axum::{response::IntoResponse, routing::get, Router};

// Al agregar WebTemplate al derive, se implementa automáticamente IntoResponse
#[derive(Template, WebTemplate)]
#[template(path = "inicio.html")]
struct InicioTemplate {
    establecimiento: String,
}

async fn handler_inicio() -> impl IntoResponse {
    InicioTemplate {
        establecimiento: "Amazon, Soriana y el Mercado".to_string(),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler_inicio));

    let direccion = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(direccion).await.unwrap();
    println!("🚀 Servidor web listo en http://{}", direccion);

    axum::serve(listener, app).await.unwrap();
}