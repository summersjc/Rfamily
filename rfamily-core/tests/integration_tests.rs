use std::fs;
use std::path::Path;
use std::process::Command;

/// Helper to run rfamily binary and capture output
fn run_rfamily(args: &[&str]) -> std::process::Output {
    Command::new("cargo")
        .arg("run")
        .arg("-p")
        .arg("rfamily-cli")
        .arg("--quiet")
        .arg("--")
        .args(args)
        .output()
        .expect("Failed to execute rfamily")
}

/// Helper to check if output file exists and is valid GEDCOM
fn verify_gedcom_file(path: &str) -> (bool, String) {
    if !Path::new(path).exists() {
        return (false, "File does not exist".to_string());
    }

    match fs::read_to_string(path) {
        Ok(content) => {
            let has_header = content.contains("0 HEAD");
            let has_trailer = content.contains("0 TRLR");
            let has_individuals = content.contains("0 @I");

            if !has_header {
                (false, "Missing GEDCOM header (0 HEAD)".to_string())
            } else if !has_trailer {
                (false, "Missing GEDCOM trailer (0 TRLR)".to_string())
            } else if !has_individuals {
                (false, "No individuals found (0 @I)".to_string())
            } else {
                (true, content)
            }
        }
        Err(e) => (false, format!("Failed to read file: {}", e)),
    }
}

/// Helper to count individuals in GEDCOM file
fn count_individuals(content: &str) -> usize {
    content
        .lines()
        .filter(|line| line.starts_with("0 @I"))
        .count()
}

/// Helper to count families in GEDCOM file
fn count_families(content: &str) -> usize {
    content
        .lines()
        .filter(|line| line.starts_with("0 @F"))
        .count()
}

/// Cleanup helper
fn cleanup_file(path: &str) {
    let _ = fs::remove_file(path);
}

#[test]
fn test_cli_list_presets() {
    let output = run_rfamily(&["--list-presets"]);

    assert!(output.status.success(), "Command should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check for all 52 presets
    assert!(stdout.contains("albanian"), "Missing albanian preset");
    assert!(stdout.contains("british"), "Missing british preset");
    assert!(stdout.contains("japanese"), "Missing japanese preset");
    assert!(stdout.contains("spanish"), "Missing spanish preset");
    assert!(stdout.contains("lds"), "Missing lds preset");
    assert!(stdout.contains("vietnamese"), "Missing vietnamese preset");

    // Count lines (should be 55: header + 52 presets + 2 blank lines)
    let line_count = stdout.lines().filter(|l| !l.trim().is_empty()).count();
    assert!(
        line_count >= 53,
        "Should list all 52+ presets, got {}",
        line_count
    );
}

#[test]
fn test_cli_generate_with_preset() {
    let output_file = "test_preset_output.ged";
    cleanup_file(output_file);

    let output = run_rfamily(&[
        "--preset",
        "english",
        "--count",
        "100",
        "--output",
        output_file,
    ]);

    assert!(output.status.success(), "Command should succeed");

    let (valid, content) = verify_gedcom_file(output_file);
    assert!(valid, "Generated file should be valid GEDCOM");

    let individual_count = count_individuals(&content);
    assert!(
        individual_count >= 100,
        "Should generate at least 100 individuals, got {}",
        individual_count
    );
    assert!(
        individual_count <= 120,
        "Should not generate too many individuals, got {}",
        individual_count
    );

    // Check for English-specific content
    assert!(
        content.contains("1 CHAR UTF-8"),
        "Should use UTF-8 encoding"
    );
    assert!(content.contains("1 NAME"), "Should have names");

    cleanup_file(output_file);
}

#[test]
fn test_cli_generate_with_japanese_preset() {
    let output_file = "test_japanese_output.ged";
    cleanup_file(output_file);

    let output = run_rfamily(&[
        "--preset",
        "japanese",
        "--count",
        "50",
        "--output",
        output_file,
    ]);

    assert!(output.status.success(), "Command should succeed");

    let (valid, content) = verify_gedcom_file(output_file);
    assert!(valid, "Generated file should be valid GEDCOM");

    let individual_count = count_individuals(&content);
    assert!(
        individual_count >= 50,
        "Should generate at least 50 individuals, got {}",
        individual_count
    );
    assert!(
        individual_count <= 65,
        "Should not generate too many individuals, got {}",
        individual_count
    );

    // Japanese content uses UTF-8
    assert!(
        content.contains("1 CHAR UTF-8"),
        "Should use UTF-8 encoding"
    );

    cleanup_file(output_file);
}

#[test]
fn test_cli_generate_with_arabic_preset() {
    let output_file = "test_arabic_output.ged";
    cleanup_file(output_file);

    let output = run_rfamily(&[
        "--preset",
        "arabic",
        "--count",
        "50",
        "--output",
        output_file,
    ]);

    assert!(output.status.success(), "Command should succeed");

    let (valid, content) = verify_gedcom_file(output_file);
    assert!(valid, "Generated file should be valid GEDCOM");

    let individual_count = count_individuals(&content);
    assert!(
        individual_count >= 50,
        "Should generate at least 50 individuals, got {}",
        individual_count
    );
    assert!(
        individual_count <= 65,
        "Should not generate too many individuals, got {}",
        individual_count
    );

    cleanup_file(output_file);
}

#[test]
fn test_cli_deprecated_lds_flag() {
    let output_file = "test_lds_output.ged";
    cleanup_file(output_file);

    let output = run_rfamily(&["--lds", "--count", "100", "--output", output_file]);

    assert!(output.status.success(), "Command should succeed");

    // Should show deprecation warning
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("deprecated") || stderr.contains("Deprecated"),
        "Should show deprecation warning"
    );

    let (valid, content) = verify_gedcom_file(output_file);
    assert!(valid, "Generated file should be valid GEDCOM");

    // LDS preset should include ordinances
    assert!(
        content.contains("BAPL") || content.contains("CONF") || content.contains("ENDL"),
        "LDS preset should include ordinance records"
    );

    cleanup_file(output_file);
}

