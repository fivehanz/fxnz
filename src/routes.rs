mod index;
use axum::{routing::get, Router, body::Body};
use index::{home, slug_handler};


// all routes
pub fn routes() -> Router<(), Body> {
    Router::new()
        .route("/", get(home))
        .route("/:slug", get(slug_handler))
}