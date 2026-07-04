// IKinder

use futures::{StreamExt, TryStreamExt};
use sqlx::{FromRow, Pool, Row, Sqlite};

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    crate::show_name(file!());

    dotenv::dotenv()?;

    let db_url = std::env::var("DATABASE_URL")?;
    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    //Run Migrations
    // sqlx::migrate!("./migrations")
    // 	.run(&pool)
    // 	.await?;

    // rows(&pool).await?;
    // from_rows(&pool).await?;
    stream(&pool).await?;
    println!();
    update_message(4, "Updated message", &pool).await?;
    stream(&pool).await?;

    Ok(())
}

async fn rows(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    let messages = sqlx::query("SELECT id, message FROM messages")
        .map(|row: sqlx::sqlite::SqliteRow| {
            let id: i64 = row.get(0);
            let message: String = row.get(1);
            (id, message)
        })
        .fetch_all(pool)
        .await?;

    for (id, message) in messages {
        println!("{} : {}", id, message);
    }

    Ok(())
}

#[derive(Debug, FromRow)]
struct Message {
    id: i64,
    message: String,
}

async fn from_rows(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    let messages = sqlx::query_as::<_, Message>("SELECT id, message FROM messages")
        .fetch_all(pool)
        .await?;

    println!("{:#?}", messages);

    Ok(())
}

async fn stream(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    let mut message_stream =
        sqlx::query_as::<_, Message>("SELECT id, message FROM messages").fetch(pool);
    while let Some(message) = message_stream.try_next().await? {
        println!("{} : {}", message.id, message.message);
    }

    Ok(())
}

async fn update_message(id: i64, message: &str, pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    sqlx::query("UPDATE messages SET message = ? where id = ?")
        .bind(message)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
