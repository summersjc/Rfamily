mod api;
mod state;

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use state::AppState;

#[derive(OpenApi)]
#[openapi(
    paths(
        api::presets::list_presets,
        api::presets::get_preset,
        api::presets::example_ruleset,
        api::validate::validate_ruleset,
        api::generate::preview,
        api::generate::generate,
    ),
    components(
        schemas(
            rfamily_common::api::PresetInfo,
            rfamily_common::api::PresetsResponse,
            rfamily_common::api::PresetDetailResponse,
            rfamily_common::api::ValidateRequest,
            rfamily_common::api::ValidationResponse,
            rfamily_common::api::PreviewRequest,
            rfamily_common::api::PreviewResponse,
            rfamily_common::api::GenerateRequest,
            rfamily_common::api::GenerationStatistics,
            rfamily_common::api::ErrorResponse,
        )
    ),
    tags(
        (name = "Presets", description = "Preset management endpoints"),
        (name = "Validation", description = "Ruleset validation endpoints"),
        (name = "Generation", description = "GEDCOM generation endpoints")
    ),
    info(
        title = "Rfamily API",
        version = "0.1.0",
        description = "REST API for generating GEDCOM files with customizable rulesets. \
                       Supports 51 language presets and custom ruleset configurations.",
        contact(
            name = "Rfamily Project",
            url = "https://github.com/summersjc/Rfamily"
        )
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,rfamily_web=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize application state
    let state = AppState::new();

    // Build API router
    let api_router = Router::new()
        .route("/presets", get(api::presets::list_presets))
        .route("/presets/:name", get(api::presets::get_preset))
        .route("/validate", post(api::validate::validate_ruleset))
        .route("/preview", post(api::generate::preview))
        .route("/generate", post(api::generate::generate))
        .route("/example", get(api::presets::example_ruleset))
        .with_state(state.clone());

    // Build main router
    let app = Router::new()
        .merge(SwaggerUi::new("/api/docs").url("/api/openapi.json", ApiDoc::openapi()))
        .nest("/api", api_router)
        .nest_service("/", ServeDir::new("rfamily-web/static"))
        .layer(CorsLayer::permissive()) // TODO: Configure CORS appropriately for production
        .layer(TraceLayer::new_for_http());

    // Start server
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Starting Rfamily web server on {}", addr);
    tracing::info!("API available at http://localhost:{}/api", port);
    tracing::info!(
        "API Documentation available at http://localhost:{}/api/docs",
        port
    );

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
