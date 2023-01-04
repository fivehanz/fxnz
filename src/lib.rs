mod routes;
use routes::routes;


pub async fn app() {
    // initialize the routes
    let app = routes();

    // run the server
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}