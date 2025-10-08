use dotenv::dotenv;
use mini_blog::{router::get_router, state::AppState};
use sqlx::PgPool;
use std::env;
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{Layer as _, fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    info!("Starting mini-blog server...");

    // Load variables from .env file
    dotenv().ok();

    // Using expect when the variable is required
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    info!("database_url: {}", database_url);

    let pool = PgPool::connect(&database_url).await?;
    info!("connected database: {:?}", pool);

    let state = AppState::new(pool, "", "")?;
    let router = get_router(state).await?;

    let addr = format!("0.0.0.0:{}", "6869");
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on: {}", addr);

    axum::serve(listener, router.into_make_service()).await?;
    Ok(())
}
