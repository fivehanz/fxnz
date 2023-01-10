mod routes;
mod database;

use routes::routes;
use axum::Server;
use database::database_init;
use sea_orm::DbErr;


// main server runner
// TODO: Change DbErr to include Server Errors
pub async fn app() -> Result<(), DbErr> { 
    // initialize the routes
    let app = routes();

    // run the server
    Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    // run the db connection
    database_init().await?;

    // everything's OK.
    Ok(())
}


