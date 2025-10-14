use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use tracing::info;

use crate::{
    dto::{CreateBlogReq, ListBlogResp, SimpleBlog, UpdateBlogReq, User},
    errors::AppError,
    models::blog::BlogRepository,
    service::blog::BlogService,
    state::AppState,
};

pub async fn index(State(_): State<AppState>) -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}

pub async fn create_blog(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(payload): Json<CreateBlogReq>,
) -> Result<impl IntoResponse, AppError> {
    println!("cerate blog: {:?}", payload);

    let blog_repo = BlogRepository::new(&state.pool);
    let b_service = BlogService::new(&blog_repo);

    let resp = b_service.create_blog(user.id, &payload).await?;
    println!("created blog: {:?}", resp.blog);
    Ok(Json(resp))
}

pub async fn update_blog(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateBlogReq>,
) -> Result<impl IntoResponse, AppError> {
    println!("update blog: {:?}", payload);

    let blog_repo = BlogRepository::new(&state.pool);
    let b_service = BlogService::new(&blog_repo);

    let resp = b_service.update_blog(user.id, &payload).await?;
    Ok(Json(resp))
}

pub async fn delete_blog(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path(blog_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    println!("delete blog: {:?}", blog_id);

    let blog_repo = BlogRepository::new(&state.pool);
    let b_service = BlogService::new(&blog_repo);

    let _ = b_service.delete_blog(user.id, blog_id).await?;
    Ok((StatusCode::OK).into_response())
}

pub async fn get_blog(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path(blog_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    println!("get blog: {:?}", blog_id);

    let blog_repo = BlogRepository::new(&state.pool);
    let b_service = BlogService::new(&blog_repo);

    let resp = b_service.get_blog(blog_id).await?;
    if user.id != resp.blog.author_id {
        return Err(AppError::PermissionDenied(
            "cannot access other user's blog".to_string(),
        ));
    }

    Ok(Json(resp))
}

pub async fn list_blogs(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    info!("user: {} is listing blogs......", user.id);

    let blog_repo = BlogRepository::new(&state.pool);
    let b_service = BlogService::new(&blog_repo);

    let resp = b_service.list_blogs(user.id).await?;
    let simple_blogs = resp
        .into_iter()
        .map(|b| SimpleBlog::from(b.blog))
        .collect::<Vec<_>>();
    Ok(Json(ListBlogResp {
        blogs: simple_blogs,
    }))
}
