use axum::{routing::get, Extension, Router};
use mixmag_axum::config::database::{establish_connection_pool_with_config, DatabaseConfig};
use mixmag_axum::routes;
use std::time::Duration;
use tokio::runtime::Builder;
use tokio::sync::oneshot;
use tracing_subscriber;

async fn api_handler() -> &'static str {
    "This is the API endpoint"
}

async fn run(close_rx: oneshot::Receiver<()>) {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_config = DatabaseConfig::new()
        .with_max_connections(5)
        .with_min_connections(1);
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
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running at http://0.0.0.0:3000");

    // Start the server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            _ = close_rx.await;
        })
        .await
        .unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a custom Tokio runtime with adjusted settings
    let runtime = Builder::new_multi_thread() // Use a multi-threaded runtime
        .worker_threads(2) // Reduce the number of worker threads
        .thread_stack_size(2 * 1024 * 1024) // Increase the stack size for each thread (2 MB)
        .enable_all()
        .build()?;

    // Create a channel for graceful shutdown
    let (close_tx, close_rx) = oneshot::channel();

    // Run the server in a new thread with a larger stack size
    std::thread::Builder::new()
        .stack_size(8 * 1024 * 1024) // Set the stack size to 8 MB
        .spawn(move || {
            runtime.block_on(run(close_rx));
        })
        .unwrap()
        .join()
        .unwrap();

    Ok(())
}
