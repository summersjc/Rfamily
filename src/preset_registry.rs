use crate::ruleset::Ruleset;
use std::collections::HashMap;

pub struct PresetRegistry {
    presets: HashMap<String, &'static str>,
}

impl PresetRegistry {
    pub fn new() -> Self {
        let mut presets = HashMap::new();

        // Embed all preset JSON files at compile time
        // European Languages
        presets.insert(
            "albanian".to_string(),
            include_str!("../presets/albanian.json"),
        );
        presets.insert(
            "bulgarian".to_string(),
            include_str!("../presets/bulgarian.json"),
        );
        presets.insert(
            "croatian".to_string(),
            include_str!("../presets/croatian.json"),
        );
        presets.insert("czech".to_string(), include_str!("../presets/czech.json"));
        presets.insert("danish".to_string(), include_str!("../presets/danish.json"));
        presets.insert("dutch".to_string(), include_str!("../presets/dutch.json"));
        presets.insert(
            "english".to_string(),
            include_str!("../presets/english.json"),
        );
        presets.insert(
            "estonian".to_string(),
            include_str!("../presets/estonian.json"),
        );
        presets.insert(
            "finnish".to_string(),
            include_str!("../presets/finnish.json"),
        );
        presets.insert("french".to_string(), include_str!("../presets/french.json"));
        presets.insert("german".to_string(), include_str!("../presets/german.json"));
        presets.insert("greek".to_string(), include_str!("../presets/greek.json"));
        presets.insert(
            "hungarian".to_string(),
            include_str!("../presets/hungarian.json"),
        );
        presets.insert(
            "icelandic".to_string(),
            include_str!("../presets/icelandic.json"),
        );
        presets.insert(
            "italian".to_string(),
            include_str!("../presets/italian.json"),
        );
        presets.insert(
            "latvian".to_string(),
            include_str!("../presets/latvian.json"),
        );
        presets.insert(
            "lithuanian".to_string(),
            include_str!("../presets/lithuanian.json"),
        );
        presets.insert(
            "macedonian".to_string(),
            include_str!("../presets/macedonian.json"),
        );
        presets.insert(
            "norwegian".to_string(),
            include_str!("../presets/norwegian.json"),
        );
        presets.insert("polish".to_string(), include_str!("../presets/polish.json"));
        presets.insert(
            "portuguese".to_string(),
            include_str!("../presets/portuguese.json"),
        );
        presets.insert(
            "romanian".to_string(),
            include_str!("../presets/romanian.json"),
        );
        presets.insert(
            "russian".to_string(),
            include_str!("../presets/russian.json"),
        );
        presets.insert(
            "serbian".to_string(),
            include_str!("../presets/serbian.json"),
        );
        presets.insert("slovak".to_string(), include_str!("../presets/slovak.json"));
        presets.insert(
            "slovenian".to_string(),
            include_str!("../presets/slovenian.json"),
        );
        presets.insert(
            "spanish".to_string(),
            include_str!("../presets/spanish.json"),
        );
        presets.insert(
            "swedish".to_string(),
            include_str!("../presets/swedish.json"),
        );
        presets.insert(
            "turkish".to_string(),
            include_str!("../presets/turkish.json"),
        );
        presets.insert(
            "ukrainian".to_string(),
            include_str!("../presets/ukrainian.json"),
        );

        // Middle Eastern Languages
        presets.insert("arabic".to_string(), include_str!("../presets/arabic.json"));
        presets.insert(
            "armenian".to_string(),
            include_str!("../presets/armenian.json"),
        );
        presets.insert("farsi".to_string(), include_str!("../presets/farsi.json"));

        // Asian Languages
        presets.insert(
            "chinese".to_string(),
            include_str!("../presets/chinese.json"),
        );
        presets.insert(
            "japanese".to_string(),
            include_str!("../presets/japanese.json"),
        );
        presets.insert("korean".to_string(), include_str!("../presets/korean.json"));
        presets.insert("khmer".to_string(), include_str!("../presets/khmer.json"));
        presets.insert(
            "mongolian".to_string(),
            include_str!("../presets/mongolian.json"),
        );
        presets.insert("thai".to_string(), include_str!("../presets/thai.json"));
        presets.insert(
            "vietnamese".to_string(),
            include_str!("../presets/vietnamese.json"),
        );

        // Pacific Languages
        presets.insert("fijian".to_string(), include_str!("../presets/fijian.json"));
        presets.insert(
            "malagasy".to_string(),
            include_str!("../presets/malagasy.json"),
        );
        presets.insert("malay".to_string(), include_str!("../presets/malay.json"));
        presets.insert("samoan".to_string(), include_str!("../presets/samoan.json"));
        presets.insert("tongan".to_string(), include_str!("../presets/tongan.json"));
        presets.insert(
            "tagalog".to_string(),
            include_str!("../presets/tagalog.json"),
        );

        // African Languages
        presets.insert(
            "swahili".to_string(),
            include_str!("../presets/swahili.json"),
        );

        // Caribbean/Latin American Languages
        presets.insert(
            "haitian".to_string(),
            include_str!("../presets/haitian.json"),
        );
        presets.insert(
            "guarani".to_string(),
            include_str!("../presets/guarani.json"),
        );
        presets.insert(
            "cebuano".to_string(),
            include_str!("../presets/cebuano.json"),
        );

        // Special Presets
        presets.insert("lds".to_string(), include_str!("../presets/lds.json"));

        PresetRegistry { presets }
    }

