mod routes;
mod database;
mod model;

use routes::routes;
use axum::Server;
use database::database_init;
use sea_orm::DbErr;


// main server runner
// TODO: Change DbErr to include Server Errors
pub async fn app() -> Result<(), DbErr> { 
    // run the db connection and return database connection
    // ! is it required here?
    let _db = database_init().await?;

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


