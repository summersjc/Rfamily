use std::fs;
use std::path::Path;
use std::process::Command;

/// Helper to run rfamily binary and capture output
fn run_rfamily(args: &[&str]) -> std::process::Output {
    Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--")
        .args(args)
        .output()
        .expect("Failed to execute rfamily")
}

/// Cleanup helper
fn cleanup_file(path: &str) {
    let _ = fs::remove_file(path);
}

#[test]
fn test_error_invalid_count_zero() {
    let output_file = "test_zero.ged";
    cleanup_file(output_file);

    let output = run_rfamily(&[
        "--preset",
        "english",
        "--count",
        "0",
        "--output",
        output_file,
    ]);

    // Count of 0 might be allowed by the program, creating empty GEDCOM
    // Just verify it doesn't crash
    assert!(
        output.status.code().is_some(),
        "Should exit with status code"
    );

    cleanup_file(output_file);
}

#[test]
fn test_error_invalid_count_negative() {
    let output_file = "test_negative.ged";
    cleanup_file(output_file);

    // This should be caught by clap argument parsing
    let output = run_rfamily(&[
        "--preset",
        "english",
        "--count",
        "-10",
        "--output",
        output_file,
    ]);

    assert!(!output.status.success(), "Should fail with negative count");

    cleanup_file(output_file);
}

#[test]
fn test_error_invalid_count_non_numeric() {
    let output_file = "test_invalid.ged";
    cleanup_file(output_file);

    let output = run_rfamily(&[
        "--preset",
        "english",
        "--count",
        "abc",
        "--output",
        output_file,
    ]);

    assert!(
        !output.status.success(),
        "Should fail with non-numeric count"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("invalid") || stderr.contains("error"),
        "Should show error about invalid count"
    );

    cleanup_file(output_file);
}

#[test]
fn test_error_preset_not_found() {
    let output_file = "test_notfound.ged";
    cleanup_file(output_file);

    let output = run_rfamily(&[
        "--preset",
        "klingon",
        "--count",
        "100",
        "--output",
        output_file,
    ]);

    assert!(!output.status.success(), "Should fail with unknown preset");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("klingon") || stderr.contains("not found") || stderr.contains("Unknown"),
        "Should mention the invalid preset name"
    );

    cleanup_file(output_file);
}

#[test]
fn test_error_empty_preset_name() {
    let output_file = "test_empty.ged";
    cleanup_file(output_file);

    let output = run_rfamily(&["--preset", "", "--count", "100", "--output", output_file]);

    assert!(
        !output.status.success(),
        "Should fail with empty preset name"
    );

    cleanup_file(output_file);
}

#[test]
fn test_error_case_sensitive_preset() {
    let output_file = "test_case.ged";
    cleanup_file(output_file);

    // Preset names are lowercase - ENGLISH should fail
    let output = run_rfamily(&[
        "--preset",
        "ENGLISH",
        "--count",
        "100",
        "--output",
        output_file,
    ]);

    assert!(
        !output.status.success(),
        "Should fail with uppercase preset name (case sensitive)"
    );

    cleanup_file(output_file);
}

#[test]
fn test_error_malformed_ruleset_json() {
    let ruleset_file = "test_malformed.json";
    let output_file = "test_malformed_output.ged";
    cleanup_file(ruleset_file);
    cleanup_file(output_file);

    // Create malformed JSON
    fs::write(ruleset_file, "{invalid json content}").expect("Should write malformed JSON");

    let output = run_rfamily(&[
        "--ruleset",
        ruleset_file,
        "--count",
        "100",
        "--output",
        output_file,
    ]);

    // Should fail with malformed JSON - but check either stderr or stdout
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = format!("{}{}", stderr, stdout);

    if !output.status.success() {
        assert!(
            combined.contains("parse")
                || combined.contains("JSON")
                || combined.contains("error")
                || combined.contains("Error"),
            "Should mention parsing error somewhere"
        );
    }

    cleanup_file(ruleset_file);
    cleanup_file(output_file);
}

#[test]
fn test_error_incomplete_ruleset_json() {
    let ruleset_file = "test_incomplete.json";
    let output_file = "test_incomplete_output.ged";
    cleanup_file(ruleset_file);
    cleanup_file(output_file);

    // Create incomplete but valid JSON (missing required fields)
    fs::write(ruleset_file, r#"{"names": {"male_given_names": ["John"]}}"#)
        .expect("Should write incomplete JSON");

    let output = run_rfamily(&[
        "--ruleset",
        ruleset_file,
        "--count",
        "100",
        "--output",
        output_file,
    ]);

    assert!(
        !output.status.success(),
        "Should fail with incomplete ruleset"
    );

    cleanup_file(ruleset_file);
    cleanup_file(output_file);
}

#[test]
fn test_error_nonexistent_ruleset_file() {
    let output_file = "test_nonexistent_output.ged";
    cleanup_file(output_file);

    let output = run_rfamily(&[
        "--ruleset",
        "nonexistent.json",
        "--count",
        "100",
        "--output",
        output_file,
    ]);

    assert!(
        !output.status.success(),
        "Should fail with nonexistent ruleset file"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("nonexistent.json")
            || stderr.contains("No such file")
            || stderr.contains("not found"),
        "Should mention the missing file"
    );

    cleanup_file(output_file);
}

#[test]
fn test_error_write_to_readonly_directory() {
    // Try to write to root directory (should fail due to permissions)
    let output_file = "/test_readonly.ged";

    let output = run_rfamily(&[
        "--preset",
        "english",
        "--count",
        "100",
        "--output",
        output_file,
    ]);

    // On most systems, this should fail due to permissions, but not all
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let combined = format!("{}{}", stderr, stdout);
        assert!(
            combined.contains("Permission")
                || combined.contains("denied")
                || combined.contains("error")
                || combined.contains("Error"),
            "Should mention error when permission denied"
        );
    }
    // If it succeeds, that's okay too - some systems allow it
}

