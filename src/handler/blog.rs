use axum::{extract::Path, http::StatusCode, response::IntoResponse};

use crate::{
    dto::{CreateBlogReq, LoginReq, RegisterRequest, UpdateBlogReq},
    errors::AppError,
    models::{blog::BlogRepository, user::UserRepository},
    service::{blog::BlogService, user::UserService},
    state::AppState,
};

pub async fn create_blog(
    State(state): State<AppState>,
    Json(payload): Json<CreateBlogReq>,
) -> Result<impl IntoResponse, AppError> {
    println!("cerate blog: {:?}", payload);

    let blog_repo = BlogRepository::new(&state.pool);
    let b_service = BlogService::new(&blog_repo);

    let resp = b_service.create_blog(&payload).await?;
    println!("created blog: {:?}", resp.user);
    Ok(Json(resp))
}

pub async fn update_blog(
    State(state): State<AppState>,
    Json(payload): Json<UpdateBlogReq>,
) -> Result<impl IntoResponse, AppError> {
    println!("update blog: {:?}", payload);

    let blog_repo = BlogRepository::new(&state.pool);
    let b_service = BlogService::new(&blog_repo);

    let resp = b_service.update_blog(&payload).await?;
    Ok(Json(resp))
}

pub async fn delete_blog(
    State(state): State<AppState>,
    Path(blog_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    println!("delete blog: {:?}", blog_id);

    let blog_repo = BlogRepository::new(&state.pool);
    let b_service = BlogService::new(&blog_repo);

    let resp = b_service.delete_blog(blog_id).await?;
    Ok((StatusCode::OK).into_response())
}

pub async fn get_blog(
    State(state): State<AppState>,
    Path(blog_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    println!("get blog: {:?}", blog_id);

    let blog_repo = BlogRepository::new(&state.pool);
    let b_service = BlogService::new(&blog_repo);

    let resp = b_service.get_blog(blog_id).await?;
    Ok(Json(resp))
}

pub async fn list_blogs(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let blog_repo = BlogRepository::new(&state.pool);
    let b_service = BlogService::new(&blog_repo);

    let resp = b_service.list_blogs(0).await?;
    Ok(Json(resp))
}
