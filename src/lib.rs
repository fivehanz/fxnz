mod routes;
mod database;
mod model;
mod utils;

use std::error::Error;
use routes::routes;
use axum::Server;

// main server runner
// TODO: handle server errors
pub async fn app() -> Result<(), Box<dyn Error>> { 
    // initialize the routes
    let app = routes();

    // run the server
    Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    // everything's OK.
    Ok(())
}


