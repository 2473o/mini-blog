use axum::{
    Router, http,
    middleware::from_fn_with_state,
    response::IntoResponse,
    routing::{get, post},
};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    auth::verify_token,
    errors::AppError,
    handler::{
        blog::{create_blog, delete_blog, get_blog, list_blogs, update_blog},
        user::{get_user, login, register},
    },
    state::AppState,
};

pub async fn get_router(state: AppState) -> Result<Router, AppError> {
    let cors = CorsLayer::new()
        // Allow requests from any origin
        .allow_origin(Any)
        // Allow specific HTTP methods
        .allow_methods([
            http::Method::GET,
            http::Method::POST,
            http::Method::PUT,
            http::Method::DELETE,
            http::Method::OPTIONS,
        ])
        // Allow specific request headers
        .allow_headers([
            http::header::CONTENT_TYPE,
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
        ])
        // Set max age for browsers to cache CORS preflight requests
        .max_age(std::time::Duration::from_secs(3600));

    let api_router = Router::new()
        .route("/api/v1/users/{user_id}", get(get_user))
        .route("/api/v1/blogs", post(create_blog).get(list_blogs))
        .route(
            "/api/v1/blogs/{blog_id}",
            get(get_blog).put(update_blog).delete(delete_blog),
        )
        .layer(from_fn_with_state(state.clone(), verify_token::<AppState>))
        .route("/index", get(index))
        .route("/api/v1/users/register", post(register))
        .route("/api/v1/users/login", post(login))
        .layer(cors)
        .with_state(state);

    Ok(api_router)
}

async fn index() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}