#[test]
fn test_cli_deprecated_spanish_flag() {
    let output_file = "test_spanish_output.ged";
    cleanup_file(output_file);

    let output = run_rfamily(&["--spanish", "--count", "75", "--output", output_file]);

    assert!(output.status.success(), "Command should succeed");

    // Should show deprecation warning
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("deprecated") || stderr.contains("Deprecated"),
        "Should show deprecation warning"
    );

    let (valid, content) = verify_gedcom_file(output_file);
    assert!(valid, "Generated file should be valid GEDCOM");

    let individual_count = count_individuals(&content);
    assert!(
        individual_count >= 75,
        "Should generate at least 75 individuals, got {}",
        individual_count
    );
    assert!(
        individual_count <= 100,
        "Should not generate too many individuals, got {}",
        individual_count
    );

    cleanup_file(output_file);
}

#[test]
fn test_cli_short_flags() {
    let output_file = "test_short_flags.ged";
    cleanup_file(output_file);

    let output = run_rfamily(&["-p", "korean", "-c", "60", "-o", output_file]);

    assert!(output.status.success(), "Command should succeed");

    let (valid, content) = verify_gedcom_file(output_file);
    assert!(valid, "Generated file should be valid GEDCOM");

    let individual_count = count_individuals(&content);
    assert!(
        individual_count >= 60,
        "Should generate at least 60 individuals, got {}",
        individual_count
    );
    assert!(
        individual_count <= 80,
        "Should not generate too many individuals, got {}",
        individual_count
    );

    cleanup_file(output_file);
}

#[test]
fn test_cli_generate_ruleset() {
    let ruleset_file = "test_generated_ruleset.json";
    cleanup_file(ruleset_file);

    let output = run_rfamily(&["--generate-ruleset", ruleset_file]);

    assert!(output.status.success(), "Command should succeed");
    assert!(
        Path::new(ruleset_file).exists(),
        "Ruleset file should be created"
    );

    // Verify it's valid JSON
    let content = fs::read_to_string(ruleset_file).expect("Should read ruleset file");
    let parsed: serde_json::Value =
        serde_json::from_str(&content).expect("Generated ruleset should be valid JSON");

    // Check for expected fields in names object
    let names = parsed.get("names").expect("Should have names object");
    assert!(
        names.get("male_given_names").is_some(),
        "Should have male_given_names"
    );
    assert!(
        names.get("female_given_names").is_some(),
        "Should have female_given_names"
    );
    assert!(names.get("surnames").is_some(), "Should have surnames");
    assert!(parsed.get("locations").is_some(), "Should have locations");

    cleanup_file(ruleset_file);
}