    pub fn load(&self, name: &str) -> Result<Ruleset, String> {
        match self.presets.get(name) {
            Some(json) => serde_json::from_str(json)
                .map_err(|e| format!("Failed to parse preset '{}': {}", name, e)),
            None => Err(format!(
                "Preset '{}' not found. Use --list-presets to see available presets.",
                name
            )),
        }
    }

    pub fn list(&self) -> Vec<String> {
        let mut names: Vec<String> = self.presets.keys().cloned().collect();
        names.sort();
        names
    }

    #[allow(dead_code)]
    pub fn has_preset(&self, name: &str) -> bool {
        self.presets.contains_key(name)
    }
}

impl Default for PresetRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_loads_all_presets() {
        let registry = PresetRegistry::new();
        let presets = registry.list();

        assert!(
            presets.len() >= 51,
            "Expected at least 51 presets, found {}",
            presets.len()
        );

        // Check core presets exist
        assert!(registry.has_preset("english"));
        assert!(registry.has_preset("spanish"));
        assert!(registry.has_preset("french"));
        assert!(registry.has_preset("italian"));
        assert!(registry.has_preset("icelandic"));
        assert!(registry.has_preset("lds"));

        // Check some new language presets
        assert!(registry.has_preset("albanian"));
        assert!(registry.has_preset("german"));
        assert!(registry.has_preset("japanese"));
        assert!(registry.has_preset("arabic"));
        assert!(registry.has_preset("swahili"));
    }

    #[test]
    fn test_load_valid_preset() {
        let registry = PresetRegistry::new();
        let ruleset = registry.load("english").unwrap();

        assert!(!ruleset.names.male_given_names.is_empty());
        assert!(!ruleset.names.female_given_names.is_empty());
        assert!(!ruleset.names.surnames.is_empty());
    }

    #[test]
    fn test_load_invalid_preset() {
        let registry = PresetRegistry::new();
        let result = registry.load("nonexistent");

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_all_listed_presets_load() {
        let registry = PresetRegistry::new();

        for preset_name in registry.list() {
            let result = registry.load(&preset_name);
            assert!(result.is_ok(), "Failed to load preset: {}", preset_name);
        }
    }

    #[test]
    fn test_preset_count_is_exactly_51() {
        let registry = PresetRegistry::new();
        let presets = registry.list();

        assert_eq!(
            presets.len(),
            51,
            "Expected exactly 51 presets, found {}",
            presets.len()
        );
    }

    #[test]
    fn test_all_european_presets_exist() {
        let registry = PresetRegistry::new();
        let european = vec![
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

        for preset in european {
            assert!(
                registry.has_preset(preset),
                "Missing European preset: {}",
                preset
            );
        }
    }

    #[test]
    fn test_all_asian_presets_exist() {
        let registry = PresetRegistry::new();
        let asian = vec![
            "chinese",
            "japanese",
            "korean",
            "khmer",
            "mongolian",
            "thai",
            "vietnamese",
        ];

        for preset in asian {
            assert!(
                registry.has_preset(preset),
                "Missing Asian preset: {}",
                preset
            );
        }
    }

    #[test]
    fn test_all_middle_eastern_presets_exist() {
        let registry = PresetRegistry::new();
        let middle_eastern = vec!["arabic", "armenian", "farsi"];

        for preset in middle_eastern {
            assert!(
                registry.has_preset(preset),
                "Missing Middle Eastern preset: {}",
                preset
            );
        }
    }

    #[test]
    fn test_all_pacific_presets_exist() {
        let registry = PresetRegistry::new();
        let pacific = vec![
            "cebuano", "fijian", "malagasy", "malay", "samoan", "tagalog", "tongan",
        ];

        for preset in pacific {
            assert!(
                registry.has_preset(preset),
                "Missing Pacific preset: {}",
                preset
            );
        }
    }

    #[test]
    fn test_african_and_special_presets_exist() {
        let registry = PresetRegistry::new();

        assert!(
            registry.has_preset("swahili"),
            "Missing African preset: swahili"
        );
        assert!(
            registry.has_preset("haitian"),
            "Missing Caribbean preset: haitian"
        );
        assert!(
            registry.has_preset("guarani"),
            "Missing Latin American preset: guarani"
        );
        assert!(registry.has_preset("lds"), "Missing special preset: lds");
    }

    #[test]
    fn test_preset_list_is_sorted() {
        let registry = PresetRegistry::new();
        let presets = registry.list();

        let mut sorted_presets = presets.clone();
        sorted_presets.sort();

        assert_eq!(
            presets, sorted_presets,
            "Preset list should be sorted alphabetically"
        );
    }

    #[test]
    fn test_all_presets_have_valid_structure() {
        let registry = PresetRegistry::new();

        for preset_name in registry.list() {
            let ruleset = registry.load(&preset_name).unwrap();

            // Check names
            assert!(
                !ruleset.names.male_given_names.is_empty(),
                "{} preset missing male names",
                preset_name
            );
            assert!(
                !ruleset.names.female_given_names.is_empty(),
                "{} preset missing female names",
                preset_name
            );

            // Icelandic uses patronymic naming and may not have surnames
            if !ruleset.names.use_patronymic && !ruleset.names.use_matronymic {
                assert!(
                    !ruleset.names.surnames.is_empty(),
                    "{} preset missing surnames",
                    preset_name
                );
            }

            // Check locations
            assert!(
                !ruleset.locations.countries.is_empty(),
                "{} preset missing countries",
                preset_name
            );

            // Check at least 5 names of each type
            assert!(
                ruleset.names.male_given_names.len() >= 5,
                "{} preset has too few male names",
                preset_name
            );
            assert!(
                ruleset.names.female_given_names.len() >= 5,
                "{} preset has too few female names",
                preset_name
            );
            assert!(
                ruleset.locations.countries.len() >= 1,
                "{} preset has no countries",
                preset_name
            );
        }
    }

    #[test]
    fn test_all_presets_have_valid_demographics() {
        let registry = PresetRegistry::new();

        for preset_name in registry.list() {
            let ruleset = registry.load(&preset_name).unwrap();

            // Check demographics exist
            let demo = &ruleset.demographics;

            // Check probabilities are valid (between 0 and 1)
            assert!(
                demo.sex_ratio >= 0.0 && demo.sex_ratio <= 1.0,
                "{} preset has invalid sex_ratio",
                preset_name
            );
            assert!(
                demo.twin_rate >= 0.0 && demo.twin_rate <= 1.0,
                "{} preset has invalid twin_rate",
                preset_name
            );
            assert!(
                demo.triplet_rate >= 0.0 && demo.triplet_rate <= 1.0,
                "{} preset has invalid triplet_rate",
                preset_name
            );

            // Check relationship probabilities
            let rel = &ruleset.relationships;
            assert!(
                rel.marriage_probability >= 0.0 && rel.marriage_probability <= 1.0,
                "{} preset has invalid marriage_probability",
                preset_name
            );
            assert!(
                rel.divorce_probability >= 0.0 && rel.divorce_probability <= 1.0,
                "{} preset has invalid divorce_probability",
                preset_name
            );

            // Check children parameters
            assert!(
                rel.children_mean > 0.0,
                "{} preset has invalid children_mean",
                preset_name
            );
            assert!(
                rel.children_stddev >= 0.0,
                "{} preset has invalid children_stddev",
                preset_name
            );
        }
    }

    #[test]
    fn test_all_presets_have_valid_dates() {
        let registry = PresetRegistry::new();

        for preset_name in registry.list() {
            let ruleset = registry.load(&preset_name).unwrap();

            // Check date ranges
            assert!(
                ruleset.dates.birth_year_start < ruleset.dates.birth_year_end,
                "{} preset has invalid birth year range",
                preset_name
            );
            assert!(
                ruleset.dates.min_marriage_age > 0,
                "{} preset has invalid min_marriage_age",
                preset_name
            );
            assert!(
                ruleset.dates.max_marriage_age >= ruleset.dates.min_marriage_age,
                "{} preset has invalid max_marriage_age",
                preset_name
            );
            assert!(
                ruleset.dates.min_parent_age > 0,
                "{} preset has invalid min_parent_age",
                preset_name
            );
            assert!(
                ruleset.dates.max_parent_age >= ruleset.dates.min_parent_age,
                "{} preset has invalid max_parent_age",
                preset_name
            );
        }
    }

    #[test]
    fn test_lds_preset_has_ordinances_enabled() {
        let registry = PresetRegistry::new();
        let lds_ruleset = registry.load("lds").unwrap();

        assert!(
            lds_ruleset.ordinances.include_lds_ordinances,
            "LDS preset should have ordinances enabled"
        );
        assert!(
            !lds_ruleset.ordinances.temples.is_empty(),
            "LDS preset should have temple list"
        );
    }

    #[test]
    fn test_non_lds_presets_have_ordinances_disabled() {
        let registry = PresetRegistry::new();

        for preset_name in registry.list() {
            if preset_name == "lds" {
                continue;
            }

            let ruleset = registry.load(&preset_name).unwrap();
            assert!(
                !ruleset.ordinances.include_lds_ordinances,
                "{} preset should have ordinances disabled",
                preset_name
            );
        }
    }

    #[test]
    fn test_has_preset_returns_false_for_invalid() {
        let registry = PresetRegistry::new();

        assert!(!registry.has_preset("nonexistent"));
        assert!(!registry.has_preset(""));
        assert!(!registry.has_preset("ENGLISH")); // Case sensitive
        assert!(!registry.has_preset("hindi")); // Not yet implemented
    }

    #[test]
    fn test_load_error_message_quality() {
        let registry = PresetRegistry::new();
        let error = registry.load("notfound").unwrap_err();

        assert!(
            error.contains("notfound"),
            "Error should mention the preset name"
        );
        assert!(
            error.contains("not found"),
            "Error should indicate preset not found"
        );
        assert!(
            error.contains("--list-presets"),
            "Error should suggest --list-presets"
        );
    }

    #[test]
    fn test_unicode_presets_load_correctly() {
        let registry = PresetRegistry::new();

        // Test presets with non-Latin characters
        let unicode_presets = vec![
            "chinese", "japanese", "korean", "arabic", "thai", "russian", "greek",
        ];

        for preset_name in unicode_presets {
            let ruleset = registry.load(preset_name).unwrap();

            // Just verify they load - names will contain Unicode characters
            assert!(
                !ruleset.names.male_given_names.is_empty(),
                "{} preset should have male names",
                preset_name
            );
            assert!(
                !ruleset.names.female_given_names.is_empty(),
                "{} preset should have female names",
                preset_name
            );
        }
    }

    #[test]
    fn test_default_trait_implementation() {
        let registry1 = PresetRegistry::new();
        let registry2 = PresetRegistry::default();

        assert_eq!(
            registry1.list().len(),
            registry2.list().len(),
            "Default implementation should create same registry as new()"
        );
    }

    #[test]
    fn test_regional_grouping_totals() {
        let registry = PresetRegistry::new();

        // Count by region (based on the organization in new())
        let european_count = vec![
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
        ]
        .len();

        let middle_eastern_count = vec!["arabic", "armenian", "farsi"].len();
        let asian_count = vec![
            "chinese",
            "japanese",
            "korean",
            "khmer",
            "mongolian",
            "thai",
            "vietnamese",
        ]
        .len();
        let pacific_count = vec![
            "cebuano", "fijian", "malagasy", "malay", "samoan", "tagalog", "tongan",
        ]
        .len();
        let african_count = 1; // swahili
        let caribbean_latin_count = vec!["haitian", "guarani"].len();
        let special_count = 1; // lds

        let total = european_count
            + middle_eastern_count
            + asian_count
            + pacific_count
            + african_count
            + caribbean_latin_count
            + special_count;

        assert_eq!(total, 51, "Regional grouping should total 51 presets");
        assert_eq!(registry.list().len(), 51, "Registry should have 51 presets");

        // Verify counts
        assert_eq!(european_count, 30, "Should have 30 European presets");
        assert_eq!(asian_count, 7, "Should have 7 Asian presets");
        assert_eq!(
            middle_eastern_count, 3,
            "Should have 3 Middle Eastern presets"
        );
        assert_eq!(pacific_count, 7, "Should have 7 Pacific presets");
    }
}
