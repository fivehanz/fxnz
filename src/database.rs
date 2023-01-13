use sea_orm::{Database, DbErr, DatabaseConnection};


// connect database 
pub async fn database_init() -> Result<DatabaseConnection, DbErr>{
    Database::connect(dotenvy::var("DATABASE_URL").unwrap()).await
}