#[test]
fn test_cli_with_custom_ruleset() {
    // First generate a ruleset
    let ruleset_file = "test_custom_ruleset.json";
    let output_file = "test_custom_output.ged";
    cleanup_file(ruleset_file);
    cleanup_file(output_file);

    let gen_output = run_rfamily(&["--generate-ruleset", ruleset_file]);
    assert!(gen_output.status.success(), "Should generate ruleset");

    // Now use it to generate GEDCOM
    let output = run_rfamily(&[
        "--ruleset",
        ruleset_file,
        "--count",
        "80",
        "--output",
        output_file,
    ]);

    assert!(output.status.success(), "Command should succeed");

    let (valid, content) = verify_gedcom_file(output_file);
    assert!(valid, "Generated file should be valid GEDCOM");

    let individual_count = count_individuals(&content);
    assert!(
        individual_count >= 80,
        "Should generate at least 80 individuals, got {}",
        individual_count
    );
    assert!(
        individual_count <= 100,
        "Should not generate too many individuals, got {}",
        individual_count
    );

    cleanup_file(ruleset_file);
    cleanup_file(output_file);
}

#[test]
fn test_cli_family_relationships() {
    let output_file = "test_families.ged";
    cleanup_file(output_file);

    let output = run_rfamily(&[
        "--preset",
        "english",
        "--count",
        "200",
        "--output",
        output_file,
    ]);

    assert!(output.status.success(), "Command should succeed");

    let (valid, content) = verify_gedcom_file(output_file);
    assert!(valid, "Generated file should be valid GEDCOM");

    let family_count = count_families(&content);
    assert!(family_count > 0, "Should generate at least some families");

    // Check for family relationships
    assert!(content.contains("1 HUSB"), "Should have husband records");
    assert!(content.contains("1 WIFE"), "Should have wife records");
    assert!(content.contains("1 CHIL"), "Should have child records");

    cleanup_file(output_file);
}

#[test]
fn test_cli_invalid_preset_name() {
    let output_file = "test_invalid.ged";
    cleanup_file(output_file);

    let output = run_rfamily(&[
        "--preset",
        "nonexistent_preset",
        "--count",
        "100",
        "--output",
        output_file,
    ]);

    // Should fail with error
    assert!(
        !output.status.success(),
        "Command should fail with invalid preset"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Unknown preset") || stderr.contains("not found"),
        "Should show error message about unknown preset"
    );

    cleanup_file(output_file);
}

#[test]
fn test_cli_utf8_encoding_consistency() {
    let output_file = "test_utf8.ged";
    cleanup_file(output_file);

    // Test multiple presets with different character sets
    for preset in &["chinese", "thai", "arabic", "russian", "korean"] {
        let output = run_rfamily(&["--preset", preset, "--count", "30", "--output", output_file]);

        assert!(
            output.status.success(),
            "Command should succeed for {} preset",
            preset
        );

        let (valid, content) = verify_gedcom_file(output_file);
        assert!(
            valid,
            "Generated file should be valid GEDCOM for {} preset",
            preset
        );

        // All should use UTF-8
        assert!(
            content.contains("1 CHAR UTF-8"),
            "{} preset should use UTF-8 encoding",
            preset
        );

        cleanup_file(output_file);
    }
}

#[test]
fn test_cli_large_generation() {
    let output_file = "test_large.ged";
    cleanup_file(output_file);

    // Generate 1000 individuals
    let output = run_rfamily(&[
        "--preset",
        "english",
        "--count",
        "1000",
        "--output",
        output_file,
    ]);

    assert!(
        output.status.success(),
        "Command should succeed for large generation"
    );

    let (valid, content) = verify_gedcom_file(output_file);
    assert!(valid, "Generated file should be valid GEDCOM");

    let individual_count = count_individuals(&content);
    assert!(
        individual_count >= 1000,
        "Should generate at least 1000 individuals, got {}",
        individual_count
    );
    assert!(
        individual_count <= 1050,
        "Should not generate too many individuals, got {}",
        individual_count
    );

    // Check file size is reasonable (should be multiple KB)
    let metadata = fs::metadata(output_file).expect("Should get file metadata");
    assert!(
        metadata.len() > 10000,
        "File should be at least 10KB for 1000 individuals"
    );

    cleanup_file(output_file);
}

#[test]
fn test_cli_all_european_presets() {
    let output_file = "test_european.ged";

    let european_presets = vec![
        "albanian",
        "bulgarian",
        "croatian",
        "czech",
        "danish",
        "dutch",
        "english",
        "estonian",
        "finnish",
        "french",
        "german",
        "greek",
        "hungarian",
        "icelandic",
        "italian",
        "latvian",
        "lithuanian",
        "macedonian",
        "norwegian",
        "polish",
        "portuguese",
        "romanian",
        "russian",
        "serbian",
        "slovak",
        "slovenian",
        "spanish",
        "swedish",
        "turkish",
        "ukrainian",
    ];

    for preset in european_presets {
        cleanup_file(output_file);

        let output = run_rfamily(&["--preset", preset, "--count", "50", "--output", output_file]);

        assert!(
            output.status.success(),
            "Command should succeed for {} preset",
            preset
        );

        let (valid, _) = verify_gedcom_file(output_file);
        assert!(
            valid,
            "Generated file should be valid GEDCOM for {} preset",
            preset
        );
    }

    cleanup_file(output_file);
}

