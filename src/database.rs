use sea_orm::{Database, DatabaseConnection};


// connect database 
pub async fn database_init() -> Result<DatabaseConnection, String> {
    // check for ENV var; return error otherwise
    match dotenvy::var("DATABASE_URL") {
        Ok(db_string) => {
            match Database::connect(db_string).await {
                Ok(dbc) => Ok(dbc),
                Err(e) => Err(format!("[ERORR]: {}", e))
            }
        },
        Err(e) => Err(format!("[ERORR]: {}", e))
    }    
}

