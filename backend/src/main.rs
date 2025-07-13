use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use rust_react_starter::{create_app, database::create_database_pool};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_react_starter=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize database
    let pool = create_database_pool().await?;
    tracing::info!("Database initialized successfully");

    let app = create_app(pool);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3001".to_string());
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    
    tracing::info!("Server running on http://127.0.0.1:{}", port);
    axum::serve(listener, app).await?;

    Ok(())
}