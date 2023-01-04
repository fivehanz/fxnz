mod routes;
use routes::routes;

pub async fn app() {
    let app = routes();

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}