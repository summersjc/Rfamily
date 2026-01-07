use rfamily_core::gedcom::{GedcomParser, ParseMode};
use rfamily_core::generator::GedcomGenerator;
use rfamily_core::ruleset::Ruleset;
use std::fs;
use std::io::BufWriter;
use std::path::Path;

#[test]
fn test_parse_generated_gedcom() {
    // This test requires test_parse_input.ged to exist in the project root
    let path = Path::new("../test_parse_input.ged");

    if !path.exists() {
        // Skip test if file doesn't exist
        println!("Skipping test - test_parse_input.ged not found");
        return;
    }

    let mut parser = GedcomParser::new(ParseMode::Lenient);
    let result = parser.parse_file(path);

    assert!(result.is_ok(), "Parser failed: {:?}", result.err());

    let gedcom = result.unwrap();

    println!("Parsed GEDCOM:");
    println!("  Individuals: {}", gedcom.individuals.len());
    println!("  Families: {}", gedcom.families.len());
    println!("  Encoding: {}", gedcom.header.encoding);

    // Verify we parsed some individuals
    assert!(!gedcom.individuals.is_empty(), "No individuals parsed");

    // Check that individuals have expected fields
    for (xref, individual) in &gedcom.individuals {
        println!("\nIndividual {}", xref);
        if let Some(ref name) = individual.name {
            println!("  Name: {}", name);
        }
        if let Some(ref sex) = individual.sex {
            println!("  Sex: {}", sex);
        }
        if let Some(ref birth_date) = individual.birth_date {
            println!("  Birth: {}", birth_date);
        }
    }

    // Verify families
    for (xref, family) in &gedcom.families {
        println!("\nFamily {}", xref);
        if let Some(ref husband) = family.husband_xref {
            println!("  Husband: {}", husband);
        }
        if let Some(ref wife) = family.wife_xref {
            println!("  Wife: {}", wife);
        }
        println!("  Children: {}", family.children_xrefs.len());
    }

    println!("\nParser warnings: {}", parser.warnings().len());
}

/// Round-trip test: Generate GEDCOM → Parse → Verify structure
#[test]
fn test_round_trip_generate_and_parse() {
    let output_path = "../test_round_trip.ged";

    // Clean up any existing file
    let _ = fs::remove_file(output_path);

    // Generate GEDCOM with families
    let ruleset = Ruleset::default_english();
    let mut generator = GedcomGenerator::new(ruleset);
    let mut rng = rand::thread_rng();

    generator.generate(50, &mut rng);

    // Write to file
    let file = fs::File::create(output_path).unwrap();
    let mut writer = BufWriter::new(file);
    generator.write_gedcom(&mut writer).unwrap();
    drop(writer);

    // Parse the generated file
    let mut parser = GedcomParser::new(ParseMode::Lenient);
    let result = parser.parse_file(output_path);

    assert!(
        result.is_ok(),
        "Parser should successfully read generated file"
    );

    let gedcom = result.unwrap();

    // Verify basic structure
    assert!(
        !gedcom.individuals.is_empty(),
        "Parsed file should have individuals"
    );
    assert_eq!(
        gedcom.header.encoding, "UTF-8",
        "Header encoding should be UTF-8"
    );

    // Verify all xref formats are correct
    for xref in gedcom.individuals.keys() {
        assert!(
            xref.starts_with("@I"),
            "Individual xref should start with @I"
        );
        assert!(xref.ends_with("@"), "Individual xref should end with @");
    }

    for xref in gedcom.families.keys() {
        assert!(xref.starts_with("@F"), "Family xref should start with @F");
        assert!(xref.ends_with("@"), "Family xref should end with @");
    }

    // Verify family relationships are consistent
    for family in gedcom.families.values() {
        // Check husband exists
        if let Some(ref husband_xref) = family.husband_xref {
            assert!(
                gedcom.individuals.contains_key(husband_xref),
                "Husband xref should reference a valid individual"
            );
        }

        // Check wife exists
        if let Some(ref wife_xref) = family.wife_xref {
            assert!(
                gedcom.individuals.contains_key(wife_xref),
                "Wife xref should reference a valid individual"
            );
        }

        // Check children exist
        for child_xref in &family.children_xrefs {
            assert!(
                gedcom.individuals.contains_key(child_xref),
                "Child xref should reference a valid individual"
            );
        }
    }

    // Clean up
    let _ = fs::remove_file(output_path);

    println!("Round-trip test passed!");
    println!(
        "  Generated and parsed {} individuals",
        gedcom.individuals.len()
    );
    println!("  Generated and parsed {} families", gedcom.families.len());
    println!("  Parser warnings: {}", parser.warnings().len());
}

/// Test parsing GEDCOM with multiple generations
#[test]
fn test_parse_multi_generation_gedcom() {
    let output_path = "../test_multi_gen.ged";

    // Clean up any existing file
    let _ = fs::remove_file(output_path);

    // Generate GEDCOM with multiple generations
    let mut ruleset = Ruleset::default_english();
    ruleset.relationships.generate_families = true;
    ruleset.relationships.generations = 3;

    let mut generator = GedcomGenerator::new(ruleset);
    let mut rng = rand::thread_rng();

    generator.generate(100, &mut rng);

    // Write to file
    let file = fs::File::create(output_path).unwrap();
    let mut writer = BufWriter::new(file);
    generator.write_gedcom(&mut writer).unwrap();
    drop(writer);

    // Parse the generated file
    let mut parser = GedcomParser::new(ParseMode::Lenient);
    let result = parser.parse_file(output_path);

    assert!(
        result.is_ok(),
        "Parser should handle multi-generation GEDCOM"
    );

    let gedcom = result.unwrap();

    // Verify we have multiple families (for multiple generations)
    assert!(
        gedcom.families.len() >= 2,
        "Multi-generation GEDCOM should have multiple families"
    );

    // Verify some individuals have both parent and spouse families
    let mut individuals_with_both = 0;
    for individual in gedcom.individuals.values() {
        if !individual.parent_family_xrefs.is_empty() && !individual.spouse_family_xrefs.is_empty()
        {
            individuals_with_both += 1;
        }
    }

    assert!(
        individuals_with_both > 0,
        "Some individuals should be both children and parents"
    );

    // Clean up
    let _ = fs::remove_file(output_path);

    println!("Multi-generation test passed!");
    println!("  Individuals: {}", gedcom.individuals.len());
    println!("  Families: {}", gedcom.families.len());
    println!("  Individuals with both roles: {}", individuals_with_both);
}
