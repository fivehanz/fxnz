use sea_orm::{Database, DbErr};
// use migration::{Migrator, MigratorTrait};


// connect database 
pub async fn database_init() -> Result<(), DbErr>{
    let database_uri = dotenvy::var("DATABASE_URL").unwrap();
    let _db = Database::connect(database_uri).await?;

    // Migrator::up(&db, None).await?;

    Ok(())
}