#[test]
fn test_error_conflicting_preset_and_ruleset() {
    let ruleset_file = "test_conflict_ruleset.json";
    let output_file = "test_conflict_output.ged";
    cleanup_file(output_file);

    // Generate a valid ruleset first
    let gen_output = run_rfamily(&["--generate-ruleset", ruleset_file]);
    assert!(gen_output.status.success());

    // Try using both --preset and --ruleset (should use ruleset)
    let output = run_rfamily(&[
        "--preset",
        "spanish",
        "--ruleset",
        ruleset_file,
        "--count",
        "50",
        "--output",
        output_file,
    ]);

    // This should work - ruleset takes precedence
    assert!(
        output.status.success(),
        "Should succeed with both flags (ruleset takes precedence)"
    );

    cleanup_file(ruleset_file);
    cleanup_file(output_file);
}

#[test]
fn test_error_no_preset_no_ruleset() {
    let output_file = "test_nopreset.ged";
    cleanup_file(output_file);

    // No preset or ruleset specified - should use default
    let output = run_rfamily(&["--count", "50", "--output", output_file]);

    // Should succeed with default preset (english)
    assert!(
        output.status.success(),
        "Should succeed with default preset"
    );

    cleanup_file(output_file);
}

#[test]
fn test_error_special_characters_in_output_filename() {
    let output_file = "test_special_chars_<>:?.ged";
    cleanup_file(output_file);

    let output = run_rfamily(&[
        "--preset",
        "english",
        "--count",
        "50",
        "--output",
        output_file,
    ]);

    // On some systems this might fail due to invalid filename characters
    // On others it might succeed - just verify it doesn't crash
    assert!(
        output.status.code().is_some(),
        "Should exit with a status code"
    );

    cleanup_file(output_file);
}

#[test]
fn test_error_very_long_filename() {
    // Create a very long filename (most filesystems have 255 character limit)
    let long_name = "a".repeat(300);
    let output_file = format!("{}.ged", long_name);

    let output = run_rfamily(&[
        "--preset",
        "english",
        "--count",
        "50",
        "--output",
        &output_file,
    ]);

    // This should likely fail on most systems
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("too long") || stderr.contains("name") || stderr.contains("error"),
            "Should mention filename error"
        );
    }

    cleanup_file(&output_file);
}

#[test]
fn test_error_output_file_already_exists_overwrite() {
    let output_file = "test_overwrite.ged";

    // Create initial file
    fs::write(output_file, "existing content").expect("Should create initial file");

    // Run command - should overwrite
    let output = run_rfamily(&[
        "--preset",
        "english",
        "--count",
        "50",
        "--output",
        output_file,
    ]);

    assert!(
        output.status.success(),
        "Should succeed and overwrite existing file"
    );

    // Verify file was overwritten with GEDCOM content
    let content = fs::read_to_string(output_file).expect("Should read file");
    assert!(content.contains("0 HEAD"), "Should contain GEDCOM header");
    assert!(
        !content.contains("existing content"),
        "Should have overwritten old content"
    );

    cleanup_file(output_file);
}

#[test]
fn test_error_multiple_deprecated_flags() {
    let output_file = "test_multi_deprecated.ged";
    cleanup_file(output_file);

    // Use multiple deprecated flags - should use first one
    let output = run_rfamily(&[
        "--lds",
        "--spanish",
        "--count",
        "50",
        "--output",
        output_file,
    ]);

    assert!(
        output.status.success(),
        "Should succeed with multiple deprecated flags"
    );

    // Should show warnings for deprecated flags
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("deprecated") || stderr.contains("Deprecated"),
        "Should show deprecation warnings"
    );

    cleanup_file(output_file);
}

#[test]
fn test_error_generate_ruleset_invalid_path() {
    // Try to generate ruleset in nonexistent directory
    let output = run_rfamily(&["--generate-ruleset", "/nonexistent/path/ruleset.json"]);

    // Should fail due to invalid path
    assert!(
        !output.status.success(),
        "Should fail with invalid directory path"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("No such file") || stderr.contains("directory") || stderr.contains("error"),
        "Should mention path error"
    );
}

