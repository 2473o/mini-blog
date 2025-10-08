use crate::{
    dto::{BlogResp, CreateBlogReq, UpdateBlogReq},
    errors::AppError,
    models::blog::{BlogRepository, CreateBlog, UpdateBlog},
};

pub struct BlogService<'a> {
    blog_store: &'a BlogRepository<'a>,
}

impl<'a> BlogService<'a> {
    pub fn new(blog_store: &'a BlogRepository<'a>) -> Self {
        Self { blog_store }
    }

    pub async fn create_blog(
        &self,
        author_id: i64,
        req: &CreateBlogReq,
    ) -> Result<BlogResp, AppError> {
        let blog = self
            .blog_store
            .create(&CreateBlog {
                title: req.title.clone(),
                content: req.content.clone(),
                author_id,
            })
            .await?;

        Ok(BlogResp { blog })
    }

    pub async fn update_blog(
        &self,
        author_id: i64,
        req: &UpdateBlogReq,
    ) -> Result<BlogResp, AppError> {
        if req.id == 0 {
            return Err(AppError::InvalidArgument("blog id is invalid".to_string()));
        }

        let blog_opt = self
            .blog_store
            .update(
                author_id,
                &UpdateBlog {
                    id: req.id,
                    title: req.title.clone(),
                    content: req.content.clone(),
                },
            )
            .await?;

        match blog_opt {
            Some(blog) => Ok(BlogResp { blog }),
            None => Err(AppError::NotFound("blog not found".to_string())),
        }
    }

    pub async fn delete_blog(&self, user_id: i64, blog_id: i64) -> Result<(), AppError> {
        if blog_id == 0 {
            return Err(AppError::InvalidArgument("blog id is invalid".to_string()));
        }

        let blog = self
            .blog_store
            .get_by_id(blog_id)
            .await?
            .ok_or_else(|| AppError::NotFound("blog not found".to_string()))?;

        if blog.author_id != user_id {
            return Err(AppError::PermissionDenied(
                "cannot delete other user's blog".to_string(),
            ));
        }

        self.blog_store.delete_by_id(blog_id).await
    }

    pub async fn get_blog(&self, blog_id: i64) -> Result<BlogResp, AppError> {
        if blog_id == 0 {
            return Err(AppError::InvalidArgument("blog id is invalid".to_string()));
        }

        let blog_opt = self.blog_store.get_by_id(blog_id).await?;
        match blog_opt {
            Some(blog) => Ok(BlogResp { blog }),
            None => Err(AppError::NotFound("blog not found".to_string())),
        }
    }

    pub async fn list_blogs(&self, author_id: i64) -> Result<Vec<BlogResp>, AppError> {
        let blogs = self.blog_store.get_blogs_by_author_id(author_id).await?;
        let resp = blogs.into_iter().map(|blog| BlogResp { blog }).collect();
        Ok(resp)
    }
}
