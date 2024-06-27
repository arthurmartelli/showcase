use std::u16;

use learning_axum::App;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let port = std::env::var("PORT")
        .expect("Missing env PORT")
        .parse::<u16>()
        .expect("Port must be a valid number");

    let app = App::new(port);
    dbg!(&app);

    app.run().await.expect("Error starting the server");
}
