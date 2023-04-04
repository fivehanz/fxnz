use axum::{extract::Path, response::Redirect};
use crate::{database::database_init, utils::url::get_url_from_db};


// slug handler
pub async fn slug_handler(Path(slug): Path<String>) -> Redirect {  
    // error handling with match
    match database_init().await {

        // redirect to the relevant URL
        Ok(db) => {
            match get_url_from_db(slug, &db).await {
                Ok(url) => Redirect::temporary(url.as_str()),
                Err(e) => {
                    println!("{}", e);
                    Redirect::temporary("/") 
                }
            }
        },
        
        // redirect to home
        Err(e) => {
            println!("{} -- {}", e, slug);
            Redirect::temporary("/")
        }
    }
    // TODO: add logging, metrics
}

// TODO: Implement CRUD for Users, Links

// default home route handler
pub async fn home() -> String {
    String::from("Hey, if you can see this -- routes, databases, error handling are working #6 -- Hanz.")
}
