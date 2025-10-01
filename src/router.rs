use axum::{
    Router, http,
    response::IntoResponse,
    routing::{any, delete, get, post, put},
};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    errors::AppError,
    handlers::{
        channel_handler::{
            create_channel, get_channel, join_channel, leave_channel, list_channel_memebers,
            list_channels, list_user_channels,
        },
        message_handler::{get_message, list_messages, send_message_to_channel, update_message},
        user_handler::{get_user, login, register},
        websocket::message_loop,
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
        .route("/index", get(index))
        .route("/api/v1/users/register", post(register))
        .route("/api/v1/users/login", post(login))
        .route("/api/v1/users/{user_id}", get(get_user))
        .route("/api/v1/blogs", post(join_channel))
        .route("/api/v1/blogs/{blog_id}", get(leave_channel))
        .layer(cors)
        .with_state(state);

    Ok(api_router)
}

async fn index() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}
