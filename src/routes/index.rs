use axum::extract::Path;
// use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
// use crate::database::database_init;
// use crate::model::link;
// use crate::model::link::Entity as Link;

// slug handler
pub async fn slug_handler(Path(slug): Path<String>) -> String {    
    // let db = database_init().await.unwrap();

    // let url = Link::find()
    //     .filter(link::Column::Slug.contains(&slug))
    //     .one(&db)
    //     .await
    //     .unwrap()
    //     .unwrap().url;
    
    // url

    slug
}

// default home route handler
pub async fn home() -> String {
    String::from("Hey, if you can see this -- routes are working #4 -- Rust.")
}
