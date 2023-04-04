use hanzlol::app;


#[tokio::main]
async fn main() {

    // run the app server
    app().await.unwrap_or_else(|e| println!("{}", e))
}