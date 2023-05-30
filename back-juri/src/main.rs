mod database;
mod utils;
mod user;

use juri::Router;
use std::net::SocketAddr;

#[juri::main]
async fn main() {
    let mut router = Router::new();
    router.root("/api");
    
    router.router(user::router());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8520));
    juri::Server::bind(addr).server(router).await.unwrap();
}
