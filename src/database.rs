use sea_orm::Database;


// connect database 
pub async fn database_init() {
    let database_uri = dotenvy::var("DATABASE_URL").unwrap();
    let db = Database::connect(database_uri).await;
}

