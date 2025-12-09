use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use rfamily_common::api::{ErrorResponse, PresetDetailResponse, PresetInfo, PresetsResponse};

#[utoipa::path(
    get,
    path = "/api/presets",
    responses(
        (status = 200, description = "List of all available presets", body = PresetsResponse),
    ),
    tag = "Presets"
)]
pub async fn list_presets(
    State(state): State<AppState>,
) -> Result<Json<PresetsResponse>, (StatusCode, Json<ErrorResponse>)> {
    let registry = state.preset_registry();
    let preset_names = registry.list();

    let presets = preset_names
        .iter()
        .map(|name| PresetInfo {
            name: name.clone(),
            display_name: format_display_name(name),
            description: format!("{} names and locations", format_display_name(name)),
            region: get_region(name),
        })
        .collect();

    Ok(Json(PresetsResponse { presets }))
}

#[utoipa::path(
    get,
    path = "/api/presets/{name}",
    params(
        ("name" = String, Path, description = "Preset name (e.g., 'english', 'japanese')")
    ),
    responses(
        (status = 200, description = "Preset details with full ruleset", body = PresetDetailResponse),
        (status = 404, description = "Preset not found", body = ErrorResponse),
    ),
    tag = "Presets"
)]
pub async fn get_preset(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<PresetDetailResponse>, (StatusCode, Json<ErrorResponse>)> {
    let registry = state.preset_registry();

    match registry.load(&name) {
        Ok(ruleset) => Ok(Json(PresetDetailResponse {
            name: name.clone(),
            ruleset,
        })),
        Err(e) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: e,
                details: None,
            }),
        )),
    }
}

#[utoipa::path(
    get,
    path = "/api/example",
    responses(
        (status = 200, description = "Example ruleset template", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
    tag = "Presets"
)]
pub async fn example_ruleset(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let registry = state.preset_registry();
    let ruleset = registry.load("english").map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to load example ruleset".to_string(),
                details: None,
            }),
        )
    })?;

    Ok(Json(serde_json::json!({
        "ruleset": ruleset
    })))
}

fn format_display_name(name: &str) -> String {
    // Special cases for better display names
    match name {
        "english" => "English (USA)".to_string(),
        "british" => "English (UK)".to_string(),
        "lds" => "LDS (Latter-day Saints)".to_string(),
        _ => {
            // Capitalize first letter and replace underscores
            let formatted = name.replace('_', " ");
            let mut chars = formatted.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        }
    }
}

fn get_region(name: &str) -> String {
    match name {
        // North America
        "english" => "North America",

        // European
        "albanian" | "british" | "bulgarian" | "croatian" | "czech" | "danish" | "dutch"
        | "estonian" | "finnish" | "french" | "german" | "greek" | "hungarian" | "icelandic"
        | "italian" | "latvian" | "lithuanian" | "macedonian" | "norwegian" | "polish"
        | "portuguese" | "romanian" | "russian" | "serbian" | "slovak" | "slovenian"
        | "spanish" | "swedish" | "turkish" | "ukrainian" => "Europe",

        // Asian
        "chinese" | "japanese" | "korean" | "khmer" | "mongolian" | "thai" | "vietnamese" => "Asia",

        // Middle Eastern
        "arabic" | "armenian" | "farsi" => "Middle East",

        // Pacific
        "fijian" | "malagasy" | "malay" | "samoan" | "tongan" | "tagalog" => "Pacific",

        // African
        "swahili" => "Africa",

        // Caribbean & Latin American
        "haitian_creole" | "guarani" | "cebuano" => "Caribbean & Latin America",

        // Special
        "lds" => "Special",

        _ => "Other",
    }
    .to_string()
}
