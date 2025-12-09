use axum::{http::StatusCode, Json};
use rfamily_common::api::{ErrorResponse, ValidateRequest, ValidationResponse};

#[utoipa::path(
    post,
    path = "/api/validate",
    request_body = ValidateRequest,
    responses(
        (status = 200, description = "Validation result with errors and warnings", body = ValidationResponse),
    ),
    tag = "Validation"
)]
pub async fn validate_ruleset(
    Json(req): Json<ValidateRequest>,
) -> Result<Json<ValidationResponse>, (StatusCode, Json<ErrorResponse>)> {
    let ruleset = req.ruleset;

    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Validation logic for names
    if ruleset.names.male_given_names.is_empty() {
        errors.push("names.male_given_names: array cannot be empty".to_string());
    }
    if ruleset.names.female_given_names.is_empty() {
        errors.push("names.female_given_names: array cannot be empty".to_string());
    }
    if !ruleset.names.use_patronymic && ruleset.names.surnames.is_empty() {
        warnings.push("No surnames provided (using patronymic naming)".to_string());
    }

    // Validation logic for dates
    if ruleset.dates.birth_year_start >= ruleset.dates.birth_year_end {
        errors.push("dates.birth_year_start: must be less than birth_year_end".to_string());
    }
    if ruleset.dates.life_expectancy_mean > 150 {
        warnings.push(format!(
            "Life expectancy mean ({}) is unusually high",
            ruleset.dates.life_expectancy_mean
        ));
    }
    if ruleset.dates.min_marriage_age > ruleset.dates.max_marriage_age {
        errors.push(
            "dates.min_marriage_age: must be less than or equal to max_marriage_age".to_string(),
        );
    }
    if ruleset.dates.min_parent_age > ruleset.dates.max_parent_age {
        errors
            .push("dates.min_parent_age: must be less than or equal to max_parent_age".to_string());
    }

    // Validation logic for locations
    if ruleset.locations.countries.is_empty() {
        errors.push("locations.countries: must have at least one country".to_string());
    }

    // Validation logic for demographics
    if !(0.0..=1.0).contains(&ruleset.demographics.sex_ratio) {
        errors.push("demographics.sex_ratio: must be between 0.0 and 1.0".to_string());
    }
    if !(0.0..=1.0).contains(&ruleset.demographics.twin_rate) {
        errors.push("demographics.twin_rate: must be between 0.0 and 1.0".to_string());
    }
    if !(0.0..=1.0).contains(&ruleset.demographics.triplet_rate) {
        errors.push("demographics.triplet_rate: must be between 0.0 and 1.0".to_string());
    }

    // Validation logic for relationships
    if !(0.0..=1.0).contains(&ruleset.relationships.marriage_probability) {
        errors.push("relationships.marriage_probability: must be between 0.0 and 1.0".to_string());
    }
    if !(0.0..=1.0).contains(&ruleset.relationships.divorce_probability) {
        errors.push("relationships.divorce_probability: must be between 0.0 and 1.0".to_string());
    }
    if !(0.0..=1.0).contains(&ruleset.relationships.remarriage_probability) {
        errors
            .push("relationships.remarriage_probability: must be between 0.0 and 1.0".to_string());
    }
    if ruleset.relationships.min_children > ruleset.relationships.max_children {
        errors.push(
            "relationships.min_children: must be less than or equal to max_children".to_string(),
        );
    }

    let valid = errors.is_empty();

    Ok(Json(ValidationResponse {
        valid,
        errors,
        warnings,
    }))
}
