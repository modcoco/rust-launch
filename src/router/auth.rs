use std::sync::Arc;

use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    middleware::Next,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use tower_http::add_extension::AddExtensionLayer;

#[derive(Clone)]
struct AuthToken {
    token: String,
}

#[async_trait]
impl<B> FromRequest<B> for AuthToken
where
    B: Send,
{
    type Rejection = Response;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        // 获取 Authorization header 中的 Token
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(token) = auth_header.to_str() {
                // 验证 token（这里只是简单的示例，你可以根据实际需要进行验证）
                if token.starts_with("Bearer ") {
                    let token = token.trim_start_matches("Bearer ").to_string();
                    return Ok(AuthToken { token });
                }
            }
        }

        // 如果没有提供正确的 Token，返回 401 错误
        Err((axum::http::StatusCode::UNAUTHORIZED, "Unauthorized").into_response())
    }
}

// 需要在路由中使用该中间件
async fn protected_route() -> &'static str {
    "This is a protected route"
}

#[tokio::main]
async fn main() {
    // 设置路由，添加中间件
    let app = Router::new()
        .route("/protected", get(protected_route))
        .layer(AddExtensionLayer::new(()));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
