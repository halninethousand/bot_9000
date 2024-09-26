use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use std::env::var;
use core::future::Future;
use core::pin::Pin;

// massive clusterfuck return type 
pub fn insert_and_fetch() -> Pin<Box<dyn Future<Output = Result<(), sqlx::Error>> + Send>> {
    Box::pin(async move {
        dotenv().ok();
        let database_url = var("DATABASE_URL")
            .expect("DATABASE_URL load error");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        let new_quote = "Michael Singer: Jai Guru Dev. Jai Masters.";
        sqlx::query!(
            r#"
            INSERT INTO quotes (quote) VALUES ($1)
            "#,
            new_quote
        )
        .execute(&pool)
        .await?;

        let rows = sqlx::query!(
            r#"
            SELECT quote FROM quotes;
            "#
        )
        .fetch_all(&pool)
        .await?;

        for row in rows {
            println!("Quote: {}", row.quote);
        }

        Ok(())
    })
}
