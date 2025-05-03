use std::env;

use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, query_as, PgPool};
use tracing::info;
use tracing_subscriber::EnvFilter;

#[derive(Debug)]
struct Item {
    category: Option<String>,
    description: Option<String>,
    rating: Option<i32>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("loading environment variables");
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    info!("connecting to paradedb");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await?;

    info!("running full text search");
    search_full_text(&pool, 5).await?;

    Ok(())
}

async fn search_full_text(pool: &PgPool, limit: i64) -> anyhow::Result<()> {
    let recs = query_as!(
        Item,
        r#"
        SELECT description, rating, category
        FROM mock_items
        WHERE description @@@ 'shoes' OR category @@@ 'footwear' AND rating @@@ '>2'
        ORDER BY description
        LIMIT $1;
        "#,
        limit,
    )
    .fetch_all(pool)
    .await?;

    for rec in recs {
        println!(
            "- [{}] {}: {}",
            rec.category.as_deref().unwrap_or("No Category"),
            rec.description.as_deref().unwrap_or("No Description"),
            rec.rating.unwrap_or(0),
        );
    }

    Ok(())
}
