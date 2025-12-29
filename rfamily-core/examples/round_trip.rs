/// Example: Round-trip GEDCOM generation and parsing
///
/// This example demonstrates generating a GEDCOM file and then parsing it back
/// to verify data integrity. This is useful for testing and validation.
///
/// Usage:
///   cargo run --example round_trip
use rfamily_core::gedcom::{GedcomParser, ParseMode};
use rfamily_core::generator::GedcomGenerator;
use rfamily_core::ruleset::Ruleset;
use std::fs::File;
use std::io::BufWriter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Round-Trip Example: Generate → Parse → Verify ===\n");

    // Step 1: Generate GEDCOM file
    println!("Step 1: Generating GEDCOM file...");

    let mut ruleset = Ruleset::default_english();
    ruleset.relationships.generate_families = true;
    ruleset.relationships.generations = 3;

    let mut generator = GedcomGenerator::new(ruleset);
    let mut rng = rand::thread_rng();

    generator.generate(100, &mut rng);

    let output_path = "round_trip_example.ged";
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);
    generator.write_gedcom(&mut writer)?;
    drop(writer);

    println!("✓ Generated GEDCOM with 100 individuals");

    // Step 2: Parse the generated file
    println!("\nStep 2: Parsing generated GEDCOM file...");

    let mut parser = GedcomParser::new(ParseMode::Lenient);
    let gedcom = parser.parse_file(output_path)?;

    println!("✓ Parsed GEDCOM file");
    println!("  Individuals: {}", gedcom.individuals.len());
    println!("  Families: {}", gedcom.families.len());
    println!("  Warnings: {}", parser.warnings().len());

    // Step 3: Verify data integrity
    println!("\nStep 3: Verifying data integrity...");

    let mut errors = 0;

    // Verify xref formats
    for (xref, _) in &gedcom.individuals {
        if !xref.starts_with("@I") || !xref.ends_with("@") {
            println!("✗ Invalid individual xref: {}", xref);
            errors += 1;
        }
    }

    for (xref, _) in &gedcom.families {
        if !xref.starts_with("@F") || !xref.ends_with("@") {
            println!("✗ Invalid family xref: {}", xref);
            errors += 1;
        }
    }

    // Verify family relationships
    for (fam_xref, family) in &gedcom.families {
        // Check husband reference
        if let Some(ref husband_xref) = family.husband_xref {
            if !gedcom.individuals.contains_key(husband_xref) {
                println!(
                    "✗ Family {} references non-existent husband {}",
                    fam_xref, husband_xref
                );
                errors += 1;
            }
        }

        // Check wife reference
        if let Some(ref wife_xref) = family.wife_xref {
            if !gedcom.individuals.contains_key(wife_xref) {
                println!(
                    "✗ Family {} references non-existent wife {}",
                    fam_xref, wife_xref
                );
                errors += 1;
            }
        }

        // Check children references
        for child_xref in &family.children_xrefs {
            if !gedcom.individuals.contains_key(child_xref) {
                println!(
                    "✗ Family {} references non-existent child {}",
                    fam_xref, child_xref
                );
                errors += 1;
            }
        }
    }

    // Count individuals with both parent and spouse families (multi-generational)
    let mut individuals_with_both_roles = 0;
    for individual in gedcom.individuals.values() {
        if !individual.parent_family_xrefs.is_empty() && !individual.spouse_family_xrefs.is_empty()
        {
            individuals_with_both_roles += 1;
        }
    }

    // Summary
    println!("\n=== Verification Results ===");
    if errors == 0 {
        println!("✓ All references are valid!");
        println!("✓ xref formats are correct!");
    } else {
        println!("✗ Found {} errors", errors);
    }

    println!("\n=== Statistics ===");
    println!("Individuals: {}", gedcom.individuals.len());
    println!("Families: {}", gedcom.families.len());
    println!(
        "Multi-role individuals: {} ({:.1}%)",
        individuals_with_both_roles,
        100.0 * individuals_with_both_roles as f64 / gedcom.individuals.len() as f64
    );

    if gedcom.families.len() > 0 {
        let avg_children: f64 = gedcom
            .families
            .iter()
            .map(|(_, f)| f.children_xrefs.len())
            .sum::<usize>() as f64
            / gedcom.families.len() as f64;
        println!("Average children per family: {:.2}", avg_children);
    }

    println!("\n✓ Round-trip test completed successfully!");

    // Clean up
    let _ = std::fs::remove_file(output_path);

    Ok(())
}
