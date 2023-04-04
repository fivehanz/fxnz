use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, DatabaseConnection};
use crate::model::link;
use crate::model::link::Entity as Link;


// gets url string from the database
// returns Result<String, String>
pub async fn get_url_from_db(slug: String, db: &DatabaseConnection) -> Result<String, String> {
    // try getting from db and match
    match Link::find().filter(
        link::Column::Slug.contains(&slug)
    ).one(db)
    .await {
        // if get model
        Ok(model) => {
            match model {
                // return if URL string found
                Some(model) => Ok(model.url),
                None => Err(format!("[ERORR]: not found in database -- {}", slug))
            }
        },
        // else db model error
        Err(e) => Err(format!("[ERORR]: {}", e))
    }
}

