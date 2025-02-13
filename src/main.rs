use axum::{Router, routing::get, Extension};
use tokio::net::TcpListener;

mod routes;
mod config;
mod services;

async fn api_handler() -> &'static str {
    "This is the API endpoint"
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let pool = config::database::establish_connection_pool();


    // Define other routes
    let api_routes = Router::new()
        .route("/", get(api_handler))
        .nest("/health", routes::health::router())
         .layer(Extension(pool));

    // Define the route for `/api`
    let app = Router::new().nest("/api", api_routes);

    // Bind to the address (3000)
    // Start the server
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running at http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();

}

