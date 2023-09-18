use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::IntoResponse,
    Json,
};
use tower::ServiceExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            "backend=debug,tower_http=debug",
        )
    }

    tracing_subscriber::fmt::init();

    let app = axum::Router::new()
        .route(common::url::CONFIG, axum::routing::get(get_config))
        .route("/*O", axum::routing::get(get_file))
        .route("/", axum::routing::get(get_file))
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                .make_span_with(
                    tower_http::trace::DefaultMakeSpan::new().level(tracing::Level::INFO),
                )
                .on_response(
                    tower_http::trace::DefaultOnResponse::new().level(tracing::Level::INFO),
                ),
        );

    let addr = std::net::SocketAddrV4::new(std::net::Ipv4Addr::LOCALHOST, 9999).into();
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

/// Required to be able to shutdown the application by CTRL+C signal.
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

async fn get_config() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(common::config::Config {
            header: "Hello from backend!".to_string(),
        }),
    )
}

async fn get_file(request: Request<Body>) -> Result<impl IntoResponse, impl IntoResponse> {
    let matcher = regex::Regex::new(r#"^\/.+\..+$"#).unwrap();
    let dist_path = std::env::var("BACKEND_DIST_PATH").expect("BACKEND_DIST_PATH is not set.");

    if matcher.is_match(request.uri().path()) {
        match tower_http::services::ServeDir::new(dist_path)
            .oneshot(request)
            .await
        {
            Ok(file) => Ok(file),
            Err(_) => Err(StatusCode::NOT_FOUND),
        }
    } else {
        match tower_http::services::ServeFile::new(format!("{dist_path}/index.html"))
            .oneshot(request)
            .await
        {
            Ok(file) => Ok(file),
            Err(_) => Err(StatusCode::NOT_FOUND),
        }
    }
}
