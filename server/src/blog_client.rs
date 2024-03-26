use anyhow::Result;
use domain::blog_post::BlogPost;
use sqlx::Row;

pub async fn get_blog_posts(pool: sqlx::SqlitePool) -> Result<Vec<BlogPost>> {
    let posts = sqlx::query_as::<_, BlogPost>("SELECT * FROM blog_posts")
        .fetch_all(&pool)
        .await?;
    Ok(posts)
}

pub async fn get_blog_post(pool: sqlx::SqlitePool, id: i32) -> Result<BlogPost> {
    let post = sqlx::query_as::<_, BlogPost>("SELECT * FROM blog_posts WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;
    Ok(post)
}

pub async fn add_blog_post(
    pool: sqlx::SqlitePool,
    date: String,
    title: String,
    body: String,
    author: String,
) -> Result<i32> {
    let id = sqlx::query("INSERT INTO blog_posts (date, title, body, author) VALUES (?, ?, ?, ?); SELECT last_insert_rowid();")
        .bind(date)
        .bind(title)
        .bind(body)
        .bind(author)
        .fetch_one(&pool)
        .await?
        .get(0);
    Ok(id)
}

pub async fn update_blog_post(
    pool: sqlx::SqlitePool,
    id: i32,
    date: String,
    title: String,
    body: String,
    author: String,
) -> Result<()> {
    sqlx::query("UPDATE blog_posts SET date = ?, title = ?, body = ?, author = ? WHERE id = ?")
        .bind(date)
        .bind(title)
        .bind(body)
        .bind(author)
        .bind(id)
        .execute(&pool)
        .await?;
    Ok(())
}

pub async fn delete_blog_post(pool: sqlx::SqlitePool, id: i32) -> Result<()> {
    sqlx::query("DELETE FROM blog_posts WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;
    Ok(())
}