#[test]
fn test_cli_all_asian_presets() {
    let output_file = "test_asian.ged";

    let asian_presets = vec![
        "chinese",
        "japanese",
        "khmer",
        "korean",
        "mongolian",
        "thai",
        "vietnamese",
    ];

    for preset in asian_presets {
        cleanup_file(output_file);

        let output = run_rfamily(&["--preset", preset, "--count", "50", "--output", output_file]);

        assert!(
            output.status.success(),
            "Command should succeed for {} preset",
            preset
        );

        let (valid, _) = verify_gedcom_file(output_file);
        assert!(
            valid,
            "Generated file should be valid GEDCOM for {} preset",
            preset
        );
    }

    cleanup_file(output_file);
}

#[test]
fn test_cli_all_pacific_presets() {
    let output_file = "test_pacific.ged";

    let pacific_presets = vec![
        "cebuano", "fijian", "malagasy", "malay", "samoan", "tagalog", "tongan",
    ];

    for preset in pacific_presets {
        cleanup_file(output_file);

        let output = run_rfamily(&["--preset", preset, "--count", "50", "--output", output_file]);

        assert!(
            output.status.success(),
            "Command should succeed for {} preset",
            preset
        );

        let (valid, _) = verify_gedcom_file(output_file);
        assert!(
            valid,
            "Generated file should be valid GEDCOM for {} preset",
            preset
        );
    }

    cleanup_file(output_file);
}

#[test]
fn test_cli_generate_ious() {
    let output_file = "test_ious_output.ged";
    cleanup_file(output_file);

    let output = run_rfamily(&[
        "generate-ious",
        "--preset",
        "english",
        "--output",
        output_file,
        "--marriages",
        "2",
        "--children-per-marriage",
        "3.0",
        "--siblings",
        "2",
        "--descendant-gens",
        "2",
    ]);

    assert!(
        output.status.success(),
        "generate-ious command should succeed"
    );

    let (valid, content) = verify_gedcom_file(output_file);
    assert!(valid, "Generated IOUS file should be valid GEDCOM");

    // Verify we have a reasonable number of individuals
    let count = count_individuals(&content);
    assert!(
        count >= 5,
        "IOUS should generate at least 5 individuals (IOUS + spouse + children), got {}",
        count
    );

    // Verify we have families
    let family_count = content
        .lines()
        .filter(|line| line.starts_with("0 @F"))
        .count();
    assert!(
        family_count >= 2,
        "IOUS with 2 marriages should have at least 2 families, got {}",
        family_count
    );

    cleanup_file(output_file);
}

#[test]
fn test_cli_generate_ious_with_target_limit() {
    let output_file = "test_ious_limited.ged";
    cleanup_file(output_file);

    let output = run_rfamily(&[
        "generate-ious",
        "--preset",
        "japanese",
        "--output",
        output_file,
        "--marriages",
        "5",
        "--children-per-marriage",
        "5.0",
        "--siblings",
        "5",
        "--descendant-gens",
        "3",
        "--total-descendants",
        "30",
    ]);

    assert!(
        output.status.success(),
        "generate-ious with limit should succeed"
    );

    let (valid, content) = verify_gedcom_file(output_file);
    assert!(valid, "Generated IOUS file should be valid GEDCOM");

    // Verify limit is respected
    let count = count_individuals(&content);
    assert!(
        count <= 30,
        "IOUS should respect total-descendants limit, got {}",
        count
    );

    cleanup_file(output_file);
}

#[test]
fn test_cli_generate_ious_minimal() {
    let output_file = "test_ious_minimal.ged";
    cleanup_file(output_file);

    // Minimal IOUS generation
    let output = run_rfamily(&[
        "generate-ious",
        "--preset",
        "spanish",
        "--output",
        output_file,
        "--marriages",
        "1",
        "--children-per-marriage",
        "1.0",
        "--siblings",
        "0",
        "--descendant-gens",
        "1",
    ]);

    assert!(
        output.status.success(),
        "Minimal IOUS generation should succeed"
    );

    let (valid, content) = verify_gedcom_file(output_file);
    assert!(valid, "Minimal IOUS file should be valid GEDCOM");

    // Should have at least IOUS + spouse + 1 child
    let count = count_individuals(&content);
    assert!(
        count >= 3,
        "Minimal IOUS should have at least 3 individuals, got {}",
        count
    );

    cleanup_file(output_file);
}