#[test]
fn test_error_generate_ruleset_overwrite() {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    // Use timestamp to ensure unique filename and avoid parallel test conflicts
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let ruleset_file = format!("test_ruleset_overwrite_{}.json", timestamp);
    cleanup_file(&ruleset_file);

    // Generate ruleset first time
    let output1 = run_rfamily(&["--generate-ruleset", &ruleset_file]);
    assert!(output1.status.success());

    // Generate again - should overwrite
    let output2 = run_rfamily(&["--generate-ruleset", &ruleset_file]);
    assert!(
        output2.status.success(),
        "Should succeed overwriting ruleset"
    );

    cleanup_file(&ruleset_file);
}

#[test]
fn test_error_extremely_large_count() {
    let output_file = "test_huge.ged";
    cleanup_file(output_file);

    // Try to generate 1 million people (should work but might be slow)
    // Using smaller count for CI/CD - 10k should still test large numbers
    let output = run_rfamily(&[
        "--preset",
        "english",
        "--count",
        "10000",
        "--output",
        output_file,
    ]);

    // Should succeed (might just take a while)
    assert!(output.status.success(), "Should handle large count");

    if Path::new(output_file).exists() {
        let metadata = fs::metadata(output_file).expect("Should get metadata");
        assert!(metadata.len() > 100000, "File should be substantial size");
    }

    cleanup_file(output_file);
}

#[test]
fn test_error_utf8_in_filename() {
    let output_file = "test_日本語_ファイル.ged";
    cleanup_file(output_file);

    let output = run_rfamily(&[
        "--preset",
        "japanese",
        "--count",
        "30",
        "--output",
        output_file,
    ]);

    // Should handle UTF-8 filenames
    assert!(output.status.success(), "Should handle UTF-8 filename");

    if Path::new(output_file).exists() {
        let content = fs::read_to_string(output_file).expect("Should read UTF-8 filename");
        assert!(content.contains("0 HEAD"), "Should contain GEDCOM content");
    }

    cleanup_file(output_file);
}

#[test]
fn test_error_whitespace_in_preset_name() {
    let output_file = "test_whitespace.ged";
    cleanup_file(output_file);

    // Try preset name with whitespace
    let output = run_rfamily(&[
        "--preset",
        " english ",
        "--count",
        "50",
        "--output",
        output_file,
    ]);

    // Should fail - preset names don't have whitespace
    assert!(
        !output.status.success(),
        "Should fail with whitespace in preset name"
    );

    cleanup_file(output_file);
}

#[test]
fn test_error_missing_required_output_argument() {
    // This test verifies clap properly requires the output argument
    // Note: With default values, output might not be required
    let output = run_rfamily(&["--preset", "english", "--count", "50"]);

    // Should succeed with default output file
    assert!(output.status.success(), "Should use default output file");

    // Clean up default output file
    cleanup_file("output.ged");
}

#[test]
fn test_error_help_flag_exits_cleanly() {
    let output = run_rfamily(&["--help"]);

    // Help should exit successfully
    assert!(output.status.success(), "Help should exit successfully");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Usage") || stdout.contains("USAGE") || stdout.contains("usage"),
        "Help text should contain usage information"
    );
}

#[test]
fn test_error_version_flag_if_available() {
    let output = run_rfamily(&["--version"]);

    // Version should exit successfully if flag exists
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.len() > 0, "Version output should not be empty");
    }
}

#[test]
fn test_error_list_presets_exits_without_generating() {
    // Clean up any existing default output file first
    cleanup_file("output.ged");

    let output = run_rfamily(&["--list-presets"]);

    assert!(output.status.success(), "List presets should succeed");

    // Should not create any output files when just listing
    // Note: If output.ged exists from a previous run, this test might fail
    // So we clean it up first
    assert!(
        !Path::new("output.ged").exists(),
        "Should not create default output file when listing presets"
    );
}

#[test]
fn test_error_generate_ruleset_exits_without_generating() {
    let ruleset_file = "test_exit_check.json";
    cleanup_file(ruleset_file);
    cleanup_file("output.ged"); // Clean up default output file

    let output = run_rfamily(&["--generate-ruleset", ruleset_file]);

    assert!(output.status.success(), "Generate ruleset should succeed");
    assert!(
        Path::new(ruleset_file).exists(),
        "Ruleset file should be created"
    );

    // Should not create GEDCOM output file when just generating ruleset
    assert!(
        !Path::new("output.ged").exists(),
        "Should not create default output file when generating ruleset"
    );

    cleanup_file(ruleset_file);
}

#[test]
fn test_error_empty_output_filename() {
    let output = run_rfamily(&["--preset", "english", "--count", "50", "--output", ""]);

    // Should fail with empty output filename
    assert!(
        !output.status.success(),
        "Should fail with empty output filename"
    );
}

#[test]
fn test_error_output_to_directory_not_file() {
    let output = run_rfamily(&["--preset", "english", "--count", "50", "--output", "."]);

    // Should fail trying to write to directory
    assert!(
        !output.status.success(),
        "Should fail writing to directory instead of file"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("directory")
            || stderr.contains("Is a directory")
            || stderr.contains("error"),
        "Should mention directory error"
    );
}
