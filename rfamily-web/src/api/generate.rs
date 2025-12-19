use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use rfamily_common::api::{
    ErrorResponse, GenerateRequest, GenerationStatistics, PreviewRequest, PreviewResponse,
};
use rfamily_core::{
    generator::{GedcomGenerator, Sex},
    ruleset::Ruleset,
};
use std::io::{BufWriter, Cursor};

use crate::state::AppState;

const MAX_PREVIEW_COUNT: usize = 100;
const MAX_GENERATE_COUNT: usize = 10_000_000;

#[utoipa::path(
    post,
    path = "/api/preview",
    request_body = PreviewRequest,
    responses(
        (status = 200, description = "Preview generated successfully with GEDCOM content and statistics", body = PreviewResponse),
        (status = 400, description = "Invalid request (count out of range, both preset and ruleset specified)", body = ErrorResponse),
        (status = 404, description = "Preset not found", body = ErrorResponse),
        (status = 500, description = "Generation failed", body = ErrorResponse),
    ),
    tag = "Generation"
)]
pub async fn preview(
    State(state): State<AppState>,
    Json(req): Json<PreviewRequest>,
) -> Result<Json<PreviewResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Validate count
    if req.count > MAX_PREVIEW_COUNT {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("Preview count must be <= {}", MAX_PREVIEW_COUNT),
                details: None,
            }),
        ));
    }

    // Load ruleset
    let ruleset = resolve_ruleset(&state, req.preset_name, req.ruleset)?;

    // Generate in spawn_blocking (CPU-bound work)
    let count = req.count;
    let result = tokio::task::spawn_blocking(move || {
        let start = std::time::Instant::now();
        let mut rng = rand::thread_rng();
        let mut generator = GedcomGenerator::new(ruleset);

        generator.generate(count, &mut rng);

        // Write to memory buffer
        let mut buffer = Vec::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut writer = BufWriter::new(cursor);
            generator
                .write_gedcom(&mut writer)
                .map_err(|e| e.to_string())?;
        }

        let gedcom = String::from_utf8(buffer).map_err(|e| e.to_string())?;
        let duration = start.elapsed();

        // Calculate statistics
        let stats = calculate_statistics(&generator, duration);

        Ok::<_, String>((gedcom, stats))
    })
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Generation task failed: {}", e),
                details: None,
            }),
        )
    })?
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Generation failed: {}", e),
                details: None,
            }),
        )
    })?;

    Ok(Json(PreviewResponse {
        gedcom: result.0,
        statistics: result.1,
    }))
}

#[utoipa::path(
    post,
    path = "/api/generate",
    request_body = GenerateRequest,
    responses(
        (status = 200, description = "GEDCOM file generated successfully",
            content_type = "application/x-gedcom",
            headers(
                ("Content-Disposition" = String, description = "Attachment filename")
            )
        ),
        (status = 400, description = "Invalid request (count out of range, both preset and ruleset specified)", body = ErrorResponse),
        (status = 404, description = "Preset not found", body = ErrorResponse),
        (status = 413, description = "Count exceeds maximum limit", body = ErrorResponse),
        (status = 500, description = "Generation failed", body = ErrorResponse),
    ),
    tag = "Generation"
)]
pub async fn generate(
    State(state): State<AppState>,
    Json(req): Json<GenerateRequest>,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    // Validate count
    if req.count > MAX_GENERATE_COUNT {
        return Err((
            StatusCode::PAYLOAD_TOO_LARGE,
            Json(ErrorResponse {
                error: format!("Count exceeds maximum limit of {}", MAX_GENERATE_COUNT),
                details: None,
            }),
        ));
    }

    if req.count == 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Count must be greater than 0".to_string(),
                details: None,
            }),
        ));
    }

    // Load ruleset
    let ruleset = resolve_ruleset(&state, req.preset_name.clone(), req.ruleset)?;

    // Generate file
    let count = req.count;
    let preset_name = req.preset_name.unwrap_or_else(|| "custom".to_string());

    let gedcom_data = tokio::task::spawn_blocking(move || {
        let mut rng = rand::thread_rng();
        let mut generator = GedcomGenerator::new(ruleset);

        generator.generate(count, &mut rng);

        // Write to memory buffer
        let mut buffer = Vec::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut writer = BufWriter::new(cursor);
            generator
                .write_gedcom(&mut writer)
                .map_err(|e| e.to_string())?;
        }

        Ok::<_, String>(buffer)
    })
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Generation task failed: {}", e),
                details: None,
            }),
        )
    })?
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Generation failed: {}", e),
                details: None,
            }),
        )
    })?;

    // Return as downloadable file
    let filename = format!("family-{}-{}.ged", count, preset_name);

    Ok((
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/x-gedcom"),
            (
                header::CONTENT_DISPOSITION,
                &format!("attachment; filename=\"{}\"", filename),
            ),
        ],
        gedcom_data,
    )
        .into_response())
}

fn resolve_ruleset(
    state: &AppState,
    preset_name: Option<String>,
    custom_ruleset: Option<Ruleset>,
) -> Result<Ruleset, (StatusCode, Json<ErrorResponse>)> {
    match (preset_name, custom_ruleset) {
        (Some(_), Some(_)) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Cannot specify both preset_name and ruleset".to_string(),
                details: None,
            }),
        )),
        (Some(name), None) => state.preset_registry().load(&name).map_err(|e| {
            (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: e,
                    details: None,
                }),
            )
        }),
        (None, Some(ruleset)) => Ok(ruleset),
        (None, None) => {
            // Default to English
            state.preset_registry().load("english").map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: e,
                        details: None,
                    }),
                )
            })
        }
    }
}

fn calculate_statistics(
    generator: &GedcomGenerator,
    duration: std::time::Duration,
) -> GenerationStatistics {
    let individuals = generator.individuals();
    let families = generator.families();

    let males = individuals
        .values()
        .filter(|i| matches!(i.sex, Sex::Male))
        .count();
    let females = individuals.len() - males;

    GenerationStatistics {
        total_individuals: individuals.len(),
        males,
        females,
        families: families.len(),
        generation_time_ms: duration.as_millis() as u64,
    }
}
