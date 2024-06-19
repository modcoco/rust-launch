// // lib1.rs
// pub fn router() -> axum::Router<axum::BoxRoute> {
//     axum::Router::new().route("/path_in_lib1", axum::handler::get(handler1).boxed())
// }

// // lib2.rs
// pub fn router() -> axum::Router<axum::BoxRoute> {
//     axum::Router::new().route("/path_in_lib2", axum::handler::get(handler2).boxed())
// }

// let app = Router::new()
//     .route("/path_in_app", get(handler).boxed())
//     .nest("/lib1", lib1::router()) // nested under /lib1
//     .nest("/lib2", lib2::router()) // nested under /lib2
//     .boxed();
