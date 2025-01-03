use anyhow::Result;
use axum::{routing::post, Router};
use backend::{signin, signup, AppConfig, AppState};
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{
    fmt::Layer, layer::SubscriberExt, registry, util::SubscriberInitExt, Layer as _,
};

const ADDR: &str = "127.0.0.1:";

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().pretty().with_filter(LevelFilter::INFO);
    registry().with(layer).init();

    let config = AppConfig::new()?;
    let state = AppState::new(config).await?;

    let addr = format!("{}{}", ADDR, state.config.server.port);

    info!("Server listening on {}", addr);
    let listener = TcpListener::bind(addr).await?;

    let app = Router::new()
        .route("/signup", post(signup))
        .route("/signin", post(signin))
        .with_state(state);

    axum::serve(listener, app).await?;

    Ok(())
}
