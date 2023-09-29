use axum::Server;
use nft_api::api::routes::routes;


#[tokio::main]
async fn main() {
    let app = routes();

    let server = Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service());

    if let Err(err) = server.await {
        eprintln!("Server error: {:?}", err);
    }
}
