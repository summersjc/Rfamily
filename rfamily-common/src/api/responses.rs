use rfamily_core::ruleset::Ruleset;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PresetInfo {
    /// Preset identifier (e.g., "english", "japanese")
    pub name: String,
    /// Human-readable display name
    pub display_name: String,
    /// Brief description of the preset
    pub description: String,
    /// Geographic region (e.g., "Europe", "Asia")
    pub region: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PresetsResponse {
    /// List of all available presets
    pub presets: Vec<PresetInfo>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PresetDetailResponse {
    /// Preset name
    pub name: String,
    /// Complete ruleset configuration
    pub ruleset: Ruleset,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ValidationResponse {
    /// Whether the ruleset is valid
    pub valid: bool,
    /// Validation errors (if any)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub errors: Vec<String>,
    /// Validation warnings (if any)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub warnings: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PreviewResponse {
    /// Generated GEDCOM content
    pub gedcom: String,
    /// Generation statistics
    pub statistics: GenerationStatistics,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GenerationStatistics {
    /// Total number of individuals generated
    pub total_individuals: usize,
    /// Number of male individuals
    pub males: usize,
    /// Number of female individuals
    pub females: usize,
    /// Number of families generated
    pub families: usize,
    /// Generation time in milliseconds
    pub generation_time_ms: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    /// Error message
    pub error: String,
    /// Additional error details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}
