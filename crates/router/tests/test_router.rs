
// // lib2.rs
// use axum::{handler::get, Router};

// pub async fn handler2() -> &'static str {
//     "Handler2 from lib2"
// }

// pub fn get_lib2_routes() -> Router {
//     Router::new().route("/handler2", get(handler2))
// }

// // lib1.rs
// use axum::{handler::get, Router};

// pub async fn handler1() -> &'static str {
//     "Handler1 from lib1"
// }

// pub fn get_lib1_routes() -> Router {
//     Router::new().route("/handler1", get(handler1))
// }

// #[tokio::main]
// async fn main() {
//     // Other required imports
//     use common::tokio;
//     use std::net::SocketAddr;

//     let routes = lib1::get_lib1_routes().or(lib2::get_lib2_routes());

//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//     Server::bind(&addr)
//         .serve(routes.into_make_service())
//         .await
//         .unwrap();
// }
