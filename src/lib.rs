mod routes;
mod database;

use routes::routes;
use axum::Server;
use database::database_init;

// main server 
pub async fn app() {
    // initialize the routes
    let app = routes();

    // run the server
    Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    // run the db connection
    database_init().await;
}


