mod handlers;
mod models;
mod router;

use axum::serve;
use router::create_app;

#[tokio::main]
async fn main() {
    let app = create_app();

    // Bind the TCP listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let addr = listener.local_addr().unwrap();
    println!("Server running on http://{}", addr);

    // Serve the app using axum::serve
    serve(listener, app).await.unwrap();
}
