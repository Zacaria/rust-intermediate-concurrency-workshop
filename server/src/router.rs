use axum::{
    routing::{get, post},
    Extension,
};
use domain::blog_post::BlogPost;
use sqlx::SqlitePool;

use crate::{
    blog_client::{
        add_blog_post, delete_blog_post, get_blog_post, get_blog_posts, update_blog_post,
    },
    // save_domain::BlogPost,
};

pub fn get_router(pool: SqlitePool) -> axum::Router {
    axum::Router::new()
        .route("/hello", get(say_hello))
        .route("/", get(get_blog_posts_handler))
        .route("/:id", get(get_blog_post_handler))
        .route("/add", post(add_blog_post_handler))
        .route("/update/:id", post(update_blog_post_handler))
        .route("/delete/:id", post(delete_blog_post_handler))
        .layer(Extension(pool.clone()))
}

async fn say_hello() -> &'static str {
    "Hello, World!"
}

async fn get_blog_posts_handler(
    Extension(pool): Extension<sqlx::SqlitePool>,
) -> axum::Json<Vec<BlogPost>> {
    let posts = get_blog_posts(pool).await.unwrap();
    axum::Json(posts)
}

async fn get_blog_post_handler(
    Extension(pool): Extension<sqlx::SqlitePool>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> axum::Json<BlogPost> {
    let post = get_blog_post(pool, id).await.unwrap();
    axum::Json(post)
}

async fn add_blog_post_handler(
    Extension(pool): Extension<sqlx::SqlitePool>,
    axum::extract::Json(post): axum::extract::Json<BlogPost>,
) -> axum::Json<i32> {
    let id = add_blog_post(pool, post.date, post.title, post.body, post.author)
        .await
        .unwrap();
    axum::Json(id)
}

async fn update_blog_post_handler(
    Extension(pool): Extension<sqlx::SqlitePool>,
    axum::extract::Path(id): axum::extract::Path<i32>,
    axum::extract::Json(post): axum::extract::Json<BlogPost>,
) -> axum::Json<()> {
    update_blog_post(pool, id, post.date, post.title, post.body, post.author)
        .await
        .unwrap();
    axum::Json(())
}

async fn delete_blog_post_handler(
    Extension(pool): Extension<sqlx::SqlitePool>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> axum::Json<()> {
    delete_blog_post(pool, id).await.unwrap();
    axum::Json(())
}
