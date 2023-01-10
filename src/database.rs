use sea_orm::{Database, DbErr};


// connect database 
pub async fn database_init() -> Result<(), DbErr>{
    let database_uri = dotenvy::var("DATABASE_URL").unwrap();
    let _db = Database::connect(database_uri).await?;

    Ok(())
}

