use axum::extract::Path;


// slug handler
pub async fn slug_handler(Path(slug): Path<String>) -> String {
    slug
}

// default home route handler
pub async fn home() -> String {
    String::from("Hey, if you can see this -- routes are working #3 -- Rust.")
}
