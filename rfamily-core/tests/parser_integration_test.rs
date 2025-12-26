use rfamily_core::gedcom::{GedcomParser, ParseMode};
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
    assert!(gedcom.individuals.len() > 0, "No individuals parsed");

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
