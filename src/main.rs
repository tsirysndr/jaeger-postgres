use std::{net::SocketAddr, time::Duration};

use anyhow::Context;
use clap::Parser;
use jaeger_postgres::{
    config::Config,
    proto::storage::{
        dependency_reader_server::DependencyReaderServer, trace_reader_server::TraceReaderServer,
    },
    server::StorageServer,
    store::Store,
};
use opentelemetry_proto::tonic::collector::trace::v1::trace_service_server::TraceServiceServer;
use sqlx::postgres::PgPoolOptions;
use tonic::transport::Server;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| "jaeger_postgres=info".into()),
        )
        .init();
    let config = Config::parse();
    let address: SocketAddr = config.listen_addr.parse().context("parse LISTEN_ADDR")?;
    let pool = PgPoolOptions::new()
        .max_connections(config.database_max_connections)
        .acquire_timeout(Duration::from_secs(10))
        .connect(&config.database_url)
        .await
        .context("connect to PostgreSQL from DATABASE_URL")?;
    let store = Store::new(pool, config.max_search_depth);
    store.migrate().await?;
    let service = StorageServer::new(store);

    tracing::info!(%address, "Jaeger PostgreSQL storage listening");
    Server::builder()
        .add_service(TraceServiceServer::new(service.clone()))
        .add_service(TraceReaderServer::new(service.clone()))
        .add_service(DependencyReaderServer::new(service))
        .serve_with_shutdown(address, shutdown())
        .await?;
    Ok(())
}

async fn shutdown() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("install Ctrl-C handler")
    };
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("install SIGTERM handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! { _ = ctrl_c => {}, _ = terminate => {} }
}
