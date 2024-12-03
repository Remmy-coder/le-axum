use axum::{
    routing::{get, get_service, post},
    Router,
};
use tower_http::{limit::RequestBodyLimitLayer, services::ServeDir, trace::TraceLayer};

use crate::handlers::{about, home, projects, run_code};

pub fn create_app() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/about", get(about))
        .route("/projects", get(projects))
        .route("/run-code", post(run_code))
        .nest_service("/static", get_service(ServeDir::new("static")))
        .layer(TraceLayer::new_for_http())
        .layer(RequestBodyLimitLayer::new(1024 * 64))
}
