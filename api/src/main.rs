#[path = "routes/coffee.rs"]
mod coffee;

use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/coffees", get(coffee::get_coffees))
        .route("/coffee", post(coffee::create_coffee));

    // run our app with hyper
    const HOST: &str = "0.0.0.0:8000";
    println!("Starting server on http://{}", HOST);
    let listener = tokio::net::TcpListener::bind(HOST).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
