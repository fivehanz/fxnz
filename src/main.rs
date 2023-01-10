use hanzlol::app;


#[tokio::main]
async fn main() {

    // run the app server
    app().await.expect("Database Connection Failed.")
}