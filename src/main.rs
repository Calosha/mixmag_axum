use axum::{routing::get, Extension, Router};
use mixmag_axum::config::database::{establish_connection_pool_with_config, DatabaseConfig};
use mixmag_axum::routes;
use std::mem;
use tokio::net::TcpListener;
use tracing_subscriber;

async fn api_handler() -> &'static str {
    "This is the API endpoint"
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_config = DatabaseConfig::new()
        .with_max_connections(10)
        .with_min_connections(5);
    let pool = establish_connection_pool_with_config(db_config);

    // Define other routes
    let api_routes = Router::new()
        .route("/", get(api_handler))
        .nest("/health", routes::health::router())
        .nest("/articles", routes::articles::router())
        .layer(Extension(pool));

    // Define the route for `/api`
    let app = Router::new().nest("/api", api_routes);

    // Bind to the address (3000)
    // Start the server
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running at http://0.0.0.0:3000");
    // Add this debug code temporarily:
    println!("Size of u32: {} bytes", mem::size_of::<u32>());
    println!("Size of i64: {} bytes", mem::size_of::<i64>());
    println!("Size of usize: {} bytes", mem::size_of::<usize>());
    axum::serve(listener, app).await.unwrap();
}
