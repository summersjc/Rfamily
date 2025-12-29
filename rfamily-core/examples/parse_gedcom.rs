/// Example: Parse a GEDCOM file
///
/// This example demonstrates how to use the GEDCOM parser to read and validate
/// existing GEDCOM files, access parsed data, and handle warnings.
///
/// Usage:
///   cargo run --example parse_gedcom -- path/to/file.ged
use rfamily_core::gedcom::{GedcomParser, ParseMode};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get file path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <gedcom-file>", args[0]);
        eprintln!("\nExample:");
        eprintln!("  {} family.ged", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    println!("Parsing GEDCOM file: {}\n", file_path);

    // Create parser in lenient mode (accepts real-world GEDCOM quirks)
    // Use ParseMode::Strict for strict GEDCOM 5.5.1 validation
    let mut parser = GedcomParser::new(ParseMode::Lenient);

    // Parse the file
    let gedcom = match parser.parse_file(file_path) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error parsing GEDCOM file: {}", e);
            std::process::exit(1);
        }
    };

    // Print summary
    println!("=== GEDCOM File Summary ===");
    println!("Encoding: {}", gedcom.header.encoding);
    if let Some(ref source) = gedcom.header.source {
        println!("Source: {}", source);
    }
    if let Some(ref lang) = gedcom.header.language {
        println!("Language: {}", lang);
    }
    println!("\nIndividuals: {}", gedcom.individuals.len());
    println!("Families: {}", gedcom.families.len());
    println!("Warnings: {}", parser.warnings().len());

    // Show warnings if any
    if !parser.warnings().is_empty() {
        println!("\n=== Warnings ===");
        for warning in parser.warnings() {
            println!("  {}", warning);
        }
    }

    // Show first 5 individuals
    println!("\n=== Sample Individuals ===");
    for (xref, individual) in gedcom.individuals.iter().take(5) {
        println!("\n{}", xref);
        if let Some(ref name) = individual.name {
            println!("  Name: {}", name);
        }
        if let Some(ref given) = individual.given_name {
            println!("  Given: {}", given);
        }
        if let Some(ref surname) = individual.surname {
            println!("  Surname: {}", surname);
        }
        if let Some(ref sex) = individual.sex {
            println!("  Sex: {}", sex);
        }
        if let Some(ref birth_date) = individual.birth_date {
            println!("  Birth: {}", birth_date);
            if let Some(ref birth_place) = individual.birth_place {
                println!("    at {}", birth_place);
            }
        }
        if let Some(ref death_date) = individual.death_date {
            println!("  Death: {}", death_date);
            if let Some(ref death_place) = individual.death_place {
                println!("    at {}", death_place);
            }
        }
        if !individual.parent_family_xrefs.is_empty() {
            println!(
                "  Child in {} families",
                individual.parent_family_xrefs.len()
            );
        }
        if !individual.spouse_family_xrefs.is_empty() {
            println!(
                "  Spouse in {} families",
                individual.spouse_family_xrefs.len()
            );
        }
    }

    // Show first 3 families
    println!("\n=== Sample Families ===");
    for (xref, family) in gedcom.families.iter().take(3) {
        println!("\n{}", xref);
        if let Some(ref husband_xref) = family.husband_xref {
            if let Some(husband) = gedcom.individuals.get(husband_xref) {
                println!(
                    "  Husband: {}",
                    husband.name.as_ref().unwrap_or(&"Unknown".to_string())
                );
            }
        }
        if let Some(ref wife_xref) = family.wife_xref {
            if let Some(wife) = gedcom.individuals.get(wife_xref) {
                println!(
                    "  Wife: {}",
                    wife.name.as_ref().unwrap_or(&"Unknown".to_string())
                );
            }
        }
        if let Some(ref marriage_date) = family.marriage_date {
            println!("  Married: {}", marriage_date);
        }
        if let Some(ref divorce_date) = family.divorce_date {
            println!("  Divorced: {}", divorce_date);
        }
        if !family.children_xrefs.is_empty() {
            println!("  Children: {}", family.children_xrefs.len());
        }
    }

    // Verify data integrity
    println!("\n=== Data Integrity Check ===");
    let mut broken_refs = 0;

    // Check family references
    for (xref, family) in &gedcom.families {
        if let Some(ref husband_xref) = family.husband_xref {
            if !gedcom.individuals.contains_key(husband_xref) {
                println!(
                    "  Broken reference: {} husband {} not found",
                    xref, husband_xref
                );
                broken_refs += 1;
            }
        }
        if let Some(ref wife_xref) = family.wife_xref {
            if !gedcom.individuals.contains_key(wife_xref) {
                println!("  Broken reference: {} wife {} not found", xref, wife_xref);
                broken_refs += 1;
            }
        }
        for child_xref in &family.children_xrefs {
            if !gedcom.individuals.contains_key(child_xref) {
                println!(
                    "  Broken reference: {} child {} not found",
                    xref, child_xref
                );
                broken_refs += 1;
            }
        }
    }

    if broken_refs == 0 {
        println!("✓ All references are valid!");
    } else {
        println!("✗ Found {} broken references", broken_refs);
    }

    Ok(())
}
