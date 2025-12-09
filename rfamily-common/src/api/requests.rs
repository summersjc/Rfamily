use rfamily_core::ruleset::Ruleset;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ValidateRequest {
    /// The ruleset to validate
    pub ruleset: Ruleset,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct PreviewRequest {
    /// Number of individuals to generate (10-100)
    #[schema(minimum = 10, maximum = 100)]
    pub count: usize,

    /// Name of preset to use (e.g., "english", "japanese")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset_name: Option<String>,

    /// Custom ruleset (if not using preset)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruleset: Option<Ruleset>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct GenerateRequest {
    /// Number of individuals to generate (100-10,000,000)
    #[schema(minimum = 100, maximum = 10000000)]
    pub count: usize,

    /// Name of preset to use (e.g., "english", "japanese")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset_name: Option<String>,

    /// Custom ruleset (if not using preset)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruleset: Option<Ruleset>,
}
